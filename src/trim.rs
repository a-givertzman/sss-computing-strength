use crate::{
    mass::Mass,
    math::{curve::Curve, pos_shift::PosShift, position::Position},
};

///класс с данными для вычисления дифферента судна
pub struct Trim {
    ship_length: f64,               // длинна судна
    center_draught_shift: Position, // отстояние центра величины погруженной части судна
    rad_long: f64,                 // продольный метацентрические радиус
    delta_m_h: f64,                 //Поправка к продольной метацентрической высоте на влияние
                                    //свободной поверхности жидкости в цистернах
    mass_sum: f64,        // суммарная масса судна и грузов
    mass_shift: Position, // отстояние центра тяжести судна
}
impl Trim {
    ///
    pub fn new(
        ship_length: f64,               // длинна судна
        center_draught_shift: Position, // отстояние центра величины погруженной части судна
        rad_long: f64,                 // продольный метацентрические радиус
        delta_m_h: f64,                 //Поправка к продольной метацентрической высоте на влияние
                                        //свободной поверхности жидкости в цистернах
        mass_sum: f64,        // суммарная масса судна и грузов
        mass_shift: Position, // отстояние центра тяжести судна
    ) -> Self {
        Self {
            ship_length,
            center_draught_shift,
            rad_long,
            delta_m_h,
            mass_sum,
            mass_shift,
        }
    }
    ///
    pub fn from_mass(
        water_density: f64,             // плотность окружающей воды
        ship_length: f64,               // длинна судна
        center_draught_shift: PosShift, // отстояние центра величины погруженной части судна
        rad_long: Curve,               // продольный метацентрические радиус
        mass: Mass,                     // все грузы судна
    ) -> Self {
        let mass_sum = mass.sum(); // суммарная масса судна и грузов
        let volume = mass_sum / water_density; //объемное водоизмещение
        Self::new(
            ship_length,
            center_draught_shift.value(volume),
            rad_long.value(volume),
            mass.delta_m_h(),
            mass_sum,
            mass.shift(),
        )
    }
    //дифферент судна
    #[allow(non_snake_case)]
    pub fn value(&self) -> f64 {
        //аппликата продольного метацентра
        let Z_m = self.center_draught_shift.z() + self.rad_long;
        //продольная метацентрическая высота без учета влияния
        //поправки на влияние свободной поверхности
        let H_0 = Z_m - self.center_draught_shift.z();
        //продольная исправленная метацентрическая высота
        let H = H_0 - self.delta_m_h;
        //момент дифферентующий на 1 см осадки
        let trim_moment = (self.mass_sum * H) / (100. * self.ship_length);
        //дифферент судна
        self.mass_sum * (self.mass_shift.x() - self.center_draught_shift.x()) / (100. * trim_moment)
    }
}
