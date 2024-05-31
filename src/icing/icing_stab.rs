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
    /// Коэффициент площади парусности несплощной
    /// поверхности при учете полного обледенения
    icing_coef_v_area_full: f64,
    /// Коэффициент площади парусности несплощной
    /// поверхности при учете частичного обледенения
    icing_coef_v_area_half: f64,
    /// Коэффициент площади парусности несплощной
    /// поверхности при отсутствии обледенения
    icing_coef_v_area_zero: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете полного обледенения
    icing_coef_v_moment_full: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете частичного обледенения
    icing_coef_v_moment_half: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при отсутствии обледенения
    icing_coef_v_moment_zero: f64,
}
///
impl IcingStab {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения: "full", "half" или отсутствует
    /// * icing_m_timber - Масса льда на квадратный метр площади горизонтальной поверхности
    /// палубного лесного груза
    /// * icing_m_v_full - Масса льда на квадратный метр площади парусности
    /// * icing_m_v_half
    /// * icing_m_h_full - Масса льда на квадратный метр площади горизонтальной поверхности
    /// * icing_m_h_half
    /// * icing_coef_v_area_full - Коэффициент площади парусности несплощной поверхности
    /// * icing_coef_v_area_half
    /// * icing_coef_v_area_zero
    /// * icing_coef_v_moment_full - Коэффициент статического момента
    /// * icing_coef_v_moment_half
    /// * icing_coef_v_moment_zero
    pub fn new(
        icing_stab: String,
        icing_m_timber: f64,
        icing_m_v_full: f64,
        icing_m_v_half: f64,
        icing_m_h_full: f64,
        icing_m_h_half: f64,
        icing_coef_v_area_full: f64,
        icing_coef_v_area_half: f64,
        icing_coef_v_area_zero: f64,
        icing_coef_v_moment_full: f64,
        icing_coef_v_moment_half: f64,
        icing_coef_v_moment_zero: f64,
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
            icing_coef_v_area_zero,
            icing_coef_v_moment_full,
            icing_coef_v_moment_half,
            icing_coef_v_moment_zero,
        }
    }
}
///
impl IIcingStab for IcingStab {
    /// Масса льда на метр площади поверхности открытой палубы
    fn mass_desc_h(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_m_h_full,
            "half" => self.icing_m_h_half,
            _ => 0.,
        }
    }
    /// Масса льда на метр площади палубного груза - леса
    fn mass_timber_h(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" | "half" => self.icing_m_timber,
            _ => 0.,
        }
    }
    /// Масса льда на метр площади парусности
    fn mass_v(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_m_v_full,
            "half" => self.icing_m_v_half,
            _ => 0.,
        }
    }
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности с учетом обледенения
    fn coef_v_area(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_coef_v_area_full,
            "half" => self.icing_coef_v_area_half,
            _ => self.icing_coef_v_area_zero,
        }
    }
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности без учета обледенения
    fn coef_v_ds_area(&self) -> f64 {
        self.icing_coef_v_area_zero
    }
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    fn coef_v_moment(&self) -> f64 {
        match self.icing_stab.as_str() {
            "full" => self.icing_coef_v_moment_full,
            "half" => self.icing_coef_v_moment_half,
            _ => self.icing_coef_v_moment_zero,
        }
    }
    /// Признал наличия обледенения
    fn is_some(&self) -> bool {
        match self.icing_stab.as_str() {
            "full" | "half" => true,
            _ => false,
        }
    }
}
#[doc(hidden)]
pub trait IIcingStab {
    /// Масса льда на метр площади поверхности открытой палубы
    fn mass_desc_h(&self) -> f64;
    /// Масса льда на метр площади палубного груза - леса
    fn mass_timber_h(&self) -> f64;
    /// Масса льда на метр площади парусности
    fn mass_v(&self) -> f64;
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности с учетом обледенения
    fn coef_v_area(&self) -> f64;
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности без учета обледенения
    fn coef_v_ds_area(&self) -> f64;
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    fn coef_v_moment(&self) -> f64;
    /// Признал наличия обледенения
    fn is_some(&self) -> bool;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcingStab {
    mass_desc_h: f64,
    mass_timber_h: f64,
    mass_v: f64,
    coef_v_area: f64,
    coef_v_ds_area: f64,
    coef_v_moment: f64,
    is_some: bool,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcingStab {
    pub fn new(
        mass_desc_h: f64,
        mass_timber_h: f64,
        mass_v: f64,
        coef_v_area: f64,
        coef_v_ds_area: f64,
        coef_v_moment: f64,
        is_some: bool,
    ) -> Self {
        Self {
            mass_desc_h,
            mass_timber_h,
            mass_v,
            coef_v_area,
            coef_v_ds_area,
            coef_v_moment,
            is_some,
        }
    }
}
#[doc(hidden)]
impl IIcingStab for FakeIcingStab {
    fn mass_desc_h(&self) -> f64{
        self.mass_desc_h
    }

    fn mass_timber_h(&self) -> f64 {
        self.mass_timber_h
    }

    fn mass_v(&self) -> f64 {
        self.mass_v
    }

    fn coef_v_area(&self) -> f64 {
        self.coef_v_area
    }

    fn coef_v_ds_area(&self) -> f64 {
        self.coef_v_ds_area
    }

    fn coef_v_moment(&self) -> f64 {
        self.coef_v_moment
    }

    fn is_some(&self) -> bool {
        self.is_some
    }
}
