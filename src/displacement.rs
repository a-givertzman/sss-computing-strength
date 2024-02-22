//! Водоизмещение судна
use crate::{frame::Frame, math::bound::Bound};

/// Водоизмещение судна. Вычисляет водоизмещение диапазона по  
/// интерполированным значениям погруженной площади шпангоутов.
pub struct Displacement {
    /// массив шпангоутов
    frames: Vec<Frame>,
    /// длинна судна
    ship_length: f64,
    /// шаг шпангоутов
    vec_step: f64,
}

impl Displacement {
    ///
    pub fn new(frames: Vec<Frame>, ship_length: f64) -> Self {
        Self { vec_step: ship_length/(frames.len() as f64 - 1.), frames, ship_length  }
    }
    /// Погруженный объем шпации.
    /// - bound: диапазон корпуса в длинну, для которого считается водоизмещение
    /// - draft: средняя осадка корпуса в диапазоне
    pub fn value(&self, bound: Bound, draft: f64) -> f64 {
        let area_start = self.area(bound.start(), draft);
        let area_end = self.area(bound.end(), draft);
        let result = bound.length() * (area_start + area_end)/2.;
        result
    }
    ///Интерполированние значение погруженной площади сечения.  
    ///Считается методом линейной интерполяции.
    /// - pos_x: координата шпангоута по х от центра судна
    /// - draft: осадка в районе шпангоута
    fn area(&self, pos_x: f64, draft: f64) -> f64 {
        assert!(pos_x >= -self.ship_length/2., "length = {} >= -self.ship_length/2. = {}", pos_x, -self.ship_length/2.);
        assert!(pos_x <= self.ship_length/2., "length = {} <= self.ship_length/2. = {}", pos_x, self.ship_length/2.);
        let index = (pos_x + self.ship_length/2.)/self.vec_step;
        let index_up = index.ceil();
        let index_down = index.floor();
        assert!(index_down >= 0., "length_down = {} >= 0.", index_down);
        assert!(index_up < self.frames.len() as f64, "length_up = {} < self.frames.len() = {}", index_up, self.frames.len());
        if index_up == index_down {
            return self.frames[index_up as usize].area(draft)
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
