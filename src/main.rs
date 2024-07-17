#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use crate::{
    area::{HAreaStability, HAreaStrength, VerticalArea},
    icing_stab::{IIcingStab, IcingStab},
    load::*,
    math::*,
    stability::*,
    strength::*,
    windage::Windage,
};
use data::{api_server::*, structs::loads::MatterType};
use draught::Draught;
pub use error::Error;
use icing_timber::{IcingTimberBound, IcingTimberType};
use log::info;
use std::{collections::HashMap, io, rc::Rc, time::Instant};

mod area;
mod data;
mod error;
mod icing_stab;
mod icing_timber;
mod load;
mod draught;
mod math;
mod stability;
mod strength;
mod tests;
mod trim;

fn main() {
    //    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");

    let reply = if let Err(error) = execute() {
        let str1 = r#"{"status":"failed","message":""#;
        let str2 = r#""}"#;
        format!("{str1}{}{str2}", error)
    } else {
        r#"{"status":"ok","message":null}"#.to_owned()
    };
    //    let json_data: serde_json::Value = serde_json::from_str(&reply).unwrap();
    let _ = io::Write::write_all(&mut io::stdout().lock(), reply.as_bytes());
}

fn execute() -> Result<(), Error> {
    /*      let mut input = String::new();
          io::stdin().read_line(&mut input)?;
          let json_data: serde_json::Value = serde_json::from_str(&input)?;
          let host: String = json_data
              .get("api-host")
              .ok_or(Error::FromString(
                  "Parse param error: no api-host".to_owned(),
              ))?
              .to_string();
          let port = json_data
              .get("api-port")
              .ok_or(Error::FromString(
                  "Parse param error: no api-host".to_owned(),
              ))?
              .to_string();
    */
 //   println!("{}", json_data);

    let host: String = "0.0.0.0".to_string();
    let port = "8080".to_string();

    let ship_id = 1;
    let mut api_server =
        ApiServer::new("sss-computing".to_owned(), host.to_owned(), port.to_owned());

    let mut elapsed = HashMap::new();
    let time = Instant::now();

    let results: Rc<dyn IResults> = Rc::new(Results::new());
    let parameters: Rc<dyn IParameters> = Rc::new(Parameters::new());
    let mut data = get_data(&mut api_server, ship_id)?;
    elapsed.insert("ParsedShipData sync", time.elapsed());

    /*   let time = Instant::now();
        let data = async_get_data("test_ship", 1);
        let data = block_on(data)?;
        elapsed.insert("ParsedShipData async", time.elapsed());
    */
    //  dbg!(&data.pantocaren);

    let time = Instant::now();
    //   dbg!(&data);
    
    // ускорение свободного падения
    let gravity_g = 9.81;
    // шпангоуты
    let frames: Vec<Frame> = data
        .frame_area
        .iter()
        .map(|v| Frame::new(v.x, Curve::new_linear(&v.immersion_area)))
        .collect();
    // вектор разбиения судна на отрезки
    let bounds = Rc::new(Bounds::from_frames(&data.bounds));
    // коллекция различных типов грузов
    let mut loads = Loads::new(
        &data.load_constants,
        Position::new(
            data.const_mass_shift_x,
            data.const_mass_shift_y,
            data.const_mass_shift_z,
        ),
        &data.cargoes, 
        &data.compartments
    );
    // площади поверхностей для расчета прочности
    let area_strength: Rc<dyn crate::strength::IArea> = Rc::new(crate::strength::Area::new(
        data
            .area_v_str
            .iter()
            .map(|v| VerticalArea::new(v.value, v.shift_z, Bound::new(v.bound_x1, v.bound_x2)))
            .collect::<Vec<_>>(),
        data
            .area_h_str
            .iter()
            .map(|v| HAreaStrength::new(v.value, Bound::new(v.bound_x1, v.bound_x2)))
            .collect(),
        loads.desks(),
        IcingTimberBound::new(
            data.width,
            data.length_loa,
            IcingTimberType::from(data.icing_timber_stab.clone()),
        ),
    ));
    // параметры обледенения поверхностей судна
    let icing_stab: Rc<dyn IIcingStab> = Rc::new(IcingStab::new(
        data.icing_stab.clone(),
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
    let ship_mass: Rc<dyn strength::IMass> = Rc::new(strength::Mass::new(
        loads.loads_const(),
        Rc::new(strength::IcingMass::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_strength),
        )),
        Rc::new(WettingMass::new(
            data.wetting_timber,
            loads.load_timber(),
        )),
        loads.load_variable(),
        Rc::clone(&bounds),
        Rc::clone(&results),
        Rc::clone(&parameters),
    ));
    // Объемное водоизмещение (1)
    let volume = ship_mass.sum() / data.water_density;
    // Средняя осадка
    let mean_draught = Curve::new_linear(&data.mean_draught).value(volume);
    parameters.add(ParameterID::DraughtMean, mean_draught);
    // Момент площади горизонтальных поверхностей и площади парусности судна для расчета остойчивости
    let area_stability: Rc<dyn crate::stability::IArea> = Rc::new(crate::stability::Area::new(
        Curve::new_linear(&data.area_v_stab.area()).value(mean_draught),
        Curve::new_linear(&data.area_v_stab.moment_x()).value(mean_draught),
        Curve::new_linear(&data.area_v_stab.moment_z()).value(mean_draught),
        data
            .area_h_stab
            .iter()
            .map(|v| HAreaStability::new(v.value, Position::new(v.shift_x, v.shift_y, v.shift_z)))
            .collect(),
            loads.desks(),
        IcingTimberBound::new(
            data.width,
            data.length_loa,
            IcingTimberType::from(data.icing_timber_stab.clone()),
        ),
    ));
    // Момент массы нагрузки на корпус судна
    let ship_moment: Rc<dyn stability::IShipMoment> = Rc::new(stability::ShipMoment::new(
        Rc::clone(&ship_mass),
        loads.loads_const(),
        loads.shift_const(),
        Rc::new(stability::IcingMoment::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_stability),
        )),
        Rc::new(WettingMoment::new(
            data.wetting_timber,
            loads.load_timber(),
        )),
        loads.load_variable(),
        Rc::clone(&results),
        Rc::clone(&parameters),
    ));
    // Отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new_linear(&data.center_draught_shift_x),
        Curve::new_linear(&data.center_draught_shift_y),
        Curve::new_linear(&data.center_draught_shift_z),
    )
    .value(volume);
    parameters.add(ParameterID::CenterVolumeZ, center_draught_shift.z());
    // Продольный метацентрические радиус
    let rad_long = Curve::new_linear(&data.rad_long).value(volume);
    parameters.add(ParameterID::MetacentricLongRad, rad_long);
    // Поперечный метацентрические радиус
    let rad_trans = Curve::new_linear(&data.rad_trans).value(volume);
    parameters.add(ParameterID::MetacentricTransRad, rad_trans);
    // Отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new_linear(&data.center_waterline).value(volume);
    // Площадь ватерлинии
    let area_wl = Curve::new_linear(&data.waterline_area).value(volume);
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
        Rc::new(Displacement::new(frames)),
        Rc::clone(&bounds),
        Rc::clone(&results),
    )
    .calculate();
    send_strenght_data(&mut api_server, ship_id, results.take_data())?;
    // Угол заливания отверстий
    let flooding_angle = Curve::new_linear(&data.flooding_angle).value(mean_draught);
    parameters.add(ParameterID::AngleOfDownFlooding, flooding_angle);
    // Угол входа в воду кромки палубы
    let entry_angle = Curve::new_linear(&data.entry_angle).value(mean_draught);
    parameters.add(ParameterID::OpenDeckEdgeImmersionAngle, entry_angle);
    // метацентрическая высота
    let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
        center_draught_shift.clone(), // отстояние центра величины погруженной части судна
        rad_long,                     // продольный метацентрические радиус
        rad_trans,                    // поперечный метацентрические радиус
        loads.tanks(),
        Rc::clone(&ship_mass),
        Rc::clone(&ship_moment),
        Rc::clone(&parameters),
    ));
    // Момент кренящий на 1 градус MH1deg, т∙м
    parameters.add(
        ParameterID::MomentRollPerDeg,
        ship_mass.sum() * metacentric_height.h_trans_fix() * (std::f64::consts::PI / 180.).sin(),
    );
    // Длинна по ватерлинии при текущей осадке
    let length_wl = Curve::new_linear(&data.waterline_length).value(mean_draught);
    // Ширина по ватерлинии при текущей осадке
    let breadth_wl = Curve::new_linear(&data.waterline_breadth).value(mean_draught);
    // Отстояние по вертикали центра площади проекции подводной части корпуса
    let volume_shift = Curve::new_linear(&data.volume_shift).value(mean_draught);
    // Проверяем есть ли пантокарены в отрицательной области углов
    // Если нет то считаем что судно симметорично и зеркально
    // копируем данные в отрицательную область углов крена
    let mut tmp = data
        .pantocaren
        .first()
        .expect("Main pantocaren error: no data!")
        .1
        .clone();
    tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    if tmp.first().expect("Main pantocaren error: no data!").0 <= 0. {
        data.pantocaren.iter_mut().for_each(|(_, vector)| {
            let mut negative = vector
                .iter()
                .filter(|(angle, _)| *angle > 0.)
                .map(|(angle, moment)| (-angle, -moment))
                .collect();
            vector.append(&mut negative);
            vector.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        });
    }
    // Диаграмма плечей остойчивости
    let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
        Rc::clone(&ship_moment),
        center_draught_shift.clone(),
        Curve2D::from_values_catmull_rom(data.pantocaren),
        // Curve2D::from_values_linear(data.pantocaren),
        mean_draught,
        Rc::clone(&metacentric_height),
        Rc::clone(&parameters),
    ));
    send_stability_diagram(&mut api_server, ship_id, lever_diagram.diagram())?;
    // Марки заглубления
    let mut draft_mark = DraftMark::new(
        Box::new(Draught::new(
            data.length_lbp,
            center_waterline_shift,
            // Дифферент для остойчивости
            Box::new(stability::Trim::new(
                data.length_lbp,
                mean_draught,
                center_draught_shift.clone(),
                Rc::clone(&metacentric_height),
                Rc::clone(&ship_mass),
                Rc::clone(&ship_moment),
                Rc::clone(&parameters),
            )),
            Some(Rc::clone(&parameters)),
        )),
        data.draft_mark.data(),
        Rc::clone(&parameters),
    );
    send_draft_mark(&mut api_server, ship_id, draft_mark.calculate())?;
    // Предполагаемое давление ветра +
    // Добавка на порывистость ветра
    let (p_v, m) = data
        .navigation_area_param
        .get_area(&data.navigation_area)
        .expect("main error no area data!");
    // влияние ветра на остойчивость
    let wind: Rc<dyn IWind> = Rc::new(Wind::new(
        p_v,
        m,
        Rc::new(Windage::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_stability),
            Curve::new_linear(&data.delta_windage_area).value(mean_draught),
            Moment::new(
                Curve::new_linear(&data.delta_windage_moment_x).value(mean_draught),
                0.,
                Curve::new_linear(&data.delta_windage_moment_z).value(mean_draught),
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
    let roll_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(RollingAmplitude::new(
        data.keel_area,
        Rc::clone(&metacentric_height),
        volume,     // Объемное водоизмещение (1)
        length_wl,  // длинна по ватерлинии при текущей осадке
        data.width, // ширина полная
        breadth_wl, // ширина по ватерлинии при текущей осадке
        mean_draught,
        Curve::new_linear(&data.coefficient_k.data()),
        Curve::new_linear(&data.multipler_x1.data()),
        Curve::new_linear(&data.multipler_x2.data()),
        Curve::new_linear(&data.multipler_s.get_area(&data.navigation_area)),
        Rc::clone(&roll_period),
    ));
    //dbg!(wind.arm_wind_dynamic(), roll_amplitude.calculate());
    // остойчивость
    let stability = Stability::new(
        // Угол заливания отверстий
        flooding_angle,
        // Диаграмма плеч статической остойчивости
        Rc::clone(&lever_diagram),
        // Амплитуда качки судна с круглой скулой (2.1.5)
        Rc::clone(&roll_amplitude),
        // Расчет плеча кренящего момента от давления ветра
        Rc::clone(&wind),
        Rc::clone(&parameters),
    );
    // dbg!(stability.k()?);
    // Критерии остойчивости
    let mut criterion = Criterion::new(
        data.ship_type,
        data.navigation_area,
        loads.desks().iter().any(|v| v.is_timber()),
        loads.bulks().iter().any(|v| v.moment() != 0.),
        loads.load_variable().iter().any(|v| v.value(None) != 0.),
        icing_stab.is_some(),
        flooding_angle,
        data.length_lbp,
        data.width,
        data.moulded_depth,
        Curve::new_linear(&data.h_subdivision).value(mean_draught),
        Rc::clone(&wind),
        Rc::clone(&lever_diagram),
        Rc::new(stability),
        Rc::clone(&metacentric_height),
        Rc::new(Acceleration::new(
            data.width,
            mean_draught,
            Rc::new(Curve::new_linear(&data.coefficient_k_theta.data())),
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
        )),
        Box::new(Grain::new(
            flooding_angle,
            loads.bulks(),
            Rc::clone(&ship_mass),
            Rc::clone(&lever_diagram),
            Rc::clone(&parameters),
        )),
    );

    elapsed.insert("Completed", time.elapsed());

    let time = Instant::now();
    // criterion.create().iter().for_each(|v| println!("{v}"));
    send_stability_data(&mut api_server, ship_id, criterion.create())?; //
    elapsed.insert("Write stability result", time.elapsed());
    send_parameters_data(&mut api_server, ship_id, parameters.take_data())?; //

    /*   for (key, e) in elapsed {
        println!("{}:\t{:?}", key, e);
    }*/
    Ok(())
}

/*
/// Чтение данных из стандартного потока ввода
pub fn read() -> Result<ParsedInputData, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(ParsedInputData::parse(
        &input.to_lowercase().trim().to_owned(),
    )?)
}
*/
/*
/// Writes a given value to the writer, serializing it into JSON.
pub fn write<W: Write, T: serde::Serialize>(mut writer: W, t: &T) -> Result<(), WriteError> {
    // We use to_string here instead of to_vec because it verifies that the JSON is valid UTF-8,
    // which is required by the JSON Lines specification (https://jsonlines.org).
    let json = serde_json::to_string(t).map_err(WriteError::Serialize)?;

    writer.write_all(json.as_bytes()).map_err(WriteError::Io)?;
    writer.write_all(b"\n").map_err(WriteError::Io)?;

    Ok(())
}*/
