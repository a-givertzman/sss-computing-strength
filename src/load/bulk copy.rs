//! Навалочный смещаемый груз
use crate::math::*;

/// Навалочный смещаемый груз.
/// Имеет свойства смещения груза в сторону крена судна.
#[derive(Clone)]
pub struct Bulk {
    /// Удельный погрузочный объем, м³/т.
    s_f: f64,
    /// Объем груза, м³
    volume: f64,
    /// Объемный кренящий момент, м4
    m_v: Rc<dyn ICurve>,
}
///
impl Bulk {
    /// Основной конструктор
    /// * density - Плотность жидкости в цистерне
    /// * volume - Объем жидкости в цистерне
    /// * center - Кривая координат центра объема жидкости в цистерне в системе координат судна
    /// * free_surf_inertia - Кривая момента инерции площади свободной поверхности жидкости
    /// * m_f_s_y - Продольный момент свободной поверхности жидкости
    /// * m_f_s_x - Поперечный момент свободной поверхности жидкости
    pub fn new(
 //       density: f64,
  //      volume: f64,
  //      center: PosShift,
   //     free_surf_inertia: InertiaShift,
        m_f_s_y: Option<f64>,
        m_f_s_x: Option<f64>,
    ) -> Self {
   //     assert!(density > 0., "density {} > 0", density);
    //    assert!(volume >= 0., "volume {} >= 0", volume);
        Self {
    //        density,
    //        volume,
    //        center,
    //        free_surf_inertia,
            m_f_s_y,
            m_f_s_x,
        }
    }
    /// Момент момент от смещения сыпучего груза 
    fn moment_bulk(&self) -> f64 {
   //     if self.m_f_s_x.is_some() && self.m_f_s_y.is_some() {
            return SurfaceMoment::new(self.m_f_s_x.unwrap(), self.m_f_s_y.unwrap());
   /*     }
        let result =
            SurfaceMoment::from_inertia(self.free_surf_inertia.value(self.volume), self.density);
        log::info!("\t Bulk result:{:?}", result);    
        result*/
    }
}
