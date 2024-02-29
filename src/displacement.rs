//! Водоизмещение судна
use crate::{frame::Frame, math::{bound::Bound, curve::{Curve, ICurve}}};

/// Водоизмещение судна. Вычисляет водоизмещение диапазона по  
/// интерполированным значениям погруженной площади шпангоутов.
pub struct Displacement {
    /// Шпангоуты
    frames: Vec<Frame>,
    /// Кривая с индексами шпангоутов
    curve: Curve,
}
///
impl Displacement {
    /// Основной конструктор
    pub fn new(frames: Vec<Frame>, curve: Curve) -> Self {
        assert!(frames.len() > 0, "frames.len() {} > 0", frames.len());
        Self {
            frames,
            curve,
        }
    }
    /// Вспомогательный конструктор, создает кривую из данных шпангоутов
    /// * frames: отсортированный по по индексам массив шпангоутов
    /// * ship_length:  Длинна корпуса судна
    pub fn from_frames(frames: Vec<Frame>) -> Self {
        assert!(frames.len() > 0, "frames.len() {} > 0", frames.len());
        let curve = Curve::new(frames.iter().enumerate().map(|(index, f)| (f.delta_x(), index as f64)).collect());
        Self::new(frames, curve)
    }
    /// Погруженный объем шпации.
    /// - bound: диапазон корпуса в длинну, для которого считается водоизмещение
    /// - draft: средняя осадка корпуса в диапазоне
    pub fn value(&self, bound: Bound, draft: f64) -> f64 {
        let area_start = self.area(bound.start(), draft);
        let area_end = self.area(bound.end(), draft);
        let result = bound.length() * (area_start + area_end) / 2.;
        result
    }
    /// Интерполированние значение погруженной площади сечения.  
    /// Считается методом линейной интерполяции.
    /// - pos_x: координата шпангоута по х от центра судна
    /// - draft: осадка в районе шпангоута
    fn area(&self, pos_x: f64, draft: f64) -> f64 {

        let index = self.curve.value(pos_x);
        let index_up = index.ceil();
        let index_down = index.floor();
        assert!(index_down >= 0., "length_down = {} >= 0.", index_down);
        assert!(
            index_up < self.frames.len() as f64,
            "length_up = {} < self.frames.len() = {}",
            index_up,
            self.frames.len()
        );
        if index_up == index_down {
            return self.frames[index_up as usize].area(draft);
        }
        let coeff_len_up = index - index_down;
        let coeff_len_down = index_up - index;
        let length_up = index_up as usize;
        let length_down = index_down as usize;
        let frame_up = &self.frames[length_up];
        let frame_down = &self.frames[length_down];
        let result = frame_up.area(draft) * coeff_len_up + frame_down.area(draft) * coeff_len_down;
        result
    }
}
