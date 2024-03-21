//! Кривая поверхность, позволяет получать интерполированные значения
use super::{Curve, ICurve};

///
/// Представление поверхности в виде массива кривых пар значений
/// - Обеспечивает получение промежуточных значений с помощью простой линейной интерполяции
#[derive(Clone)]
pub struct Curve2D {
    curves: Vec<(f64, Curve)>,
}
///
impl Curve2D {
    /// Основной конструктор
    #[allow(dead_code)]
    pub fn new(curves: Vec<(f64, Curve)>) -> Self {
        assert!(curves.len() > 1, "curves.len() {} > 0", curves.len());
        Self { curves }
    }
    /// Конструктор из матрицы значений
    #[allow(dead_code)]
    pub fn from_values(mut values: Vec<(f64, Vec<(f64, f64)>)>) -> Self {
        assert!(values.len() > 1, "values.len() {} > 0", values.len());
        values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self::new(values.into_iter().map(|(value, vector)| (value, Curve::new(&vector))).collect())
    }
}
///
impl ICurve2D for Curve2D {
    /// Возвращает значение из таблицы по его ключу
    /// - если такого ключа нет, то возвращает промежуточное значение между двумя соседними с помощью линейной интерполяции
    /// - если ключ за пределами ключей таблицы, то вернет либо первое либо последнее значение
    /// - panic - если нет ключей
    ///
    fn value(&self, key1: f64, key2: f64) -> f64 {   
        for index in 0..self.curves.len() {
            if self.curves[index].0 >= key1 {
                if index == 0 {
                    let res = self.curves[0].1.value(key2);
    //                log::info!("\t Curve2D value key1:{key1} key2:{key2} index = 0 res:{res}");
                    return res;
                }
                let res1 = self.curves[index-1].1.value(key2);
                let res2 = self.curves[index].1.value(key2);
                let delta = self.curves[index].0 - self.curves[index-1].0;
                let coeff1 = (self.curves[index].0 - key1) / delta;
                let coeff2 = 1. - coeff1;
                let result = res1 * coeff1 + res2 * coeff2;
   //            log::info!("\t Curve2D value key1:{key1} key2:{key2} res1:{res1} res2:{res2} delta:{delta} coeff1:{coeff1} coeff2:{coeff2} result:{result}");
                return result;
            }
        }
        self.curves.last().expect("Curve2D value error: no last curve").1.value(key2)
    }
    /// Количество элементов
    fn len(&self) -> usize {
        self.curves.len()
    }
}

#[doc(hidden)]
///
/// Interface used for testing purposes only 
pub trait ICurve2D {
    fn len(&self) -> usize;
    fn value(&self, key1: f64, key2: f64,) -> f64;
}
#[doc(hidden)]
// заглушка для тестирования
pub struct FakeCurve2D {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeCurve2D {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}
#[doc(hidden)]
impl ICurve2D for FakeCurve2D {
    ///
    fn value(&self, _: f64, _: f64,) -> f64 {
        self.value
    }
    ///
    fn len(&self) -> usize {
        20
    }
}
