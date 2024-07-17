//! Нагрузка на судно: постоянный и переменный груз. 
use std::rc::Rc;

use crate::{data::structs::loads::{CargoGeneralCategory, CompartmentData, LoadCargo, LoadConstantData, LoadConstantType, MatterType}, Bound, InertiaMoment, Position};
mod tank;
mod desk;
mod mass;
mod bulk;

pub use tank::*;
pub use desk::*;
pub use mass::*;
pub use bulk::*;

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq,)]
pub enum LoadingType {
    Hull,
    Equipment,
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
    fn create(&mut self) {
        let mut tanks: Vec<Rc<dyn ITank>> = Vec::new();
        let mut desks: Vec<Rc<dyn IDesk>> = Vec::new();
        let mut bulks: Vec<Rc<dyn IBulk>> = Vec::new();
        let mut load_variable: Vec<Rc<LoadMass>> = Vec::new();
        let mut load_timber: Vec<Rc<LoadMass>> = Vec::new();
        let mut loads_const: Vec<Rc<LoadMass>> = Vec::new();

        self.load_constants.iter().for_each(|v| {
            let bound_x = Bound::new(v.bound_x1, v.bound_x2);
            let load = Rc::new(LoadMass::new(
                v.mass,
                bound_x,
                Some(self.shift_const.clone()),
                LoadingType::from(v.loading_type),
            ));
            //   log::info!("\t Mass loads_const from load_constants:{:?} ", load);
            loads_const.push(load);
        });
    
        self.cargoes.iter().for_each(|v| {
            let mass_shift = if v.mass_shift_x.is_some() {
                Some(Position::new(
                    v.mass_shift_x.expect("LoadCargo error: no mass_shift_x!"),
                    v.mass_shift_y.expect("LoadCargo error: no mass_shift_y!"),
                    v.mass_shift_z.expect("LoadCargo error: no mass_shift_z!"),
                ))
            } else {
                None
            };
            let bound_x = Bound::new(v.bound_x1, v.bound_x2);
            let load = Rc::new(LoadMass::new(
                v.mass.expect("LoadCargo error: no mass!"),
                bound_x,
                mass_shift.clone(),
                LoadingType::from(v.general_category),
            ));
            //  log::info!("\t Mass load_variable from cargoes:{:?} ", load);
            load_variable.push(load.clone());
    
            if v.timber {
                load_timber.push(load);
            }
        });
    
        self.compartments.iter().for_each(|v| {
            let mass_shift = if v.mass_shift_x.is_some() {
                Some(Position::new(
                    v.mass_shift_x
                        .expect("CompartmentData error: no mass_shift_x!"),
                    v.mass_shift_y
                        .expect("CompartmentData error: no mass_shift_y!"),
                    v.mass_shift_z
                        .expect("CompartmentData error: no mass_shift_z!"),
                ))
            } else {
                None
            };
            let bound_x = Bound::new(v.bound_x1, v.bound_x2);
            let load = Rc::new(LoadMass::new(
                v.mass.expect("CompartmentData error: no mass!"),
                bound_x,
                mass_shift.clone(),
                LoadingType::from(v.general_category),
            ));
            // log::info!("\t Mass load_variable from compartments src:{:?} trg:{:?}", v, load, );
            load_variable.push(load);
            if v.matter_type == MatterType::Liquid {
                let tank: Rc<dyn ITank> = Rc::new(Tank::new(
                    v.density
                        .expect("CompartmentData error: no density for PhysicalType::Liquid!"),
                    v.volume
                        .expect("CompartmentData error: no volume for PhysicalType::Liquid!"),
                    bound_x,
                    mass_shift.clone(),
                    InertiaMoment::new(
                        v.m_f_s_x.expect(
                            "CompartmentData error: no x in InertiaMoment for PhysicalType::Liquid!",
                        ),
                        v.m_f_s_y.expect(
                            "CompartmentData error: no y in InertiaMoment for PhysicalType::Liquid!",
                        ),
                    ),
                    LoadingType::from(v.general_category),
                ));
                //        log::info!("\t Mass tanks from compartments:{:?} ", tank);
                tanks.push(tank);
            }
            if v.matter_type == MatterType::Bulk {
                let bulk: Rc<dyn IBulk> = Rc::new(Bulk::new(
                    1. / v
                        .density
                        .expect("CompartmentData error: no density for PhysicalType::Bulk!"),
                    v.grain_moment
                        .expect("CompartmentData error: no grain_moment for PhysicalType::Bulk!"),
                ));
                bulks.push(bulk);
            }
        });    
        self.loads_const = Some(Rc::new(loads_const));
        self.desks = Some(Rc::new(desks));
        self.load_variable = Some(Rc::new(load_variable));
        self.load_timber = Some(Rc::new(load_timber));
        self.bulks = Some(Rc::new(bulks));
        self.tanks = Some(Rc::new(tanks));
    }

    pub fn tanks(&mut self) -> Rc<Vec<Rc<dyn ITank>>> {
        if self.tanks.is_none() {
            self.create();
        }
        Rc::clone(self.tanks.as_ref().expect("Loads tanks error: no data!")) 
    }
    pub fn desks(&mut self) -> Rc<Vec<Rc<dyn IDesk>>> {
        if self.desks.is_none() {
            self.create();
        }
        Rc::clone(self.desks.as_ref().expect("Loads desks error: no data!")) 
    }
    pub fn bulks(&mut self) -> Rc<Vec<Rc<dyn IBulk>>> {
        if self.bulks.is_none() {
            self.create();
        }
        Rc::clone(self.bulks.as_ref().expect("Loads bulks error: no data!"))
    }
    pub fn load_variable(&mut self) ->  Rc<Vec<Rc<LoadMass>>> {
        if self.load_variable.is_none() {
            self.create();
        }
        Rc::clone(self.load_variable.as_ref().expect("Loads load_variable error: no data!")) 
    }
    pub fn load_timber(&mut self) ->  Rc<Vec<Rc<LoadMass>>> {
        if self.load_timber.is_none() {
            self.create();
        }
        Rc::clone(self.load_timber.as_ref().expect("Loads load_timber error: no data!")) 
    }
    /// Постоянная масса судна
    pub fn loads_const(&mut self) -> Rc<Vec<Rc<LoadMass>>> {
        if self.loads_const.is_none() {
            self.create();
        }
        Rc::clone(self.loads_const.as_ref().expect("Loads loads_const error: no data!"))
    } 
    /// Смещение центра постоянной массы судна
    pub fn shift_const(&self) -> Position {
        self.shift_const.clone()
    } 
}
