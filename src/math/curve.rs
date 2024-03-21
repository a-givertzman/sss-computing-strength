//! Кривая, позволяет получать интерполированные значения
use splines::{Interpolation, Key, Spline};

///
/// Представление кривой в виде массива пар значений
/// - Обеспечивает получение промежуточных значений с помощью простой линейной интерполяции
#[derive(Clone)]
pub struct Curve {
    spline: Spline<f64, f64>,
}
///
/// 
impl Curve {
    ///
    /// Creates new instance of the Curve from vector of the key - value pairs
    pub fn new(values: &Vec<(f64, f64)>) -> Curve {
        assert!(values.len() > 1, "Curve.new | Input array must have at least two elements (values.len > 1), \nvalues: {:?}", values);
        let values: Vec<_> = values
            .iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::default()))
            .collect();
        Self {
            spline: Spline::from_vec(values),
        }
    }
}

impl ICurve for Curve {
    /// Возвращает значение из таблицы по его ключу
    /// - если такого ключа нет, то возвращает промежуточное значение между двумя соседними с помощью линейной интерполяции
    /// - если ключ за пределами ключей таблицы, то вернет либо первое либо последнее значение
    /// - panic - если нет ключей

    fn value(&self, key: f64) -> f64 {
        let res = self.spline
            .clamped_sample(key)
            .expect(&"Curve.value | Ошибка полуения значения: нет ключей".to_string());
    //    log::info!("\t Curve value key:{key} res:{res}");
        res
    }
}

#[doc(hidden)]
///
/// Interface used for testing purposes only 
pub trait ICurve {
    fn value(&self, key: f64) -> f64;
}
#[doc(hidden)]
// заглушка для тестирования
pub struct FakeCurve {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeCurve {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}
#[doc(hidden)]
impl ICurve for FakeCurve {
    fn value(&self, _: f64) -> f64 {
        self.value
    }
}
