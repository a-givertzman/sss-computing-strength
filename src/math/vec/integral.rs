//! Численное интегрирование

use crate::Error;
/// Численное интегрирование методом трапеций
/// Количество значений должно быть не меньше 2х
/// Вектор должен быть отсортирован по увеличению значений точек.
/// Значения точек - аргументы не должны совпадать.
/// Частичные отрезки могут быть не равномерными
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut res: Vec<f64> = vec![(1.,1.), (2. ,2.), (3., 3.)].integral();
/// asserteq!(res, 4);
/// ```
pub trait Integral {
    #[allow(unused)]
    fn integral(&self) -> Result<f64, Error>;
}
///
/// 
impl Integral for Vec<(f64, f64)>  {
    fn integral(&self) -> Result<f64, Error> {
        if self.len() < 2 {
            return Err(Error::FromString(format!("Integral error: self.len() < 2")));
        } 
        let mut sum = 0.;
        for i in 0..self.len()-1 {
            assert!(self[i+1].0 > self[i].0, "Integral self[i+1].0 > self[i].0");
            sum += (self[i].1 + self[i+1].1)*(self[i+1].0 - self[i].0)/2.;
        }
        Ok(sum)
    }    
}