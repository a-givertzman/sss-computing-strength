//! Кривая, позволяет получать интерполированные значения 
use splines::{Interpolation, Key, Spline};

///
#[derive(Clone)]
pub struct Curve {
    spline: Spline<f64, f64>,
}

impl Curve {
    ///конструктор, получает вектор пар ключ/значение
    pub fn new(values: Vec<(f64, f64)>) -> Curve {
        let values: Vec<_> = values
            .into_iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::default()))
            .collect();
        Self {
            spline: Spline::from_vec(values),
        }
    }
    ///интерполированное значение, при выходе за границу приводится к ближайшему
    pub fn value(&self, key: f64) -> f64 {
        self.spline
            .clamped_sample(key)
            .expect("ошибка полуения значения")
    }
}
