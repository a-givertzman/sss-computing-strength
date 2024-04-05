//! Водоизмещение судна

use crate::math::Bound;

use super::frame::Frame;


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
    pub fn value(&self, bound: Bound, draft_start: f64, draft_end: f64) -> f64 {
        let area_start = self.area(bound.start(), draft_start);
        let area_end = self.area(bound.end(), draft_end);
        let result = bound.length() * (area_start + area_end) / 2.;
 //     log::info!("\t Displacement value bound:{bound} draft_start:{draft_start} draft_end:{draft_end} area_start:{area_start} area_end:{area_end} result:{result}");
        result
    }
    ///Интерполированние значение погруженной площади сечения.  
    ///Считается методом линейной интерполяции.
    /// - pos_x: координата шпангоута по х от центра судна
    /// - draft: осадка в районе шпангоута
    fn area(&self, pos_x: f64, draft: f64) -> f64 {
        if self.frames.first().expect("Displacement error: can't find first frame").shift_x() >= pos_x {
  //          log::info!("\t Displacement area pos_x:{pos_x} draft:{draft}  frames.first().shift_x() >= pos_x");      
            return self.frames.first().unwrap().area(draft)*2.; // для крайних шпангоутов удваеваем площадь
        }
        if self.frames.last().expect("Displacement error: can't find last frame").shift_x() <= pos_x {
  //          log::info!("\t Displacement area pos_x:{pos_x} draft:{draft}  frames.last().shift_x() <= pos_x");
            return self.frames.last().unwrap().area(draft)*2.; // для крайних шпангоутов удваеваем площадь
        }
        let (index_up, frame_up) = &self.frames.iter().enumerate().find(|(_, v)| v.shift_x() >= pos_x ).expect("Displacement error: can't find frame");             
        if *index_up == 0 {
            return self.frames[*index_up].area(draft)*2.; // для крайних шпангоутов удваеваем площадь
        }
        let frame_down = &self.frames[*index_up - 1];
        let delta_x = frame_up.shift_x() - frame_down.shift_x();
        let coeff_len_down = (frame_up.shift_x()-pos_x) / delta_x;
        let coeff_len_up = 1. - coeff_len_down;
        assert!(coeff_len_up + coeff_len_down == 1., "coeff_len_up {} + coeff_len_down {} == 1.", coeff_len_up, coeff_len_down);
        let result = frame_up.area(draft) * coeff_len_up + frame_down.area(draft) * coeff_len_down;
   //     log::info!("\t Displacement area pos_x:{pos_x} draft:{draft}  index_up:{index_up} delta_x:{delta_x} coeff_len_up:{coeff_len_up} coeff_len_down:{coeff_len_down} result:{result}");
        result
    }
}
