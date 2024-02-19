use crate::{displacement::Displacement, mass::Mass, math::{bound::Bound, curve::Curve, pos_shift::PosShift}};


///распределение осадки, м
pub struct Draught<'a> {
    water_density: f64,     // плотность окружающей воды 
    mass: &'a Mass<'a>,     // все грузы судна
    displacement: Displacement,
    ship_length: f64,       // длинна судна
    bounds: &'a Vec<Bound>, // ссылка на вектор разбиения на отрезки для эпюров
    center_draught: PosShift,// отстояние центра величины погруженной части судна
    center_waterline: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
    rad_long: Curve,        // продольный метацентрические радиус                
    rad_trans: Curve,       // поперечный метацентрические радиус
    mean_draught: Curve,    // средняя осадка
}

impl<'a> Draught<'a> {
    ///
    pub fn new(
        water_density: f64,     // плотность окружающей воды 
        mass: &'a Mass<'a>,     // все грузы судна
        ship_length: f64,       // длинна судна
        bounds: &'a Vec<Bound>, // ссылка на вектор разбиения на отрезки для эпюров
        center_draught: PosShift,// отстояние центра величины погруженной части судна
        center_waterline: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
        rad_long: Curve,        // продольный метацентрические радиус                
        rad_trans: Curve,       // поперечный метацентрические радиус
        mean_draught: Curve,    // средняя осадка
        ) -> Self {
            Self {
                water_density,
                mass,
                ship_length,
                bounds,
                center_draught,
                center_waterline,
                rad_long,               
                rad_trans,
                mean_draught,
            }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
        let stern_draught = self.stern_draught();
        let bow_draught = self.bow_draught();
        let delta_draught = (bow_draught - stern_draught)/self.bounds.len();

        self.bounds.iter().map(|v| self.displacement.value(*v, delta_draught*(v.center() + self.ship_length/2.)/self.ship_length)).collect()
    }
}
