//! Навалочный смещаемый груз

use std::rc::Rc;

use crate::{Bound, ICurve, ILoad, ILoadMass, IPosShift, Position};

/// Навалочный смещаемый груз.
/// Имеет свойства смещения груза в сторону крена судна.
pub trait IBulk {
    /// Кренящий момент от смещения сыпучего груза
    fn moment(&self) -> f64;
}
/// Навалочный смещаемый груз.
pub struct Bulk {
    /// Удельный погрузочный объем, м³/т.
    s_f: f64,
    /// Объем груза, м³
    volume: f64,
    /// Границы груза
    bound_x: Bound,
    /// Зависимость отстояния центра величины от объема груза
    center: Rc<dyn IPosShift>,    
    /// Зависимость объемного кренящего момента от объема груза
    moment: Rc<dyn ICurve>,
}
///
impl Bulk {
    /// Основной конструктор
    /// * s_f - Удельный погрузочный объем, м³/т.
    /// * volume - Объем груза, м³
    /// * bound_x - Границы груза  по Х
    /// * pos_h - Высота палубы
    /// * center - Зависимость отстояния центра величины от объема груза    
    /// * moment - Зависимость объемного кренящего момента от объема груза
    pub fn new(
        s_f: f64,
        volume: f64,
        bound_x: Bound,
        center: Rc<dyn IPosShift>,        
        moment: Rc<dyn ICurve>,
    ) -> Self {
        Self {
            s_f,
            volume,
            bound_x,
            center,            
            moment,
        }
    }
}
///
impl IBulk for Bulk {
    /// Кренящий момент от смещения сыпучего груза
    fn moment(&self) -> f64 {
        self.moment.value(self.volume) / self.s_f
    }
}
///
impl ILoad for Bulk {
    ///
    fn mass(&self) -> f64 {
        self.volume / self.s_f
    }
    ///
    fn bound_x(&self) -> Bound {
        self.bound_x
    }
    ///
    fn shift(&self) -> Position {
        self.center.value(self.volume)
    }
}
///
impl ILoadMass for Bulk {}
