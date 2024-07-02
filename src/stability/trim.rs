//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use crate::stability::metacentric_height::IMetacentricHeight;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::{math::*, IParameters, IShipMoment, ParameterID};

use crate::IMass;

/// Дифферент судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    /// длинна судна
    ship_length: f64,
    /// Средняя осадка
    mean_draught: f64,   
    ///  отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: f64, 
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Position,
    /// Исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Масса судна
    mass: Rc<dyn IMass>, 
    /// Момент массы судна
    moment: Rc<dyn IShipMoment>, 
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
}
///
impl Trim {
    /// Основной конструктор
    /// * ship_length - длинна судна
    /// * mean_draught - Средняя осадка
    /// * center_waterline_shift - отстояние центра тяжести ватерлинии по длине от миделя
    /// * center_draught_shift - отстояние центра величины погруженной части судна   
    /// * metacentric_height - Исправленная метацентрическая высота
    /// * mass - Масса судна
    /// * moment - Момент массы судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        ship_length: f64,           
        mean_draught: f64,        
        center_waterline_shift: f64,     
        center_draught_shift: Position,        
        metacentric_height: Rc<dyn IMetacentricHeight>, 
        mass: Rc<dyn IMass>, 
        moment: Rc<dyn IShipMoment>,                
        parameters: Rc<dyn IParameters>, 
    ) -> Self {
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            ship_length,
            mean_draught,
            center_waterline_shift,
            center_draught_shift,
            metacentric_height,
            mass,
            moment,
            parameters,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&mut self) -> f64 {
        // Продольная исправленная метацентрическая высота (3)
        let H = self.metacentric_height.h_long_fix();
        // Момент дифферентующий на 1 см осадки (4)
        let trim_moment = (self.mass.sum() * H) / (100. * self.ship_length);
        // Дифферент судна (5)
        let t = self.mass.sum() * (self.moment.shift().x() - self.center_draught_shift.x())
            / (100. * trim_moment);
        // Дифферент судна, градусы (5)
        let trim_angle = (t/self.ship_length).atan()*180.0/PI;    
        // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
        let draught_bow = self.mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*t;
        // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
        let draught_stern = self.mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*t;
        // Осадка на миделе в ДП, м (8)
        let draught_mid = (draught_bow + draught_stern) / 2.;

        log::info!(
            "\t Trim H:{H} mass:{} mass_shift_x:{} center_draught_x:{} M:{trim_moment} trim:{t} 
                trim_angle{trim_angle} draught_bow:{draught_bow} draught_stern:{draught_stern} draught_mid:{draught_mid}",
            self.mass.sum(),
            self.moment.shift().x(),
            self.center_draught_shift.x()
        );
        self.parameters.add(ParameterID::MomentTrimPerCm, trim_moment);
        self.parameters.add(ParameterID::Trim, trim_angle);
        self.parameters.add(ParameterID::DraughtBow, draught_bow);
        self.parameters.add(ParameterID::DraughtStern, draught_stern);
        t
    }
}
