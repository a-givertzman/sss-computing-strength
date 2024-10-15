//! Интегральная сумма
/// Интегральная сумма: $res_i = res_{i-1} + src_{i-1} + src_i, res_0 = 0$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut res: Vec<f64> = vec![1., 2., 3.].integral_sum();
/// asserteq!(res, vec![0., 1., 4., 9.,]);
/// ```
pub trait IntegralSum<T> {
    fn integral_sum(&self) -> Vec<T>;
}
// 
impl IntegralSum<f64> for Vec<f64>  {
    fn integral_sum(&self) -> Self {
        let mut data = vec![0.];
        for i in 1..(self.len()) {
            data.push(data[i - 1] + self[i - 1] + self[i]);
        }
        data
    }    
}