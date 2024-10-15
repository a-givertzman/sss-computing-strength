#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{collections::HashMap, rc::Rc, time::Instant};

use crate::{
    area::{HAreaStability, HAreaStrength, VerticalArea},
    icing_stab::{IIcingStab, IcingStab},
    load::*,
    math::*,
    stability::*,
    strength::*,
    windage::Windage,
};
use args::get_args;
use data::api_server::*;
use draught::{Draught, IDraught};
pub use error::Error;
use icing_timber::IcingTimberBound;
use env_logger::Logger;
use log::info;
use trim::ITrim;

mod area;
mod args;
mod data;
mod draught;
mod error;
mod icing_stab;
mod icing_timber;
mod load;
mod math;
mod stability;
mod strength;
mod tests;
mod trim;

fn main() {
    let _log2 = log2::open("log.txt")
    .level(Logger::from_default_env().filter().as_str())
    .size(100*1024*1024)
    .rotate(20)
    .tee(false)
    .module(true)
    .start();
    info!("starting up");
    let reply = if let Err(error) = execute() {
        let str1 = r#"{"status":"failed","message":""#;
        let str2 = r#""}"#;
        format!("{str1}{}{str2}", error)
    } else {
        r#"{"status":"ok","message":null}"#.to_owned()
    };
    info!("reply: {reply}");    
    let _ = std::io::Write::write_all(&mut std::io::stdout().lock(), reply.as_bytes());
}

fn execute() -> Result<(), Error> {
    let (host, port) = get_args()?;
    let ship_id = 1;
    let mut api_server =
        ApiServer::new("sss-computing".to_owned(), host.to_owned(), port.to_owned());
    let mut elapsed = HashMap::new();
    let time = Instant::now();
    let results: Rc<dyn IResults> = Rc::new(Results::new());
    let parameters: Rc<dyn IParameters> = Rc::new(Parameters::new());
    let data = get_data(&mut api_server, ship_id)?;
    elapsed.insert("read data", time.elapsed());

    let time = Instant::now();
    // ускорение свободного падения
    let gravity_g = 9.81;
    // вектор разбиения судна на отрезки
    let bounds = Rc::new(Bounds::from_frames(&data.bounds)?);
    // коллекция различных типов грузов
    let mut loads = Loads::new(
        &data.load_constants,
        Position::new(
            data.const_mass_shift_x,
            data.const_mass_shift_y,
            data.const_mass_shift_z,
        ),
        &data.cargoes,
        &data.compartments,
    );
    // параметры обледенения поверхностей судна
    let icing_stab: Rc<dyn IIcingStab> = Rc::new(IcingStab::new(
        data.icing_stab,
        data.icing_m_timber,
        data.icing_m_v_full,
        data.icing_m_v_half,
        data.icing_m_h_full,
        data.icing_m_h_half,
        data.icing_coef_v_area_full,
        data.icing_coef_v_area_half,
        data.icing_coef_v_area_zero,
        data.icing_coef_v_moment_full,
        data.icing_coef_v_moment_half,
        data.icing_coef_v_moment_zero,
    ));
    // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    let mut area_const_v = Vec::new();
    for v in data.area_v_str.iter() {
        area_const_v.push(VerticalArea::new(
            v.value,
            Bound::new(v.bound_x1, v.bound_x2)?,
        ));
    }
    let mut area_const_h = Vec::new();
    for v in data.area_h_str.iter() {
        area_const_h.push(HAreaStrength::new(
            v.value,
            Bound::new(v.bound_x1, v.bound_x2)?,
        ));
    }
    let ship_mass: Rc<dyn strength::IMass> = Rc::new(strength::Mass::new(
        loads.loads_const()?,
        Rc::new(strength::IcingMass::new(
            Rc::clone(&icing_stab),
            Rc::new(crate::strength::Area::new(
                area_const_v,
                area_const_h,
                loads.desks()?,
                IcingTimberBound::new(data.width, data.length_loa, data.icing_timber_stab),
            )),
        )),
        Rc::new(WettingMass::new(data.wetting_timber, loads.load_timber()?)),
        loads.load_variable()?,
        Rc::clone(&bounds),
        Rc::clone(&results),
        Rc::clone(&parameters),
    ));
    // Объемное водоизмещение (1)
    let volume = ship_mass.sum()? / data.water_density;
    // Средняя осадка
    let mean_draught = Curve::new_linear(&data.mean_draught)?.value(volume)?;
    parameters.add(ParameterID::DraughtMean, mean_draught);
    // Момент площади горизонтальных поверхностей и площади парусности судна для расчета остойчивости
    let area_stability: Rc<dyn crate::stability::IArea> = Rc::new(crate::stability::Area::new(
        Curve::new_linear(&data.area_v_stab.area())?.value(data.draught_min)?,
        Curve::new_linear(&data.area_v_stab.moment_x())?.value(data.draught_min)?,
        Curve::new_linear(&data.area_v_stab.moment_z())?.value(data.draught_min)?,
        data.area_h_stab
            .iter()
            .map(|v| HAreaStability::new(v.value, Position::new(v.shift_x, v.shift_y, v.shift_z)))
            .collect(),
        loads.desks()?,
        IcingTimberBound::new(data.width, data.length_loa, data.icing_timber_stab),
    ));
    // Момент массы нагрузки на корпус судна
    let ship_moment: Rc<dyn stability::IShipMoment> = Rc::new(stability::ShipMoment::new(
        Rc::clone(&ship_mass),
        loads.loads_const()?,
        loads.shift_const(),
        Rc::new(stability::IcingMoment::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_stability),
        )),
        Rc::new(WettingMoment::new(
            data.wetting_timber,
            loads.load_timber()?,
        )),
        loads.load_variable()?,
        Rc::clone(&parameters),
    ));
    // Отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new_linear(&data.center_draught_shift_x)?,
        Curve::new_linear(&data.center_draught_shift_y)?,
        Curve::new_linear(&data.center_draught_shift_z)?,
    )
    .value(volume)?;
    parameters.add(ParameterID::CenterVolumeZ, center_draught_shift.z());
    // Продольный метацентрические радиус
    let rad_long = Curve::new_linear(&data.rad_long)?.value(volume)?;
    parameters.add(ParameterID::MetacentricLongRad, rad_long);
    // Поперечный метацентрические радиус
    let rad_trans = Curve::new_linear(&data.rad_trans)?.value(volume)?;
    parameters.add(ParameterID::MetacentricTransRad, rad_trans);
    // Отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new_linear(&data.center_waterline)?.value(volume)?;
    // Площадь ватерлинии
    let area_wl = Curve::new_linear(&data.waterline_area)?.value(volume)?;
    // Число тонн на 1 см осадки
    parameters.add(ParameterID::TonesPerCm, 0.01 * area_wl * data.water_density);
    // Для расчета прочности дифферент находится подбором
    // как условие для схождения изгибающего момента в 0

    Computer::new(
        gravity_g,
        data.water_density,
        data.length_lbp,
        center_waterline_shift,
        mean_draught,
        Rc::clone(&ship_mass),
        Rc::new(Displacement::new(data.frame_area)?),
        Rc::clone(&bounds),
        Rc::clone(&results),
    )
    .calculate()?;
    send_strenght_data(&mut api_server, ship_id, results.take_data())?;
    // Угол заливания отверстий
    let flooding_angle = Curve::new_linear(&data.flooding_angle)?.value(mean_draught)?;
    parameters.add(ParameterID::AngleOfDownFlooding, flooding_angle);
    // Угол входа в воду кромки палубы
    let entry_angle = Curve::new_linear(&data.entry_angle)?.value(mean_draught)?;
    parameters.add(ParameterID::OpenDeckEdgeImmersionAngle, entry_angle);
    // метацентрическая высота
    let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
        center_draught_shift.clone(), // отстояние центра величины погруженной части судна
        rad_long,                     // продольный метацентрические радиус
        rad_trans,                    // поперечный метацентрические радиус
        loads.tanks()?,
        Rc::clone(&ship_mass),
        Rc::clone(&ship_moment),
        Rc::clone(&parameters),
    ));
    // Момент кренящий на 1 градус MH1deg, т∙м
    parameters.add(
        ParameterID::MomentRollPerDeg,
        ship_mass.sum()? * metacentric_height.h_trans_fix()? * (std::f64::consts::PI / 180.).sin(),
    );
    // Длинна по ватерлинии при текущей осадке
    let length_wl = Curve::new_linear(&data.waterline_length)?.value(mean_draught)?;
    // Ширина по ватерлинии при текущей осадке
    let breadth_wl = Curve::new_linear(&data.waterline_breadth)?.value(mean_draught)?;
    // Отстояние по вертикали центра площади проекции подводной части корпуса
    let volume_shift = Curve::new_linear(&data.volume_shift)?.value(mean_draught)?;
    // Диаграмма плечей остойчивости
    let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
        Rc::clone(&ship_moment),
        center_draught_shift.clone(),
        data.pantocaren.clone(),
        mean_draught,
        Rc::clone(&metacentric_height),
        Rc::clone(&parameters),
    ));
    send_stability_diagram(&mut api_server, ship_id, lever_diagram.diagram()?)?;
    // влияние ветра на остойчивость
    let wind: Rc<dyn IWind> = Rc::new(Wind::new(
        data.navigation_area.clone(),
        Rc::new(Windage::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_stability),
            Curve::new_linear(&data.delta_windage_area)?.value(mean_draught)?,
            Moment::new(
                Curve::new_linear(&data.delta_windage_moment_x)?.value(mean_draught)?,
                0.,
                Curve::new_linear(&data.delta_windage_moment_z)?.value(mean_draught)?,
            ),
            volume_shift,
        )),
        gravity_g,
        Rc::clone(&ship_mass),
        Rc::clone(&parameters),
    ));
    // период качки судна
    let roll_period: Rc<dyn IRollingPeriod> = Rc::new(RollingPeriod::new(
        length_wl,
        data.width,
        mean_draught,
        Rc::clone(&metacentric_height),
    ));
    // амплитуда качки судна
    let coefficient_k: Rc<dyn ICurve> = Rc::new(Curve::new_linear(&data.coefficient_k.data())?);
    let multipler_x1: Rc<dyn ICurve> = Rc::new(Curve::new_linear(&data.multipler_x1.data())?);
    let multipler_x2: Rc<dyn ICurve> = Rc::new(Curve::new_linear(&data.multipler_x2.data())?);
    let multipler_s: Rc<dyn ICurve> = Rc::new(Curve::new_linear(
        &data.multipler_s.get_area(&data.navigation_area.area),
    )?);
    let coefficient_k_theta: Rc<dyn ICurve> =
        Rc::new(Curve::new_linear(&data.coefficient_k_theta.data())?);
    let roll_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(RollingAmplitude::new(
        data.keel_area,
        Rc::clone(&metacentric_height),
        volume,     // Объемное водоизмещение (1)
        length_wl,  // длинна по ватерлинии при текущей осадке
        data.width, // ширина полная
        breadth_wl, // ширина по ватерлинии при текущей осадке
        mean_draught,
        Rc::clone(&coefficient_k),
        Rc::clone(&multipler_x1),
        Rc::clone(&multipler_x2),
        Rc::clone(&multipler_s),
        Rc::clone(&roll_period),
    )?);

    // Критерии остойчивости
    let criterion_computer_results = CriterionComputer::new(
        data.overall_height,
        data.ship_type,
        Curve::new_linear(&data.h_subdivision)?.value(mean_draught)?,
        data.navigation_area.area,
        loads.desks()?.iter().any(|v| v.is_timber()),
        loads.bulks()?.iter().any(|v| v.moment() != 0.),
        !loads.load_variable()?.is_empty(),
        icing_stab.is_some(),
        flooding_angle,
        data.length_lbp,
        data.moulded_depth,
        mean_draught,
        volume,
        length_wl,
        data.width,
        breadth_wl,
        data.velocity,
        Rc::clone(&ship_moment),
        Rc::clone(&ship_mass),
        loads.bulks()?,
        Rc::clone(&coefficient_k),
        Rc::clone(&multipler_x1),
        Rc::clone(&multipler_x2),
        Rc::clone(&multipler_s),
        Rc::clone(&coefficient_k_theta),
        data.keel_area,
        rad_trans,
        center_draught_shift.clone(),
        data.pantocaren.clone(),
        Rc::clone(&wind),
        Rc::clone(&metacentric_height),
    )?
    .calculate()?;

    let mut criterion_result = CriterionStability::new(
        data.ship_type,
        data.navigation_area.area,
        data.width,
        data.moulded_depth,
        Curve::new_linear(&data.h_subdivision)?.value(mean_draught)?,
        loads.desks()?.iter().any(|v| v.is_timber()),
        loads.bulks()?.iter().any(|v| v.moment() != 0.),
        !loads.load_variable()?.is_empty(),
        icing_stab.is_some(),
        flooding_angle,
        data.length_lbp,
        Rc::clone(&wind),
        Rc::clone(&lever_diagram),
        Rc::new(Stability::new(
            flooding_angle,
            Rc::clone(&lever_diagram),
            Rc::clone(&roll_amplitude),
            Rc::clone(&wind),
            Rc::clone(&parameters),
        )),
        Rc::clone(&metacentric_height),
        Rc::new(Acceleration::new(
            data.width,
            mean_draught,
            Rc::new(Curve::new_linear(&data.coefficient_k_theta.data())?),
            Rc::clone(&roll_period),
            Rc::clone(&roll_amplitude),
            Rc::clone(&metacentric_height),
        )),
        Rc::new(Circulation::new(
            data.velocity,
            length_wl,
            mean_draught,
            Rc::clone(&ship_mass),
            Rc::clone(&ship_moment),
            Rc::clone(&lever_diagram),
            Rc::clone(&parameters),
        )?),
        Box::new(Grain::new(
            flooding_angle,
            loads.bulks()?,
            Rc::clone(&ship_mass),
            Rc::clone(&lever_diagram),
            Rc::clone(&parameters),
        )),
    )?
    .create();

    let trim: Rc<dyn ITrim> = Rc::new(stability::Trim::new(
        data.length_lbp,
        mean_draught,
        center_draught_shift.clone(),
        Rc::clone(&metacentric_height),
        Rc::clone(&ship_mass),
        Rc::clone(&ship_moment),
        Rc::clone(&parameters),
    )?);
    let draught: Rc<dyn IDraught> = Rc::new(Draught::new(
        data.length_lbp,
        center_waterline_shift,
        Rc::clone(&trim),
        Some(Rc::clone(&parameters)),
    )?);
    // Марки заглубления
    let draft_mark = DraftMark::new(Rc::clone(&draught), data.draft_mark, Rc::clone(&parameters));
    send_draft_mark(&mut api_server, ship_id, draft_mark.calculate()?)?;
    //
    let criterion_draught = CriterionDraught::new(
        data.ship_type,
        data.deadweight,
        data.freeboard_type,
        data.bow_area_min,
        data.aft_trim,
        data.forward_trim,
        data.bow_h_min,
        Rc::clone(&draught),
        Rc::clone(&trim),
        Rc::clone(&parameters),
        LoadLine::new(Rc::clone(&draught), data.load_line, Rc::clone(&parameters)),
        DepthAtForwardPerpendicular::new(
            Rc::clone(&draught),
            data.bow_board.clone(),
            Rc::clone(&parameters),
        ),
        Screw::new(Rc::clone(&draught), data.screw, Rc::clone(&parameters)),
        ReserveBuoyncyInBow::new(Rc::clone(&draught), data.length_lbp, data.bow_area.clone()),
        MinimumDraft::new(data.length_lbp),
    );
    criterion_result.append(&mut criterion_draught.create());
    send_stability_data(
        &mut api_server,
        ship_id,
        criterion_result,
        criterion_computer_results,
    )?;
    send_parameters_data(&mut api_server, ship_id, parameters.take_data())?; //
    elapsed.insert("calculate", time.elapsed());

    for (key, e) in elapsed {
        info!("{}:\t{:?}", key, e);
    }

    Ok(())
}
