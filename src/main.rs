#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use crate::{
    area::{HAreaStability, HAreaStrength, VerticalArea},
    data::structs::NavigationArea,
    icing::{IIcingStab, IcingMass, IcingStab},
    load::*,
    mass::*,
    math::*,
    stability::*,
    strength::*,
    windage::{IWindage, Windage},
};
use data::api_server::*;
pub use error::Error;
use log::info;
use std::{collections::HashMap, io, rc::Rc, time::Instant};

mod area;
mod data;
mod error;
mod icing;
mod load;
mod mass;
mod math;
mod stability;
mod strength;
mod tests;

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
  */      //   println!("{}", json_data);
    
    let host: String = "0.0.0.0".to_string();
    let port = "8080".to_string();    
 
    let ship_id = 1;
    let mut api_server =
        ApiServer::new("sss-computing".to_owned(), host.to_owned(), port.to_owned());

    let mut elapsed = HashMap::new();
    let time = Instant::now();

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
        .map(|v| {
            Frame::new(
                v.x,
                Curve::new_linear(&v.immersion_area),
            )
        })
        .collect();

    // вектор разбиения судна на отрезки
    let bounds = Rc::new(Bounds::from_frames(&data.bounds));

    let mut tanks: Vec<Rc<dyn ITank>> = Vec::new();
    let mut desks: Vec<Rc<dyn IDesk>> = Vec::new();
    let mut bulk: Vec<Rc<dyn IBulk>> = Vec::new();
    let mut load_variable: Vec<Rc<LoadMass>> = Vec::new();

    // Постоянная масса судна
    let mut loads_const: Vec<Rc<dyn ILoadMass>> = Vec::new();
    let shift_const = Position::new(
        data.const_mass_shift_x,
        data.const_mass_shift_y,
        data.const_mass_shift_z,
    );

    data.load_constants.iter().for_each(|v| {
        let bound_x = Bound::from(v.bound_x);
        let load = Rc::new(LoadMass::new(
            v.mass,
            bound_x,
            Some(shift_const.clone()),
            LoadingType::Lightship,
        ));
     //   log::info!("\t Mass loads_const from load_constants:{:?} ", load);
        loads_const.push(load);
    });

    data.cargoes.iter().for_each(|v| {
        let mass_shift = v
            .mass_shift
            .as_ref()
            .map(|mass_shift| Position::new(mass_shift.0, mass_shift.1, mass_shift.2));
        let bound_x = Bound::from(v.bound_x);
        let load = Rc::new(LoadMass::new(
            v.mass,
            bound_x,
            mass_shift.clone(),
            v.loading_type,
        ));
      //  log::info!("\t Mass load_variable from cargoes:{:?} ", load);
        load_variable.push(load);
    });

    data.compartments.iter().for_each(|v| {
        let mass_shift = v
            .mass_shift
            .as_ref()
            .map(|mass_shift| Position::new(mass_shift.0, mass_shift.1, mass_shift.2));
        let bound_x = Bound::from(v.bound_x);
        let load = Rc::new(LoadMass::new(
            v.mass,
            bound_x,
            mass_shift.clone(),
            v.loading_type,
        ));
       // log::info!("\t Mass load_variable from compartments src:{:?} trg:{:?}", v, load, );
        load_variable.push(load);
        if v.m_f_s_x.is_some() && v.m_f_s_y.is_some() && v.density.is_some() {
            let tank: Rc<dyn ITank> = Rc::new(Tank::new(
                v.density.unwrap_or(1.),
                v.volume.unwrap_or(0.),
                bound_x,
                mass_shift.clone(),
                InertiaMoment::new(v.m_f_s_x.unwrap_or(0.), v.m_f_s_y.unwrap_or(0.)),
                v.loading_type,
            ));
    //        log::info!("\t Mass tanks from compartments:{:?} ", tank);
            tanks.push(tank);
        }
    });

    let loads_const = Rc::new(loads_const);
    let desks = Rc::new(desks);
    let load_variable = Rc::new(load_variable);
    let bulk = Rc::new(bulk);

    let icing_area_h_str = data
        .area_h_str
        .iter()
        .map(|v| HAreaStrength::new(v.value, Bound::from(v.bound_x)))
        .collect();
    let icing_area_h_stab = data
        .area_h_stab
        .iter()
        .map(|v| HAreaStability::new(v.value, Position::new(v.shift_x, v.shift_y, v.shift_z)))
        .collect();
    let icing_area_v = data
        .area_v
        .iter()
        .map(|v| VerticalArea::new(v.value, v.shift_z, Bound::from(v.bound_x)))
        .collect::<Vec<_>>();

    let area_strength: Rc<dyn crate::strength::IArea> = Rc::new(crate::strength::Area::new(
        icing_area_v.clone(),
        icing_area_h_str,
        Rc::clone(&desks),
    ));
    let area_stability: Rc<dyn crate::stability::IArea> = Rc::new(crate::stability::Area::new(
        icing_area_v,
        icing_area_h_stab,
        Rc::clone(&desks),
    ));

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
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(
        loads_const,
        shift_const,
        Rc::new(IcingMass::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_strength),
            Rc::clone(&area_stability),
        )),
        Rc::clone(&load_variable),
        Rc::clone(&bounds),
        Rc::clone(&parameters),
    ));
    // Объемное водоизмещение (1)
    let volume = mass.sum() / data.water_density;
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
    // Средняя осадка
    let mean_draught = Curve::new_linear(&data.mean_draught).value(volume);
    parameters.add(ParameterID::DraughtMean, mean_draught);
    // Для расчета прочности дифферент находится подбором
    // как условие для схождения изгибающего момента в 0
    let mut computer = Computer::new(
        gravity_g,
        data.water_density,
        center_waterline_shift,
        mean_draught,
        Rc::clone(&mass),
        Rc::new(Displacement::new(frames)),
        Rc::clone(&bounds),
    );
    assert!(
        computer.shear_force().len() == data.bounds.len() + 1,
        "shear_force.len {} == frame.len {} + 1",
        computer.shear_force().len(),
        data.bounds.len()
    );
    assert!(
        computer.bending_moment().len() == data.bounds.len() + 1,
        "bending_moment.len {} == frame.len {} + 1",
        computer.bending_moment().len(),
        data.bounds.len()
    );

    /*    println!("shear_force:");
        computer.shear_force().iter().for_each(|v| println!("{v};"));
        println!("bending_moment:");
        computer
            .bending_moment()
            .iter()
            .for_each(|v| println!("{v};"));
    */
    send_strenght_data(
        &mut api_server,
        ship_id,
        &&computer.mass(),
        &computer.displacement(),
        &computer.total_force(),
        &computer.shear_force(),
        &computer.bending_moment(),
    )?;

    let flooding_angle = Curve::new_linear(&data.flooding_angle).value(mean_draught);

    let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
        center_draught_shift.clone(), // отстояние центра величины погруженной части судна
        rad_long,                     // продольный метацентрические радиус
        rad_trans,                    // поперечный метацентрические радиус
        tanks,
        Rc::clone(&mass), // все грузы судна
        Rc::clone(&parameters),
    ));

    let trim = Trim::new(
        data.length_lbp,
        mean_draught,
        center_waterline_shift,
        center_draught_shift.clone(),        
        Rc::clone(&metacentric_height),
        Rc::clone(&mass),
        Rc::clone(&parameters),
    )
    .value();

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

    let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
        Rc::clone(&mass),
        center_draught_shift.clone(),
        Curve2D::from_values_catmull_rom(data.pantocaren),
        // Curve2D::from_values_linear(data.pantocaren),
        mean_draught,
        Rc::clone(&metacentric_height),
        Rc::clone(&parameters),
    ));
    // dbg!(stability.k()?);
    //dbg!(lever_diagram.dso().len());
    /*   lever_diagram
            .dso()
            .iter()
            .for_each(|(k, v)| println!("{k} {v};"));
        lever_diagram
            .ddo()
            .iter()
            .for_each(|(k, v)| println!("{k} {v};"));
    */
    // Предполагаемое давление ветра +
    // Добавка на порывистость ветра
    let (p_v, m) = data
        .navigation_area_param
        .get_area(&data.navigation_area)
        .expect("main error no area data!");

    let windage: Rc<dyn IWindage> = Rc::new(Windage::new(
        Rc::clone(&icing_stab),
        Rc::clone(&area_stability),
        Curve::new_linear(&data.delta_windage_area).value(mean_draught),
        Moment::new(
            Curve::new_linear(&data.delta_windage_moment_x).value(mean_draught),
            0.,
            Curve::new_linear(&data.delta_windage_moment_z).value(mean_draught),
        ),
        volume_shift,
    ));

    let wind = Wind::new(p_v, m, Rc::clone(&windage), gravity_g, Rc::clone(&mass));

    let roll_period: Rc<dyn IRollingPeriod> = Rc::new(RollingPeriod::new(
        length_wl,
        data.width,
        mean_draught,
        Rc::clone(&metacentric_height),
    ));

    let roll_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(RollingAmplitude::new(
        data.keel_area,
        Rc::clone(&metacentric_height),
        volume,    // Объемное водоизмещение (1)
        length_wl, // длинна по ватерлинии при текущей осадке
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

    let stability = Stability::new(
        // Угол заливания отверстий
        flooding_angle,
        // Диаграмма плеч статической остойчивости
        Rc::clone(&lever_diagram),
        // Амплитуда качки судна с круглой скулой (2.1.5)
        Rc::clone(&roll_amplitude),
        // Расчет плеча кренящего момента от давления ветра
        Box::new(wind),
    );
    // dbg!(stability.k()?);
    /* // TODO: Давление ветра + добавка на порывистость ветра для
    // контейнеровоза/или судна перевозящего палубный груз контейнеров
    // неограниченного района плавания
    let (p_v, m) = data
        .navigation_area_param
        .get_area(&NavigationArea::Unlimited)
        .expect("main error no area data!");*/
    // Критерии остойчивости
    let mut criterion = Criterion::new(
        data.ship_type,
        data.navigation_area,
        desks.iter().any(|v| v.is_timber()),
        bulk.iter().any(|v| v.moment() != 0.),
        load_variable.iter().any(|v| v.value(None) != 0.),
        icing_stab.is_some(),
        flooding_angle,
        data.length_lbp,
        data.width,
        mean_draught,
        Curve::new_linear(&data.h_subdivision).value(mean_draught),
        Rc::new(Wind::new(
            p_v,
            m,
            Rc::clone(&windage),
            gravity_g,
            Rc::clone(&mass),
        )),
        Rc::clone(&lever_diagram),
        Rc::new(stability),
        Rc::clone(&metacentric_height),
        Rc::new(Acceleration::new(
            data.width,
            mean_draught,
            Rc::new(Curve::new_linear(&data.coefficient_k_theta.data())),
            Rc::clone(&roll_amplitude),
            Rc::clone(&metacentric_height),
            Rc::clone(&roll_period),
        )),
        Rc::new(Circulation::new(
            data.velocity,
            length_wl,
            mean_draught,
            Rc::clone(&mass),
            Rc::clone(&lever_diagram),
        )),
        Box::new(Grain::new(
            flooding_angle,
            Rc::clone(&bulk),
            Rc::clone(&mass),
            Rc::clone(&lever_diagram),
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
