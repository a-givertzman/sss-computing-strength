//! Дифферент судна
use crate::{
    mass::Mass,
    math::{curve::Curve, pos_shift::PosShift, position::Position},
};
/// Дифферента судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    /// длинна судна
    ship_length: f64,       
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Position, 
    /// продольный метацентрические радиус
    rad_long: f64,                 
    /// Поправка к продольной метацентрической высоте на влияние
    /// свободной поверхности жидкости в цистернах
    delta_m_h: f64,   
    /// суммарная масса судна и грузов 
    mass_sum: f64,     
    /// отстояние центра тяжести судна  
    mass_shift: Position, 
}
impl Trim {
    /// Основной конструктор
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
    /// Конструктор, использующий класс Mass нагрузки судна для вычисления 
    /// значений. Аргументы:  
    /// - water_density: плотность окружающей воды 
    /// - ship_length: длинна судна 
    /// - center_draught_shift: кривая отстояния центра величины погруженной части судна  
    /// - rad_long: кривая продольного метацентрического радиуса
    /// - mass: класс инкапсулирующий нагрузку судна
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
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
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
