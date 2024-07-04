//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use crate::stability::metacentric_height::IMetacentricHeight;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::{math::*, IParameters, ParameterID};

use crate::mass::IMass;

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
    /// все грузы судна
    mass: Rc<dyn IMass>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
    /// Осадка на миделе в ДП, м
    draught_mid: Option<f64>,
    /// Изменение осадки
    delta_draught: Option<f64>,
}
///
impl Trim {
    /// Основной конструктор
    /// * ship_length - длинна судна
    /// * mean_draught - Средняя осадка
    /// * center_waterline_shift - отстояние центра тяжести ватерлинии по длине от миделя
    /// * center_draught_shift - отстояние центра величины погруженной части судна   
    /// * metacentric_height - Исправленная метацентрическая высота
    /// * mass - все грузы судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        ship_length: f64,           
        mean_draught: f64,        
        center_waterline_shift: f64,     
        center_draught_shift: Position,        
        metacentric_height: Rc<dyn IMetacentricHeight>, 
        mass: Rc<dyn IMass>,                 
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
            parameters,
            draught_mid: None,
            delta_draught: None,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&mut self, pos_x: f64) -> f64 {
        if self.draught_mid.is_none() {
            self.calculate();
        }        
        self.draught_mid.expect("stability::Trim value error: no draught_mid!")
            + self.delta_draught.expect("stability::Trim value error: no draught_mid!") 
            * pos_x
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    fn calculate(&mut self) {
        // Продольная исправленная метацентрическая высота (3)
        let H = self.metacentric_height.h_long_fix();
        // Момент дифферентующий на 1 см осадки (4)
        let trim_moment = (self.mass.sum() * H) / (100. * self.ship_length);
        // Дифферент судна (5)
        let t = self.mass.sum() * (self.mass.shift().x() - self.center_draught_shift.x())
            / (100. * trim_moment);
        // Дифферент судна, градусы (5)
        let trim_angle = (t/self.ship_length).atan()*180.0/PI;    
        // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
        let draught_bow = self.mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*t;
        // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
        let draught_stern = self.mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*t;
        // Осадка на миделе в ДП, м (8)
        let draught_mid = (draught_bow + draught_stern) / 2.;
        // Изменение осадки
        let delta_draught = (draught_stern - draught_bow) / self.ship_length;
        log::info!(
            "\t Trim H:{H} mass:{} mass_shift_x:{} center_draught_x:{} M:{trim_moment} trim:{t} 
                trim_angle{trim_angle} draught_bow:{draught_bow} draught_stern:{draught_stern} 
                draught_mid:{draught_mid} delta_draught:{delta_draught}",
            self.mass.sum(),
            self.mass.shift().x(),
            self.center_draught_shift.x()
        );
        self.parameters.add(ParameterID::MomentTrimPerCm, trim_moment);
        self.parameters.add(ParameterID::Trim, trim_angle);
        self.parameters.add(ParameterID::DraughtBow, draught_bow);
        self.parameters.add(ParameterID::DraughtStern, draught_stern);
        self.draught_mid = Some(draught_mid);
        self.delta_draught = Some(delta_draught);
    }
}
