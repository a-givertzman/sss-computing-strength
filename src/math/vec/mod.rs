//! Дополнительные операции над вектором чисел с плавающей точкой
pub mod integral_sum;
pub mod integral_cotes;

#[allow(unused)]
pub use integral_sum::IntegralSum as IntegralSum;
pub use integral_cotes::IntegralCotes as IntegralCotes;

/// Сумма сверху: $res_i = res_{i-1} + src_i, res_0 = 0$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut res: Vec<f64> = vec![1., 2., 3.].sum_above();
/// asserteq!(res, vec![0., 1., 3., 6.,]);
/// ```
pub trait SumAbove<T> {
    fn sum_above(&self) -> Vec<T>;
}

impl SumAbove<f64> for Vec<f64>  {
    fn sum_above(&self) -> Self {
        let mut data: Vec<f64> = vec![0.];
        let mut acc = 0.;
        for t in self {
            acc += t;
            data.push(acc);
        }
        data
    }    
}
/// Сдвиг каждого элемента на значение: $src_i = src_i + value$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.shift(1.);
/// asserteq!(vec, vec![ 2., 3., 4.,]);
/// ```
pub trait Shift {
    fn shift(&mut self, rhs: f64);
}

impl Shift for Vec<f64>  {
    fn shift(&mut self, rhs: f64) {
        self.iter_mut()
        .for_each(|v| {*v += rhs;} )
    }    
}
///
/// Деление каждого элемента на значение: $src_i = src_i/value$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.div_single(2.);
/// asserteq!(vec, vec![0.5, 1., 1.5,]);
/// ```
pub trait DivideSingle {
    fn div_single(&mut self, rhs: f64);
}
///
impl DivideSingle for Vec<f64>  {
    fn div_single(&mut self, rhs: f64) {
        self.iter_mut()
        .for_each(|v| {*v /= rhs;} )
    }    
}
///
/// Умножение каждого элемента на значение: $src_i = src_i * value$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.mul_single(2.);
/// asserteq!(vec, vec![2., 4., 6.,]);
/// ```
pub trait MultipleSingle {
    fn mul_single(&mut self, rhs: f64);
}
///
impl MultipleSingle for Vec<f64>  {
    fn mul_single(&mut self, rhs: f64) {
        self.iter_mut()
        .for_each(|v| {*v *= rhs;} )
    }    
}
///
/// Попарное сложение элементов векторов: $src1_i = src1_i + src2_i$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.add_vec(vec![3., 4., 5.]);
/// asserteq!(vec, vec![4., 6., 8.,]);
/// ```
pub trait AddVec {
    fn add_vec(&mut self, rhs: &Self);
}
///
impl AddVec for Vec<f64>  {   
    fn add_vec(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(v1, v2)| *v1 += v2 );
    }    
}
///
/// Попарное вычитание элементов векторов: $src1_i = src1_i - src2_i$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.sub_vec(vec![3., 4., 5.]);
/// asserteq!(vec, vec![-2., -2., -2.,]);
/// ```
pub trait SubVec {
    fn sub_vec(&mut self, rhs: &Self);
}
///
impl SubVec for Vec<f64>  {   
    fn sub_vec(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(v1, v2)| *v1 -= v2 );
    }    
}
///
/// Попарное умножение элементов векторов: $src1_i = src1_i * src2_i$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![1., 2., 3.];
/// vec.mul_vec(vec![3., 4., 5.]);
/// asserteq!(vec, vec![3., 8., 15.,]);
/// ```
pub trait MultipleVec {
    fn mul_vec(&mut self, rhs: &Self);
}
///
impl MultipleVec for Vec<f64>  {   
    fn mul_vec(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(v1, v2)| *v1 *= v2 );
    }    
}
///
/// Попарное деление элементов векторов: $src1_i = src1_i/src2_i$
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut vec: Vec<f64> = vec![2., 4., 6.];
/// vec.div_vec(vec![1., 2., 3.]);
/// asserteq!(vec, vec![2., 2., 2.,]);
/// ```
pub trait DivideVec {
    fn div_vec(&mut self, rhs: &Vec<f64>);
}
///
impl DivideVec for Vec<f64>  {   
    fn div_vec(&mut self, rhs: &Vec<f64>) {
        assert_eq!(self.len(), rhs.len());
        self.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(v1, v2)| *v1 /= v2 );
    }    
}
