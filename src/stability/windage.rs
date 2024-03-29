//! Парусность судна

use std::rc::Rc;

use crate::{Curve, ICurve, ILoad, Moment, Position};

/// Парусность судна, площадь и положение 
/// центра относительно миделя и ОП
pub struct Windage {
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Площадь парусности судна для минимальной осадки
    area: f64,
    /// Cтатический момент относительно миделя (x) и ОП (z) 
    /// для минимальной осадки
    moment: Moment,
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
    draught_lgvl: f64,  
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    volume_shift: f64,  
}
///
impl Windage {
    /// Аргументы конструктора:  
    /// * loads_cargo - грузы судна
    /// * area - Площадь парусности судна
    /// * moment - Cтатический момент относительно миделя (x) и ОП (z) 
    ///             для минимальной осадки
    /// * shift - Смещение центра парусности судна
    /// * delta_area - Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки
    /// * delta_moment - Разница в статических моментах относительно миделя (x) и ОП (z) 
    ///                 соответствеено для осадки по ЛГВЛ и минимальной осадки
    /// * drought_current - Текущая осадка
    /// * draught_min - Минимальная осадка  
    /// * draught_lgvl - Осадка по ЛГВЛ
    /// * volume_shift - Отстояние по вертикали центра площади проекции подводной части корпуса
    pub fn new(
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        area: f64,
        moment: Moment,
        shift: Position,
        delta_area: f64,
        delta_moment: Moment,
        drought_current: f64,
        draught_min: f64,    
        draught_lgvl: f64,   
        volume_shift: f64,  
    ) -> Self {
        assert!(area >= 0., "area {area} >= 0");
        assert!(delta_area >= 0., "delta_area {delta_area} >= 0");
        assert!(drought_current >= 0., "drought_current {drought_current} >= 0");
        assert!(draught_min >= 0., "draught_min {draught_min} >= 0");
        assert!(draught_lgvl >= 0., "draught_lgvl {draught_lgvl} >= 0");
        Self {
            loads_cargo,
            area,
            moment,
            shift,
            delta_area,
            delta_moment,
            drought_current,
            draught_min,    
            draught_lgvl,  
            volume_shift,
        }
    }
    ///
    fn calculate(&self) -> f64 {
        // Площадь и статический момент парусности сплошных поверхностей 
        // для минимальной осадки без палубного груза
        let a_v_cs_dmin1 = self.area;
        let m_vx_cs_dmin1 = self.moment.x();
        let m_vz_cs_dmin1 = self.moment.z();
        let draught = vec![self.draught_min, self.draught_lgvl];

        // Площадь и статический момент площади парусности палубного груза
        let a_v_pg = self.loads_cargo.iter().map(|l| l.windage_area()).sum();
        let shift_pg: Position = self.loads_cargo.iter().map(|l| l.windage_shift()).sum();
        let m_pg = Moment::from_pos(shift_pg, a_v_pg);

        // Площадь и статический момент для минимальной осадки
        let a_v_cs_dmin = a_v_cs_dmin1 + a_v_pg;
        let m_vx_cs_dmin = m_vx_cs_dmin1 + m_pg.x();
        let m_vz_cs_dmin = m_vz_cs_dmin1 + m_pg.z();    

        // частичное/полное обледенение
        let a_v_ds_ice = vec![0.15*a_v_cs_dmin, 0.1*a_v_cs_dmin];
        let m_vz_ds_ice = vec![0.15*m_vz_cs_dmin, 0.2*m_vz_cs_dmin];
        let m_vx_ds_ice = vec![0., 0.];

        // Парусность несплошных поверхностей
        let a_v_ds = 0.05*a_v_cs_dmin;
        let m_vx_ds = 0.;
        let m_vz_ds = 0.1*m_vz_cs_dmin;        

        // Первая точка для интерполяции: минимальная осадка
        // Площадь парусности судна для минимальной осадки 
        let a_v_dmin = a_v_ds_ice.iter().map(|v| v + a_v_cs_dmin + a_v_ds);
        let m_vx_dmin = m_vx_ds_ice.iter().map(|v| v + m_vx_cs_dmin + m_vx_ds);
        let m_vz_dmin = m_vz_ds_ice.iter().map(|v| v + m_vz_cs_dmin + m_vz_ds);


        // Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки 
        let delta_a_v_summer = self.delta_area;
        // Разница в статических моментах относительно миделя и ОП соответствеено
        let delta_m_vx_summer = self.delta_moment.x();
        let delta_m_vz_summer = self.delta_moment.z();

        // Вторая точка для интерполяции: максимальная осадка
        // Площадь парусности судна для осадки по ЛГВЛ
        let a_v_summer = draught.clone().into_iter().zip(a_v_dmin.map(|v| v - delta_a_v_summer)).collect();
        // статические моменты площади парусности для осадки по ЛГВЛ
        let m_vx_summer = draught.clone().into_iter().zip(m_vx_dmin.map(|v| v - delta_m_vx_summer)).collect();
        let m_vz_summer = draught.clone().into_iter().zip(m_vz_dmin.map(|v| v - delta_m_vz_summer)).collect();

        let a_v = Curve::new_linear(&a_v_summer).value(self.drought_current);
        let m_vx = Curve::new_linear(&m_vx_summer).value(self.drought_current);
        let m_vz = Curve::new_linear(&m_vz_summer).value(self.drought_current);

        let x_v = m_vx/a_v;
        let z_v_bp = m_vz/a_v;

        let z_v = z_v_bp - self.volume_shift;

        z_v
    }
}
