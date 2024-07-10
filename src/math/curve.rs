//! Кривая, позволяет получать интерполированные значения
use splines::{Interpolation, Key, Spline};

///
/// Представление кривой в виде массива пар значений
/// - Обеспечивает получение промежуточных значений с помощью простой линейной интерполяции
#[derive(Clone, Debug)]
pub struct Curve {
    spline: Spline<f64, f64>,
}
///
///
impl Curve {
    ///
    /// Creates new instance of the Curve with linear interpolation  
    /// from vector of the key - value pairs
    pub fn new_linear(values: &Vec<(f64, f64)>) -> Curve {
        assert!(values.len() > 1, "Curve.new_linear | Input array must have at least two elements (values.len > 1), \nvalues: {:?}", values);
        let values: Vec<_> = values
            .iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::Linear))
            .collect();
        Self {
            spline: Spline::from_vec(values),
        }
    }
    ///
    /// Creates new instance of the Curve with CatmullRom interpolation  
    /// from vector of the key - value pairs
    /// Values must be sorted by key
    pub fn new_catmull_rom(src: &Vec<(f64, f64)>) -> Curve {
        assert!(src.len() > 2, "Curve.new | Input array must have at least four elements (values.len > 1), \nvalues: {:?}", src);
        let mut res = Vec::new();
        let mut src = src.clone();
        src.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Curve.new_catmull_rom src sort error!"));
        // Для метода CatmullRom добавляем по 3 значения вначало и конец вектора
        let delta_key = src[1].0 - src[0].0;
        assert!(delta_key > 0., "Curve.new_catmull_rom delta_key > 0 {}:{} {}:{}, src:{:?}", src[0].0, src[0].1 , src[1].0, src[1].1, &src );
        let delta_value = src[1].1 - src[0].1;
        res.push(Key::new(
            src[0].0 - delta_key * 3.,
            src[0].1 - delta_value * 3.,
            Interpolation::CatmullRom,
        ));
        res.push(Key::new(
            src[0].0 - delta_key * 2.,
            src[0].1 - delta_value * 2.,
            Interpolation::CatmullRom,
        ));
        res.push(Key::new(
            src[0].0 - delta_key,
            src[0].1 - delta_value,
            Interpolation::CatmullRom,
        ));

        let values: Vec<Key<_, _>> = src
            .iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::CatmullRom))
            .collect();
        res.append(&mut values.clone());

        let delta_key = src[src.len() - 1].0 - src[src.len() - 2].0;
        assert!(delta_key > 0., "Curve.new_catmull_rom delta_key > 0");
        let delta_value = src[src.len() - 1].1 - src[src.len() - 2].1;
        res.push(Key::new(
            src.last().unwrap().0 + delta_key,
            src.last().unwrap().1 + delta_value,
            Interpolation::CatmullRom,
        ));
        res.push(Key::new(
            src.last().unwrap().0 + delta_key * 2.,
            src.last().unwrap().1 + delta_value * 2.,
            Interpolation::CatmullRom,
        ));
        res.push(Key::new(
            src.last().unwrap().0 + delta_key * 3.,
            src.last().unwrap().1 + delta_value * 3.,
            Interpolation::CatmullRom,
        ));
        Self {
            spline: Spline::from_vec(res),
        }
    }
}

impl ICurve for Curve {
    /// Возвращает значение из таблицы по его ключу
    /// - если такого ключа нет, то возвращает промежуточное значение между двумя соседними с помощью линейной интерполяции
    /// - если ключ за пределами ключей таблицы, то вернет либо первое либо последнее значение
    /// - panic - если нет ключей
    fn value(&self, key: f64) -> f64 {
        let res = self
            .spline
            .clamped_sample(key)
            .expect(&"Curve.clamped_value | Ошибка получения значения: нет ключей".to_string());
        //    log::info!("\t Curve clamped_value key:{key} res:{res}");
        res
    }
    /// Численное интегрирование методом трапеций
    fn integral(&self, start: f64, end: f64) -> f64 {
        assert!(start <= end, "Integral start {start} <= end {end}");
        if start == end {
            return 0.;
        }
        let mut sum = 0.;
        let n = 100;
        let delta = (end - start)/n as f64;
        let mut last_value = self.value(start);
        let mut key = start;
        for _ in 0..n {
            key += delta;
            let next_value = self.value(key);
            sum += (last_value + next_value)*delta/2.;
            last_value = next_value;
        }
        sum
    }
}

#[doc(hidden)]
///
/// Interface used for testing purposes only
pub trait ICurve {
    fn value(&self, key: f64) -> f64;
    fn integral(&self, start: f64, end: f64) -> f64;
}
#[doc(hidden)]
// заглушка для тестирования
pub struct FakeCurve {
    value: f64,
    integral: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeCurve {
    pub fn new(value: f64, integral: f64) -> Self {
        Self { value, integral }
    }
}
#[doc(hidden)]
impl ICurve for FakeCurve {
    fn value(&self, _: f64) -> f64 {
        self.value
    }
    fn integral(&self, _: f64, _: f64) -> f64 {
        self.integral
    }
}
