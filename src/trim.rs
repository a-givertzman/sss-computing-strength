use crate::{
    mass::Mass,
    math::{curve::Curve, pos_shift::PosShift},
};

///класс с данными для вычисления дифферента судна
pub struct Trim<'a> {
    water_density: f64,       // плотность окружающей воды
    mass: &'a Mass<'a>,       // все грузы судна
    ship_length: f64,         // длинна судна
    center_draught: PosShift, // отстояние центра величины погруженной части судна
    rad_trans: Curve,         // поперечный метацентрические радиус
}

impl<'a> Trim<'a> {
    ///
    pub fn new(
        water_density: f64,       // плотность окружающей воды
        mass: &'a Mass<'a>,       // все грузы судна
        ship_length: f64,         // длинна судна
        center_draught: PosShift, // отстояние центра величины погруженной части судна
        rad_trans: Curve,         // поперечный метацентрические радиус
    ) -> Self {
        Self {
            water_density,
            mass,
            ship_length,
            center_draught,
            rad_trans,
        }
    }
    //объемное водоизмещение
    pub fn volume(&self) -> f64 {
        //объемное водоизмещение
        self.mass.sum() / self.water_density
    }
    //дифферент судна
    #[allow(non_snake_case)]
    pub fn value(&self) -> f64 {
        //отстояние центра величины погруженной части судна по длине от миделя
        let center_draught = self.center_draught.value(self.volume());
        //аппликата продольного метацентра
        let Z_m = center_draught.z() + self.rad_trans.value(self.volume());
        //продольная метацентрическая высота без учета влияния
        //поправки на влияние свободной поверхности
        let H_0 = Z_m - center_draught.z();
        //продольная исправленная метацентрическая высота
        let H = H_0 - self.mass.delta_m_h();
        //момент дифферентующий на 1 см осадки
        let trim_moment = (self.mass.sum() * H) / (100. * self.ship_length);
        //дифферент судна
        self.mass.sum() * (self.mass.shift().x() - center_draught.x()) / (100. * trim_moment)
    }
}
