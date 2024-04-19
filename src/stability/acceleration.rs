//! Расчет критерия ускорения

/// Расчет критерия ускорения
struct Accelleration {
    ///  Ширина судна B
    b: f64,
    /// Осадка судна d
    d: f64,
    /// Коэффициент, учитывающий особенности качки судов смешанного типа
    k_0: Curve,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,    
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,   
    /// Период качки судна  (2.1.5)
    rolling_period: Rc<dyn IRollingPeriod>,   
}
/// 
impl Accelleration {
    /// Основной конструктор
    /// * b - Ширина судна B
    /// * d - Осадка судна d
    /// * k_0 - Коэффициент, учитывающий особенности качки судов смешанного типа
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * rolling_period - Период качки судна  (2.1.5)
    pub fn new(
        b: f64,
        d: f64,
        k_0: Curve,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,    
        metacentric_height: Rc<dyn IMetacentricHeight>,   
        rolling_period: Rc<dyn IRollingPeriod>,  
    ) -> Self {
        Self {
            b,
            d,
            k_0,
            rolling_amplitude,    
            metacentric_height,   
            rolling_period,  
        }
    }
    ///
    pub fn calculate(&mut self) -> f64 {
        let c = self.rolling_period.c();    
        let h_cross_0 = self.metacentric_height.h_cross_0();    
        let k_0 = self.k_0.value(self.b/self.d);
        let theta_1_r = self.rolling_amplitude.calculate();
        let a = 0.0105 * h_cross_0/(c*c*self.b)*k_0*theta_1_r;
        let k = 0.3/a; // >= 1;
        k
    }
}
