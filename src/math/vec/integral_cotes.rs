//! Численное интегрирование по формуле Котеса

use crate::Error;
/// Численное интегрирование по формуле Котеса: $\int_{a}^{b}f(x)dx = h\left(\frac{f_0 + f_n}{2} + \sum_{i=1}^{n-1} \right)$
/// где $h = (b - a)/n$, n - количество элементов
/// применимо для распределения значений на равные элементарные отрезки h
/// Количество значений должно быть больше 2х
/// 
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut res: Ok(Vec<f64>) = vec![1., 2., 3.].integral_cotes(1);
/// asserteq!(res, 4);
/// ```
pub trait IntegralCotes {
    fn integral_cotes(&self, h: f64) -> Result<f64, Error> ;
}
///
/// 
impl IntegralCotes for Vec<f64>  {
    fn integral_cotes(&self, h: f64) -> Result<f64, Error> {
        if self.len() < 2 {
            return Err(Error::FromString(format!("IntegralCotes error: len() < 2")));
        }
        let first_last = self.first().expect("IntegralCotes err first") + self.last().expect("IntegralCotes err last");
        Ok(h*(self.iter().sum::<f64>() - first_last + first_last*0.5))
    }    
}