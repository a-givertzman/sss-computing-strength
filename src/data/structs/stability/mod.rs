pub mod navigation_area;
pub mod multipler_s;
pub mod multipler_x1;
pub mod multipler_x2;
pub mod coefficient_k;
///
use super::DataArray;
///
use self::{coefficient_k::CoefficientKData, multipler_x1::MultiplerX1Data, multipler_x2::MultiplerX2Data};
///
pub type MultiplerX1Array = DataArray<MultiplerX1Data>;
///
impl MultiplerX1Array {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.b_div_d, v.x1) ).collect()
    }
}
///
pub type MultiplerX2Array = DataArray<MultiplerX2Data>;
///
impl MultiplerX2Array {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.c_b, v.x2) ).collect()
    }
}
///
pub type CoefficientKArray = DataArray<CoefficientKData>;
///
impl CoefficientKArray {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.a_div_l, v.k) ).collect()
    }
}