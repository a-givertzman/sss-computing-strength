//! Парусность судна

use std::rc::Rc;

use crate::{Curve, ICurve, ILoad, Moment, Position};

/// Парусность судна, площадь и положение 
/// центра относительно миделя и ОП
#[derive(Clone)]
pub struct Windage {
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Тип обледенения для расчета парусности
    icing_stab: String,
    /// Площадь парусности судна для минимальной осадки
    area: f64,
    /// Смещение центра парусности судна для минимальной осадки
    shift: Position, 
    /// Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки
    delta_area: f64,
    /// Разница в статических моментах относительно миделя (x) и ОП (z) 
    /// соответствеено для осадки по ЛГВЛ и минимальной осадки
    delta_moment: Moment,
    /// Текущая осадка
    drought_current: f64,
    /// Минимальная осадка  
    draught_min: f64,    
    /// Осадка по ЛГВЛ
    draught_slw: f64,  
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    volume_shift: f64, 
    /// Площадь парусности, м^2
    a_v: Option<f64>,      
    /// Плечо парусности, м
    z_v: Option<f64>, 
}
///
impl Windage {
    /// Аргументы конструктора:  
    /// * loads_cargo - грузы судна
    /// * icing_stab - Тип обледенения для расчета парусности
    /// * area - Площадь парусности судна
    /// * shift - Смещение центра парусности судна
    /// * delta_area - Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки
    /// * delta_moment - Разница в статических моментах относительно миделя (x) и ОП (z) 
    ///                 соответствеено для осадки по ЛГВЛ и минимальной осадки
    /// * drought_current - Текущая осадка
    /// * draught_min - Минимальная осадка  
    /// * draught_slw - Осадка по ЛГВЛ
    /// * volume_shift - Отстояние по вертикали центра площади проекции подводной части корпуса
    pub fn new(
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        icing_stab: String,
        area: f64,
        shift: Position,
        delta_area: f64,
        delta_moment: Moment,
        drought_current: f64,
        draught_min: f64,    
        draught_slw: f64,   
        volume_shift: f64,  
    ) -> Self {
        assert!(area >= 0., "area {area} >= 0");
        assert!(delta_area >= 0., "delta_area {delta_area} >= 0");
        assert!(drought_current >= 0., "drought_current {drought_current} >= 0");
        assert!(draught_min >= 0., "draught_min {draught_min} >= 0");
        assert!(draught_slw >= 0., "draught_slw {draught_slw} >= 0");
        Self {
            loads_cargo,
            icing_stab,
            area,
            shift,
            delta_area,
            delta_moment,
            drought_current,
            draught_min,    
            draught_slw,  
            volume_shift,
            a_v: None,            
            z_v: None,
        }
    }
    ///
    fn calculate(&mut self) {
        // Площадь и статический момент парусности сплошных поверхностей 
        // для минимальной осадки без палубного груза
        let a_v_cs_dmin1 = self.area;
        // Cтатический момент относительно миделя (x) и ОП (z) 
        // для минимальной осадки
        let moment = Moment::from_pos(self.shift.clone(), self.area);
        let m_vx_cs_dmin1 = moment.x();
        let m_vz_cs_dmin1 = moment.z();

        // Площадь и статический момент площади парусности палубного груза
        let a_v_pg = self.loads_cargo.iter().map(|l| l.windage_area()).sum();
        let shift_pg: Position = self.loads_cargo.iter().map(|l| l.windage_shift()).sum();
        let m_pg = Moment::from_pos(shift_pg.clone(), a_v_pg);

        // Площадь и статический момент для минимальной осадки
        let a_v_cs_dmin = a_v_cs_dmin1 + a_v_pg;
        let m_vx_cs_dmin = m_vx_cs_dmin1 + m_pg.x();
        let m_vz_cs_dmin = m_vz_cs_dmin1 + m_pg.z();    

        // частичное/полное обледенение
        let (a_v_ds_ice, m_vx_ds_ice, m_vz_ds_ice) = if self.icing_stab == "full" {
            (0.1*a_v_cs_dmin, 0., 0.2*m_vz_cs_dmin)
        } else if self.icing_stab == "half" {
            (0.075*a_v_cs_dmin, 0., 0.15*m_vz_cs_dmin)
        } else { // "none"
            (0., 0., 0.)
        };

        // Парусность несплошных поверхностей
        let a_v_ds = 0.05*a_v_cs_dmin;
        let m_vx_ds = 0.;
        let m_vz_ds = 0.1*m_vz_cs_dmin;        

        // Первая точка для интерполяции: минимальная осадка
        // Площадь парусности судна для минимальной осадки 
        let a_v_dmin = a_v_cs_dmin + a_v_ds + a_v_ds_ice ;
        let m_vx_dmin = m_vx_cs_dmin + m_vx_ds + m_vx_ds_ice;
        let m_vz_dmin = m_vz_cs_dmin + m_vz_ds + m_vz_ds_ice;

        // Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки 
        let delta_a_v_summer = self.delta_area;
        // Разница в статических моментах относительно миделя и ОП соответствеено
        let delta_m_vx_summer = self.delta_moment.x();
        let delta_m_vz_summer = self.delta_moment.z();

        // Вторая точка для интерполяции: максимальная осадка
        // Площадь парусности судна для осадки по ЛГВЛ
        let a_v_summer = a_v_dmin - delta_a_v_summer;
        // статические моменты площади парусности для осадки по ЛГВЛ
        let m_vx_summer = m_vx_dmin - delta_m_vx_summer;
        let m_vz_summer = m_vz_dmin - delta_m_vz_summer;

        let a_v = Curve::new_linear(&vec![(self.draught_min, a_v_dmin), (self.draught_slw, a_v_summer)]).value(self.drought_current);
        let m_vx = Curve::new_linear(&vec![(self.draught_min, m_vx_dmin), (self.draught_slw, m_vx_summer)]).value(self.drought_current);
        let m_vz = Curve::new_linear(&vec![(self.draught_min, m_vz_dmin), (self.draught_slw, m_vz_summer)]).value(self.drought_current);

        let x_v = m_vx/a_v;
        let z_v_bp = m_vz/a_v;
        let z_v = z_v_bp - self.volume_shift;
        self.a_v = Some(a_v);
        self.z_v = Some(z_v);

        log::info!("Windage a_v_cs_dmin1:{a_v_cs_dmin1} m_vx_cs_dmin1:{m_vx_cs_dmin1} m_vz_cs_dmin1:{m_vz_cs_dmin1} 
        d_min:{} d_slw:{} d_current:{}
        delta_a_v_summer:{delta_a_v_summer} delta_m_vx_summer:{delta_m_vx_summer} delta_m_vz_summer:{delta_m_vz_summer}
        a_v_pg:{a_v_pg} shift_pg.z:{} m_pg:{m_pg}  
        a_v_cs_dmin:{a_v_cs_dmin} m_vx_cs_dmin:{m_vx_cs_dmin} m_vz_cs_dmin:{m_vz_cs_dmin}   
        a_v_ds:{a_v_ds} m_vx_ds:{m_vx_ds} m_vz_ds:{m_vz_ds}   
        a_v_dmin:{a_v_dmin} m_vx_dmin:{m_vx_dmin} m_vz_dmin:{m_vz_dmin}
        a_v_summer:{a_v_summer} m_vx_summer:{m_vx_summer} m_vz_summer:{m_vz_summer}
        a_v:{a_v} m_vx:{m_vx} m_vz:{m_vz} x_v:{x_v} z_v:{z_v}",
        self.draught_min, self.draught_slw, self.drought_current, shift_pg.z());
    }
}
///
impl IWindage for Windage {
    /// Площадь парусности, м^2
    fn a_v(&mut self) -> f64 {
        if self.a_v.is_none() {
            self.calculate();
        }

        self.a_v.expect("Windage a_v error: no value!")
    }    
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64 {
        if self.z_v.is_none() {
            self.calculate();
        }

        self.z_v.expect("Windage z_v error: no value!")
    }
}
#[doc(hidden)]
pub trait IWindage {
    /// Площадь парусности, м^2
    fn a_v(&mut self) -> f64;
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWindage {
    /// Площадь парусности, м^2
    a_v: f64,
    /// Плечо парусности, м
    z_v: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWindage {
    pub fn new(
        a_v: f64,
        z_v: f64,
    ) -> Self {
        Self {
            a_v,
            z_v,
        }
    }
}
#[doc(hidden)]
impl IWindage for FakeWindage {
    /// Площадь парусности, м^2
    fn a_v(&mut self) -> f64 {
        self.a_v
    }    
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64 {
        self.z_v
    }
}


