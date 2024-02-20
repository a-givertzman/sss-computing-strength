//use std::collections::HashMap;
use crate::{frame::Frame, math::bound::Bound};

///Класс, инкапсулирующий функционал расчета водоизмещения
pub struct Displacement {
    frames: Vec<Frame>,
    ship_length: f64,
    vec_step: f64,
}

impl Displacement {
    ///
    pub fn new(frames: Vec<Frame>, ship_length: f64) -> Self {
        Self { vec_step: ship_length/(frames.len() as f64 - 1.), frames, ship_length  }
    }
    ///погруженный объем шпации
    pub fn value(&self, bound: Bound, draft: f64) -> f64 {
        let area_start = self.area(bound.start(), draft);
        let area_end = self.area(bound.end(), draft);
        let result = bound.length() * (area_start + area_end)/2.;
        result
    }
    ///Интерполированние значение погруженной площади сечения.  
    ///Получается методом линейной интерполяции.
    fn area(&self, mut length: f64, draft: f64) -> f64 {
        assert!(length >= -self.ship_length/2., "length = {} >= -self.ship_length/2. = {}", length, -self.ship_length/2.);
        assert!(length <= self.ship_length/2., "length = {} <= self.ship_length/2. = {}", length, self.ship_length/2.);
        length = (length + self.ship_length/2.)/self.vec_step;
        let length_up = length.ceil();
        let length_down = length.floor();
        if length_up == length_down {
            return self.frames[length_up as usize].area(draft)
        }
        let delta_len = length_up - length_down;
        let coeff_len_up = (length_up - length) / delta_len;        
        let coeff_len_down = (length - length_down) / delta_len; 
        assert!(length_down >= 0., "length_down = {} >= 0.", length_down);
        assert!(length_up < self.frames.len() as f64, "length_up = {} < self.frames.len() = {}", length_up, self.frames.len());
        let length_up = length_up as usize;
        let length_down = length_down as usize;               
        let frame_up = &self.frames[length_up];
        let frame_down = &self.frames[length_down];
        let result = frame_up.area(draft) * coeff_len_up + frame_down.area(draft) * coeff_len_down;
 //       dbg!(&length, &draft, &length_up, &length_down, &delta_len, &coeff_len_up, &coeff_len_down, &result);
        result
    }
}
