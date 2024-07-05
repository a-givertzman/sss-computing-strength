//! Осадка судна
use std::rc::Rc;

use crate::{trim::ITrim, IParameters, ParameterID};


/// Осадка судна
pub struct Draught {
    /// длинна судна
    ship_length: f64,
    /// Средняя осадка
    mean_draught: f64,   
    ///  отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: f64, 
    /// Дифферент судна
    trim: Box<dyn ITrim>,     
    /// Набор результатов расчетов для записи в БД
    parameters: Option<Rc<dyn IParameters>>, 
    /// Осадка на миделе в ДП, м
    draught_mid: Option<f64>,
    /// Изменение осадки
    delta_draught: Option<f64>,
}
///
impl Draught {
    /// Основной конструктор
    /// * ship_length - длинна судна
    /// * mean_draught - Средняя осадка
    /// * center_waterline_shift - отстояние центра тяжести ватерлинии по длине от миделя
    /// * trim - Дифферент судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        ship_length: f64,           
        mean_draught: f64,        
        center_waterline_shift: f64,    
        trim: Box<dyn ITrim>,               
        parameters: Option<Rc<dyn IParameters>>, 
    ) -> Self {
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            ship_length,
            mean_draught,
            center_waterline_shift,
            trim,
            parameters,
            draught_mid: None,
            delta_draught: None,
        }
    }
    /// Вычисление осадки на миделе и изменения осадки
    #[allow(non_snake_case)]
    fn calculate(&mut self) {
        // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
        let draught_bow = self.mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*self.trim.value();
        // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
        let draught_stern = self.mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*self.trim.value();
        // Осадка на миделе в ДП, м (8)
        let draught_mid = (draught_bow + draught_stern) / 2.;
        // Изменение осадки
        let delta_draught = (draught_bow - draught_stern) / self.ship_length;
        if let Some(parameters) = &self.parameters {
            parameters.add(ParameterID::DraughtBow, draught_bow);
            parameters.add(ParameterID::DraughtStern, draught_stern);
        }
        self.draught_mid = Some(draught_mid);
        self.delta_draught = Some(delta_draught);
    }
}
///
impl IDraught for Draught {
    /// Значение осадки в точке
    #[allow(non_snake_case)]
    fn value(&mut self, pos_x: f64) -> f64 {
        if self.draught_mid.is_none() {
            self.calculate();
        }        
        self.draught_mid.expect("Draught value error: no draught_mid!")
            + self.delta_draught.expect("Draught value error: no draught_mid!") 
            * pos_x
    }
}
///
#[doc(hidden)]
pub trait IDraught {
    /// Вычисление дифферента
    fn value(&mut self, pos_x: f64) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeDraught {
    draught_mid: f64,
    delta_draught: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeDraught {
    pub fn new(draught_mid: f64, delta_draught: f64,) -> Self {
        Self { draught_mid,delta_draught }
    }
}
#[doc(hidden)]
impl IDraught for FakeDraught {
    fn value(&mut self, pos_x: f64) -> f64 {
        self.draught_mid
        + self.delta_draught 
        * pos_x
    }
}


