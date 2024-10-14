//! Нагрузка на судно: постоянный и переменный груз.
use std::rc::Rc;

use crate::{
    data::structs::loads::{
        CargoGeneralCategory, CompartmentData, LoadCargo, LoadConstantData, LoadConstantType,
        MatterType,
    },
    Bound, Error, InertiaMoment, Position,
};
mod bulk;
mod desk;
mod mass;
mod tank;

pub use bulk::*;
pub use desk::*;
pub use mass::*;
pub use tank::*;

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LoadingType {
    Hull,
    Equipment,
    Bulkhead,
    Ballast,
    Stores,
    Cargo,
}
///
impl std::fmt::Display for LoadingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LoadingType::Hull => "Hull",
                LoadingType::Equipment => "Equipment",
                LoadingType::Bulkhead => "Bulkhead",
                LoadingType::Ballast => "Ballast",
                LoadingType::Stores => "Stores",
                LoadingType::Cargo => "Cargo",
            },
        )
    }
}
///
impl From<CargoGeneralCategory> for LoadingType {
    fn from(value: CargoGeneralCategory) -> Self {
        match value {
            CargoGeneralCategory::Lightship => LoadingType::Hull,
            CargoGeneralCategory::Ballast => LoadingType::Ballast,
            CargoGeneralCategory::Bulkhead => LoadingType::Bulkhead,
            CargoGeneralCategory::Stores => LoadingType::Stores,
            CargoGeneralCategory::Cargo => LoadingType::Cargo,
        }
    }
}
///
impl From<LoadConstantType> for LoadingType {
    fn from(value: LoadConstantType) -> Self {
        match value {
            LoadConstantType::Equipment => LoadingType::Equipment,
            LoadConstantType::Hull => LoadingType::Hull,
        }
    }
}
/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// Суммарная масса груза
    fn mass(&self) -> f64;
    /// Границы груза вдоль продольной оси
    fn bound_x(&self) -> Bound;
    /// Смещение центра груза относительно начала координат судна
    fn shift(&self) -> Position;
}
/// Нагрузка судна: грузы, корпус, механизмы
pub struct Loads<'a> {
    load_constants: &'a Vec<LoadConstantData>,
    shift_const: Position,
    cargoes: &'a Vec<LoadCargo>,
    compartments: &'a Vec<CompartmentData>,
    tanks: Option<Rc<Vec<Rc<dyn ITank>>>>,
    desks: Option<Rc<Vec<Rc<dyn IDesk>>>>,
    bulks: Option<Rc<Vec<Rc<dyn IBulk>>>>,
    load_variable: Option<Rc<Vec<Rc<LoadMass>>>>,
    load_timber: Option<Rc<Vec<Rc<LoadMass>>>>,
    // Постоянная масса судна
    loads_const: Option<Rc<Vec<Rc<LoadMass>>>>,
}
///
impl<'a> Loads<'_> {
    /// Основной конструктор
    /// * load_constants - Постоянная нагрузка на судно
    /// * shift_const - Смещение центра масс постоянной нагрузки на судно
    /// * cargoes - Нагрузка судна без жидких грузов
    /// * compartments - Нагрузка судна: цистерны и трюмы
    pub fn new(
        load_constants: &'a Vec<LoadConstantData>,
        shift_const: Position,
        cargoes: &'a Vec<LoadCargo>,
        compartments: &'a Vec<CompartmentData>,
    ) -> Loads<'a> {
        Loads {
            load_constants,
            shift_const,
            cargoes,
            compartments,
            tanks: None,
            desks: None,
            bulks: None,
            load_variable: None,
            load_timber: None,
            loads_const: None,
        }
    }
    ///
    fn create(&mut self) -> Result<(), Error> {
        let mut tanks: Vec<Rc<dyn ITank>> = Vec::new();
        let mut desks: Vec<Rc<dyn IDesk>> = Vec::new();
        let mut bulks: Vec<Rc<dyn IBulk>> = Vec::new();
        let mut load_variable: Vec<Rc<LoadMass>> = Vec::new();
        let mut load_timber: Vec<Rc<LoadMass>> = Vec::new();
        let mut loads_const: Vec<Rc<LoadMass>> = Vec::new();

        for v in self.load_constants.iter() {
            let bound_x = Bound::new(v.bound_x1, v.bound_x2)?;
            let load = Rc::new(LoadMass::new(
                v.mass,
                bound_x,
                Some(self.shift_const.clone()),
                LoadingType::from(v.loading_type),
            )?);
            log::trace!("\t Mass loads_const from load_constants:{:?} ", load);
            loads_const.push(load);
        }

        for v in self.cargoes.iter() {
            let mass_shift = if let (Some(mass_shift_x), Some(mass_shift_y), Some(mass_shift_z)) =
                (v.mass_shift_x, v.mass_shift_y, v.mass_shift_z)
            {
                Some(Position::new(mass_shift_x, mass_shift_y, mass_shift_z))
            } else {
                None
            };
            let bound_x = Bound::new(v.bound_x1, v.bound_x2)?;
            let load = Rc::new(LoadMass::new(
                v.mass.ok_or("LoadCargo error: no mass!".to_string())?,
                bound_x,
                mass_shift.clone(),
                LoadingType::from(v.general_category),
            )?);
            log::trace!("\t Mass load_variable from cargoes:{:?} ", load);
            load_variable.push(load.clone());

            if let (Some(vertical_area), Some(horizontal_area)) =
                (v.vertical_area, v.horizontal_area)
            {
                let bound_y = if let (Some(bound_y1), Some(bound_y2)) = (v.bound_y1, v.bound_y2) {
                    Bound::new(bound_y1, bound_y2)?
                } else {
                    Bound::Full
                };
                let bound_z = if let (Some(bound_z1), Some(bound_z2)) = (v.bound_z1, v.bound_z2) {
                    Bound::new(bound_z1, bound_z2)?
                } else {
                    Bound::Full
                };
                let mass_shift = if let Some(mass_shift) = mass_shift {
                    mass_shift
                } else {
                    if let (Bound::Value(_, _), Bound::Value(_, _)) = (bound_y, bound_z) {
                        Some(Position::new(
                            bound_x.center().unwrap(),
                            bound_y.center().unwrap(),
                            bound_z.center().unwrap(),
                        ))
                    } else {
                        None
                    }
                    .ok_or(Error::FromString("Load create Desk error: no center of mass!".to_string()))?
                };
                let vertical_shift = if let (
                    Some(vertical_area_shift_x),
                    Some(vertical_area_shift_y),
                    Some(vertical_area_shift_z),
                ) = (
                    v.vertical_area_shift_x,
                    v.vertical_area_shift_y,
                    v.vertical_area_shift_z,
                ) {
                    Position::new(
                        vertical_area_shift_x,
                        vertical_area_shift_y,
                        vertical_area_shift_z,
                    )
                } else {
                    mass_shift.clone()
                };
                let desk: Rc<dyn IDesk> = Rc::new(Desk::new(
                    v.mass.ok_or("LoadCargo error: no mass!".to_string())?,
                    mass_shift,
                    bound_x,
                    bound_y,
                    vertical_area,
                    vertical_shift,
                    horizontal_area,
                    v.timber,
                ));
                desks.push(desk);
            }

            if v.timber {
                load_timber.push(load);
            }
        }

        for v in self.compartments.iter() {
            let mass_shift = if v.mass_shift_x.is_some() {
                Some(Position::new(
                    v.mass_shift_x
                        .ok_or("CompartmentData error: no mass_shift_x!".to_string())?,
                    v.mass_shift_y
                        .ok_or("CompartmentData error: no mass_shift_y!".to_string())?,
                    v.mass_shift_z
                        .ok_or("CompartmentData error: no mass_shift_z!".to_string())?,
                ))
            } else {
                None
            };
            let bound_x = Bound::new(v.bound_x1, v.bound_x2)?;
            let load = Rc::new(LoadMass::new(
                v.mass.ok_or("CompartmentData error: no mass!".to_string())?,
                bound_x,
                mass_shift.clone(),
                LoadingType::from(v.general_category),
            )?);
            log::trace!("\t Mass load_variable from compartments src:{:?} trg:{:?}", v, load, );
            load_variable.push(load);
            if v.matter_type == MatterType::Liquid {
                let tank = Tank::new(
                    v.density
                        .ok_or("CompartmentData error: no density for PhysicalType::Liquid!".to_string())?,
                    v.volume
                        .ok_or("CompartmentData error: no volume for PhysicalType::Liquid!".to_string())?,
                    bound_x,
                    mass_shift.clone(),
                    InertiaMoment::new(
                        v.m_f_s_x.ok_or("CompartmentData error: no x in InertiaMoment for PhysicalType::Liquid!".to_string())?,
                        v.m_f_s_y.ok_or("CompartmentData error: no y in InertiaMoment for PhysicalType::Liquid!".to_string())?,
                    ),
                    LoadingType::from(v.general_category),
                )?;
                log::trace!("\t Mass tanks from compartments:{:?} ", tank);
                let tank: Rc<dyn ITank> = Rc::new(tank);
                tanks.push(tank);
            }
            if v.matter_type == MatterType::Bulk {
                let bulk: Rc<dyn IBulk> = Rc::new(Bulk::new(
                    1. / v.density.ok_or("CompartmentData error: no density for PhysicalType::Bulk!".to_string())?,
                    v.grain_moment.ok_or("CompartmentData error: no grain_moment for PhysicalType::Bulk!".to_string())?,
                )?);
                bulks.push(bulk);
            }
        }
        self.loads_const = Some(Rc::new(loads_const));
        self.desks = Some(Rc::new(desks));
        self.load_variable = Some(Rc::new(load_variable));
        self.load_timber = Some(Rc::new(load_timber));
        self.bulks = Some(Rc::new(bulks));
        self.tanks = Some(Rc::new(tanks));
        Ok(())
    }

    pub fn tanks(&mut self) -> Result<Rc<Vec<Rc<dyn ITank>>>, Error> {
        if self.tanks.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.tanks
                .as_ref()
                .ok_or("Loads tanks error: no data!".to_string())?,
        ))
    }
    pub fn desks(&mut self) -> Result<Rc<Vec<Rc<dyn IDesk>>>, Error> {
        if self.desks.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.desks
                .as_ref()
                .ok_or("Loads desks error: no data!".to_string())?,
        ))
    }
    pub fn bulks(&mut self) -> Result<Rc<Vec<Rc<dyn IBulk>>>, Error> {
        if self.bulks.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.bulks
                .as_ref()
                .ok_or("Loads bulks error: no data!".to_string())?,
        ))
    }
    pub fn load_variable(&mut self) -> Result<Rc<Vec<Rc<LoadMass>>>, Error> {
        if self.load_variable.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.load_variable
                .as_ref()
                .ok_or("Loads load_variable error: no data!".to_string())?,
        ))
    }
    pub fn load_timber(&mut self) -> Result<Rc<Vec<Rc<LoadMass>>>, Error> {
        if self.load_timber.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.load_timber
                .as_ref()
                .ok_or("Loads load_timber error: no data!".to_string())?,
        ))
    }
    /// Постоянная масса судна
    pub fn loads_const(&mut self) -> Result<Rc<Vec<Rc<LoadMass>>>, Error> {
        if self.loads_const.is_none() {
            self.create()?;
        }
        Ok(Rc::clone(
            self.loads_const
                .as_ref()
                .ok_or("Loads loads_const error: no data!".to_string())?,
        ))
    }
    /// Смещение центра постоянной массы судна
    pub fn shift_const(&self) -> Position {
        self.shift_const.clone()
    }
}
