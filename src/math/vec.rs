///
/// Сумма сверху $res_i = res_{i-1} + src_i, res_0 = 0$
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

///
/// Интегральная сумма $res_i = res_{i-1} + src_{i-1} + src_i, res_0 = 0$
pub trait IntegralSum<T> {
    fn integral_sum(&self) -> Vec<T>;
}

impl IntegralSum<f64> for Vec<f64>  {
    fn integral_sum(&self) -> Self {
        let mut data = vec![0.];
        for i in 1..(self.len()) {
            data.push(data[i - 1] + self[i - 1] + self[i]);
        }
        data
    }    
}
/// Сдвиг каждого элемента на значение
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
/// Деление каждого элемента на значение
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
/// Умножение каждого элемента на значение
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
/// Попарное сложение элементов векторов
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
/// Попарное вычитание элементов векторов
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
/// Попарное умножение элементов векторов
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
/// Попарное деление элементов векторов
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