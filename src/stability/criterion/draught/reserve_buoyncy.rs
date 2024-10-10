//! Запас плавучести в носу
use std::rc::Rc;

use crate::{draught::IDraught, Curve, Error, ICurve};

/// Суммарная площадь проекции на диаметральную плоскость, в пределах  
/// 0,15 LBP в корму от носового перпендикуляра, части корпуса судна  
/// между ватерлинией и линией палубы у борта и закрытой надстройки, м^2,
///  если она имеется, должна быть не менее расчетной минимальной площади A_min.
pub struct ReserveBuoyncyInBow {
    /// Осадка судна
    draught: Rc<dyn IDraught>,
    /// Длинна судна между перпендикулярами
    length_lbp: f64,
    /// Cуммарая площадь проекции на диаметральную плоскость от осадки, м^2
    bow_area: Vec<(f64, f64)>,
}
//
impl ReserveBuoyncyInBow {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * length_lbp - Длинна судна между перпендикулярами
    /// * bow_area - Cуммарая площадь проекции на диаметральную плоскость от осадки, м^2
    pub fn new(
        draught: Rc<dyn IDraught>,
        length_lbp: f64,
        bow_area: Vec<(f64, f64)>,
    ) -> Self {
        Self {
            draught,
            length_lbp,
            bow_area,
        }
    }
    //
    pub fn calculate(&self) -> Result<f64, Error> {
        let draught_0075l = self.draught.value((0.5 - 0.075)*self.length_lbp)?;
        Curve::new_linear(&self.bow_area)?.value(draught_0075l)
    }
}
