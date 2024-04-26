//! Расчет угла крена на циркуляции

use std::rc::Rc;

use crate::{IMass, ILeverDiagram};

/// Расчет угла крена на циркуляции
pub struct Circulation {
    /// Эксплуатационная скорость судна, m/s
    v_0: f64,
    /// Длина судна по ватерлинии
    l_wl: f64,
    /// Осадка судна d
    d: f64,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
}
/// 
impl Circulation {
    /// Основной конструктор
    /// * v_0 - Эксплуатационная скорость судна, m/s
    /// * l_wl - Длина судна по ватерлинии
    /// * d - Осадка судна d
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    pub fn new(
        v_0: f64,
        l_wl: f64,
        d: f64,
        mass: Rc<dyn IMass>,
        lever_diagram: Rc<dyn ILeverDiagram>,
    ) -> Self {
        assert!(l_wl > 0., "l_wl {l_wl} > 0.");
        assert!(d > 0., "d {d} > 0.");
        Self {
            v_0,
            l_wl,
            d,
            mass,
            lever_diagram, 
        }
    }
    /// Плечо кренящего момента на циркуляции при скорости v, m/s
    pub fn heel_lever(&self, v: f64) -> f64 {
        // Кренящий момент на циркуляции
        let m_r = 0.2*(v*v*self.mass.sum()/self.l_wl)*(self.mass.shift().z() - self.d/2.).abs();
        // Плечо кренящего момента на циркуляции
        let l_r = m_r/self.mass.sum();
        log::info!("Circulation angle v:{v} m_r:{m_r} l_r:{l_r}");
        return l_r;
    }
}
///
impl ICirculation for Circulation {
    /// Угла крена на циркуляции при скорости v, m/s
    fn angle(&self) -> Option<f64> {
        // Угол соответствующий плечу кренящего момента
        let angle = self.lever_diagram.angle(self.heel_lever(self.v_0)).first().copied();
        log::info!("Circulation angle {:?} ", angle);
        return angle;
    }
    /// Максимальная скорость при заданном угле крена
    fn velocity(&self, src_angle: f64) -> f64 {
        let mut current_vel = 10.; // m/s
        let mut delta_vel = current_vel/2.;
        for i in 0..20 {
            let delta_angle = src_angle - self.lever_diagram.angle(self.heel_lever(current_vel)).first().copied().unwrap_or(90.);
            if delta_angle.abs() < 0.001 {
                break;
            }
            
            log::info!("Circulation velocity src_angle:{src_angle} current_vel:{current_vel} delta_vel:{delta_vel} delta_angle:{delta_angle}");
            current_vel = delta_vel*delta_angle.signum();
            delta_vel /= 2.;
        }
        return current_vel;
    }
}
#[doc(hidden)]
pub trait ICirculation {
    /// Угла крена на циркуляции при эксплуатационной скорости $V_0$
    fn angle(&self) -> Option<f64>;
    /// Максимальная скорость при заданном угле крена
    fn velocity(&self, angle: f64) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelleration {
    angle: Option<f64>,
    velocity: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelleration {
    pub fn new(
        angle: Option<f64>,
        velocity: f64,
    ) -> Self {
        Self {
            angle,
            velocity,
        }
    }
}
#[doc(hidden)]
impl ICirculation for FakeAccelleration {
    ///
    fn angle(&self) -> Option<f64> {
        self.angle
    }
    ///
    fn velocity(&self, _: f64) -> f64 {
        self.velocity
    }
}