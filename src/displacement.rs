//! Водоизмещение судна
use crate::{frame::Frame, math::bound::Bound};

/// Водоизмещение судна. Вычисляет водоизмещение диапазона по  
/// интерполированным значениям погруженной площади шпангоутов.
pub struct Displacement {
    /// массив шпангоутов
    frames: Vec<Frame>,
}

impl Displacement {
    ///
    pub fn new(frames: Vec<Frame>) -> Self {
        assert!(frames.len() > 1, "frames.len() {} > 0", frames.len());
        Self { frames }
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
    ///Интерполированние значение погруженной площади сечения.  
    ///Считается методом линейной интерполяции.
    /// - pos_x: координата шпангоута по х от центра судна
    /// - draft: осадка в районе шпангоута
    fn area(&self, pos_x: f64, draft: f64) -> f64 {
        assert!(
            pos_x
                >= self
                    .frames
                    .first()
                    .expect("Displacement error: no frames!")
                    .shift_x(),
            "length = {} >= self.frames.first().shift_x = {}",
            pos_x,
            self.frames
                .first()
                .expect("Displacement error: no frames!")
                .shift_x()
        );
        assert!(
            pos_x
                <= self
                    .frames
                    .last()
                    .expect("Displacement error: no frames!")
                    .shift_x(),
            "length = {} <= self.frames.last().shift_x = {}",
            pos_x,
            self.frames
                .last()
                .expect("Displacement error: no frames!")
                .shift_x()
        );
        let (index_up, frame_up) = &self.frames.iter().enumerate().find(|(_, v)| v.shift_x() >= pos_x ).expect("Displacement error: can't find frame");
        if *index_up == 0 || *index_up == self.frames.len() - 1 {
            return self.frames[*index_up].area(draft);
        }
        let frame_down = &self.frames[*index_up - 1];
        let delta_x = frame_up.shift_x() - frame_down.shift_x();
        let coeff_len_up = (frame_up.shift_x()-pos_x) / delta_x;
        let coeff_len_down = 1. - coeff_len_up;
        assert!(coeff_len_up + coeff_len_down == 1., "coeff_len_up {} + coeff_len_down {} == 1.", coeff_len_up, coeff_len_down);
        let result = frame_up.area(draft) * coeff_len_up + frame_down.area(draft) * coeff_len_down;
        result
    }
}
