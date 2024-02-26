use splines::{Interpolation, Key, Spline};

/// Представление кривой в виде массива пар значений
/// - Обеспечивает получение промежуточных значений с помощью простой линейной интерполяции
pub struct Curve {
    spline: Spline<f64, f64>,
}
///
/// 
impl Curve {
    ///
    /// Creates new instance of the Curve from vector of the key - value pairs
    pub fn new(values: Vec<(f64, f64)>) -> Curve {
        let values: Vec<_> = values
            .into_iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::default()))
            .collect();
        Self {
            spline: Spline::from_vec(values),
        }
    }
    /// Возвращает значение из таблицы по его ключу
    /// - если такого ключа нет, то возвращает промежуточное значение между двумя соседними с помощью линейной интерполяции
    /// - если ключ за пределами ключей таблицы, то вернет либо первое либо последнее значение
    /// - panic - если не удалось получить - недостаточно ключей
    pub fn value(&self, key: f64) -> f64 {
        self.spline
            .clamped_sample(key)
            .expect("Curve.value | Ошибка полуения значения: недостаточно ключей")
    }
}
