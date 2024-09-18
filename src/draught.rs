//! Осадка судна
use std::rc::Rc;

use crate::{trim::ITrim, Error, IParameters, ParameterID};


/// Осадка судна
pub struct Draught {
    /// длинна судна
    ship_length: f64,
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
    /// * center_waterline_shift - отстояние центра тяжести ватерлинии по длине от миделя
    /// * trim - Дифферент судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        ship_length: f64,               
        center_waterline_shift: f64,    
        trim: Box<dyn ITrim>,               
        parameters: Option<Rc<dyn IParameters>>, 
    ) -> Result<Self, Error> {
        if ship_length <= 0. {
            return Err(Error::FromString(format!("Draught new error: ship_length {ship_length} <= 0.")));
        }
        Ok(Self {
            ship_length,
            center_waterline_shift,
            trim,
            parameters,
            draught_mid: None,
            delta_draught: None,
        })
    }
    /// Вычисление осадки на миделе и изменения осадки
    #[allow(non_snake_case)]
    fn calculate(&mut self) -> Result<(), Error> {
        let (mean_draught, trim) = self.trim.value()?;
        // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
        let draught_bow = mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*trim;
        // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
        let draught_stern = mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*trim;
        // Осадка на миделе в ДП, м (8)
        let draught_mid = (draught_bow + draught_stern) / 2.;
        // Изменение осадки
        let delta_draught = (draught_bow - draught_stern) / self.ship_length;
     //   log::info!("\t Draught mean_draught:{:?} center_waterline_shift:{:?} ship_length:{:?}, trim:{}, 
      //      draught_bow:{draught_bow} draught_stern:{draught_stern} draught_mid:{draught_mid}", 
     //       mean_draught, self.center_waterline_shift, self.ship_length, trim);
        dbg!(mean_draught, trim, draught_bow, draught_stern, draught_mid);
        if let Some(parameters) = &self.parameters {
            parameters.add(ParameterID::DraughtBow, draught_bow);
            parameters.add(ParameterID::DraughtStern, draught_stern);
        }
        self.draught_mid = Some(draught_mid);
        self.delta_draught = Some(delta_draught);
        Ok(())
    }
}
///
impl IDraught for Draught {
    /// Значение осадки в точке
    #[allow(non_snake_case)]
    fn value(&mut self, pos_x: f64) -> Result<f64, Error> {
        if self.draught_mid.is_none() {
            self.calculate()?;
        }        
        Ok(self.draught_mid.ok_or("Draught value error: no draught_mid!".to_string())?
            + self.delta_draught.ok_or("Draught value error: no draught_mid!".to_string())?
            * pos_x)
    }
}
///
#[doc(hidden)]
pub trait IDraught {
    /// Вычисление дифферента
    fn value(&mut self, pos_x: f64) -> Result<f64, Error>;
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
        Self { draught_mid, delta_draught }
    }
}
#[doc(hidden)]
impl IDraught for FakeDraught {
    fn value(&mut self, pos_x: f64) -> Result<f64, Error> {
        Ok(self.draught_mid
        + self.delta_draught 
        * pos_x)
    }
}


