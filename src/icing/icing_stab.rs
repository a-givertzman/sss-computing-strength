//! Тип обледенения судна

/// Тип обледенения, возпращает массу льда на поверхности в  
/// в зависимости от типа обледенения
pub struct IcingStab {
    /// Тип обледенения
    icing_stab: String,
    /// Масса льда на квадратный метр площади горизонтальной поверхности
    /// палубного лесного груза
    icing_m_timber: f64,
    /// Масса льда на квадратный метр площади парусности
    /// при учете полного обледенения
    icing_m_v_full: f64,
    /// Масса льда на квадратный метр площади парусности  
    /// при учете частичного обледенения
    icing_m_v_half: f64,
    /// Масса льда на квадратный метр площади горизонтальной
    /// поверхности при учете полного обледенения
    icing_m_h_full: f64,
    /// Масса льда на квадратный метр площади горизонтальной  
    /// поверхности при учете частичного обледенения
    icing_m_h_half: f64,
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности при учете полного обледенения
    icing_coef_v_area_full: f64,
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности при учете частичного обледенения
    icing_coef_v_area_half: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете полного обледенения
    icing_coef_v_moment_full: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете частичного обледенения
    icing_coef_v_moment_half: f64,
}
///
impl IcingStab {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения: "full", "half" или отсутствует
    /// * icing_m_timber - Масса льда на квадратный метр площади горизонтальной поверхности
    /// палубного лесного груза
    /// * icing_m_v_full - Масса льда на квадратный метр площади парусности
    /// при учете полного обледенения
    /// * icing_m_v_half - Масса льда на квадратный метр площади парусности  
    /// при учете частичного обледенения
    /// * icing_m_h_full - Масса льда на квадратный метр площади горизонтальной
    /// поверхности при учете полного обледенения
    /// * icing_m_h_half  - Масса льда на квадратный метр площади горизонтальной  
    /// поверхности при учете частичного обледенения
    /// * icing_coef_v_area_full - Коэффициент увеличения площади парусности несплощной
    /// поверхности при учете полного обледенения
    /// * icing_coef_v_area_half - Коэффициент увеличения площади парусности несплощной
    /// поверхности при учете частичного обледенения
    /// * icing_coef_v_moment_full - Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете полного обледенения
    /// * icing_coef_v_moment_half - Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете частичного обледенения
    pub fn new(
        icing_stab: String,
        icing_m_timber: f64,
        icing_m_v_full: f64,
        icing_m_v_half: f64,
        icing_m_h_full: f64,
        icing_m_h_half: f64,
        icing_coef_v_area_full: f64,
        icing_coef_v_area_half: f64,
        icing_coef_v_moment_full: f64,
        icing_coef_v_moment_half: f64,
    ) -> Self {
        Self {
            icing_stab,
            icing_m_timber,
            icing_m_v_full,
            icing_m_v_half,
            icing_m_h_full,
            icing_m_h_half,
            icing_coef_v_area_full,
            icing_coef_v_area_half,
            icing_coef_v_moment_full,
            icing_coef_v_moment_half,
        }
    }
}
///
impl IIcingStab for IcingStab {
    /// Масса льда на метр площади горизонтальной поверхности
    fn mass_h(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_m_h_full,
            "half" => self.icing_m_h_half,
            _ => 0.,
        }
    }
    /// Масса льда на метр площади горизонтальной поверхности
    fn mass_v(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_m_v_full,
            "half" => self.icing_m_v_half,
            _ => 0.,
        }
    }
    /// Коэффициент увеличения площади парусности несплощной 
    /// поверхности
    fn coef_v_area(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_coef_v_area_full,
            "half" => self.icing_coef_v_area_half,
            _ => 0.,
        }
    }
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    fn coef_v_moment(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_coef_v_moment_full,
            "half" => self.icing_coef_v_moment_half,
            _ => 0.,
        }
    }
}
#[doc(hidden)]
pub trait IIcingStab {
    /// Масса льда на метр площади горизонтальной поверхности
    fn mass_h(&self) -> f64;
    /// Масса льда на метр площади горизонтальной поверхности
    fn mass_v(&self) -> f64;
    /// Коэффициент увеличения площади парусности несплощной 
    /// поверхности
    fn coef_v_area(&self) -> f64;
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    fn coef_v_moment(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcingStab {
    mass_h: f64,
    mass_v: f64,
    coef_v_area: f64,
    coef_v_moment: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcingStab {
    pub fn new(mass_h: f64, mass_v: f64, coef_v_area: f64, coef_v_moment: f64) -> Self {
        Self { mass_h, mass_v, coef_v_area, coef_v_moment }
    }
}
#[doc(hidden)]
impl IIcingStab for FakeIcingStab {
    fn mass_h(&self) -> f64 {
        self.mass_h
    }
    fn mass_v(&self) -> f64 {
        self.mass_v
    }
    fn coef_v_area(&self) -> f64 {
        self.coef_v_area
    }
    fn coef_v_moment(&self) -> f64 {
        self.coef_v_moment
    }
}
