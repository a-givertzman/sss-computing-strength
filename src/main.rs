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
use std::{collections::HashMap, rc::Rc, time::Instant};

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

fn main() -> Result<(), Error> {
    //    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");

    // data::input_api_server::create_test_db("test")?;
    // data::input_db::create_test_db("test");

    let mut elapsed = HashMap::new();

    let time = Instant::now();
    let ship_id = 1;
    let mut data = get_data("sss-computing", ship_id)?;
    elapsed.insert("ParsedShipData sync", time.elapsed());

    /*   let time = Instant::now();
        let data = async_get_data("test_ship", 1);
        let data = block_on(data)?;
        elapsed.insert("ParsedShipData async", time.elapsed());
    */
    //  dbg!(&data.pantocaren);

    let time = Instant::now();
    //   dbg!(&data);

    /*
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        info!("Test the debugging...");
        info!("Test the testing...");
        let value = Value::Bool(false);
        info!("\t bool value: {:?}", value);
        let value = Value::Int(444);
        info!("\t int value: {:?}", value);
        let value = Value::Float(55.55);
        info!("\t float value: {:?}", value);
        let value = Value::String("66.77".to_string());
        info!("\t string value: {:?}", value);
    */

    /*    let data = read().unwrap_or_else(|err| {
             error!("Parsing arguments: {err}");
             process::exit(1);
         });
    */

    // ускорение свободного падения
    let gravity_g = 9.81;

    // шпангоуты
    let frames: Vec<Frame> = data
        .frame_area
        .iter()
        .map(|v| {
            Frame::new(
                v.x - data
                    .frame_area
                    .last()
                    .expect("frames last error: no frame")
                    .x
                    / 2.,
                Curve::new_linear(&v.immersion_area),
            )
        })
        .collect();

    // вектор разбиения судна на отрезки
    let bounds = Rc::new(Bounds::from_frames(&data.bounds));

    let mut tanks: Vec<Rc<dyn ITank>> = Vec::new();
    let mut desks: Vec<Rc<dyn IDesk>> = Vec::new();
    let mut bulk: Vec<Rc<dyn IBulk>> = Vec::new();
    let mut load_mass: Vec<Rc<dyn ILoadMass>> = Vec::new();

    // Постоянная масса судна
    let mut loads_const: Vec<Rc<dyn ILoadMass>> = Vec::new();
    let const_shift = Position::new(
        data.const_mass_shift_x,
        data.const_mass_shift_y,
        data.const_mass_shift_z,
    );

    data.cargoes.iter().for_each(|v| {
        let mass_shift = if let Some(mass_shift) = v.mass_shift.as_ref().clone() { 
            Some(Position::new(mass_shift.0, mass_shift.1, mass_shift.2))
        } else {
            None
        };
        let bound_x = Bound::from(v.bound_x);

        let load = Rc::new(LoadMass::new(
            v.mass,
            bound_x,
            mass_shift.clone(),
        ));
        log::info!("\t Mass loads_const:{:?} ", load);
        loads_const.push(load);
    });

    data.compartments.iter().for_each(|v| {
        let mass_shift = if let Some(mass_shift) = v.mass_shift.as_ref().clone() { 
            Some(Position::new(mass_shift.0, mass_shift.1, mass_shift.2))
        } else {
            None
        };
        let bound_x = Bound::from(v.bound_x);

        let load = Rc::new(LoadMass::new(
            v.mass,
            bound_x,
            mass_shift.clone(),
        ));
        load_mass.push(load);

        if v.m_f_s_x.is_some() && v.m_f_s_y.is_some() && v.density.is_some() {        
            let tank: Rc<dyn ITank> = Rc::new(Tank::new(
                v.density.unwrap_or(1.),
                v.volume.unwrap_or(0.),
                bound_x,
                mass_shift.clone(),
                InertiaMoment::new(
                    v.m_f_s_x.unwrap_or(0.),
                    v.m_f_s_y.unwrap_or(0.),
                ),
            ));
            tanks.push(tank);
        }
    });

    let loads_const = Rc::new(loads_const);
    let desks = Rc::new(desks);
    let load_mass = Rc::new(load_mass);
    let bulk = Rc::new(bulk);

    /*  // Цистерны
        data.tanks.iter().for_each(|v| {
            loads_cargo.push(Rc::new(Box::new(Tank::new(
                v.density,
                v.volume,
                Bound::new(v.bound.0, v.bound.1),
                PosShift::new(
                    Curve::new_linear(&v.center_x),
                    Curve::new_linear(&v.center_y),
                    Curve::new_linear(&v.center_z),
                ),
                InertiaShift::new(
                    Curve::new_linear(&v.free_surf_inertia_x),
                    Curve::new_linear(&v.free_surf_inertia_y),
                ),
            ))));
        });
    */
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
        const_shift,
        Rc::new(IcingMass::new(
            Rc::clone(&icing_stab),
            Rc::clone(&area_strength),
            Rc::clone(&area_stability),
        )),
        Rc::clone(&load_mass),
        Rc::clone(&bounds),
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
    // Продольный метацентрические радиус
    let rad_long = Curve::new_linear(&data.rad_long).value(volume);
    // Поперечный метацентрические радиус
    let rad_cross = Curve::new_linear(&data.rad_cross).value(volume);
    // Отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new_linear(&data.center_waterline).value(volume);
    // Средняя осадка
    let mean_draught = Curve::new_linear(&data.mean_draught).value(volume);

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

    println!("shear_force:");
    computer.shear_force().iter().for_each(|v| println!("{v};"));
    println!("bending_moment:");
    computer
        .bending_moment()
        .iter()
        .for_each(|v| println!("{v};"));

    send_strenght_data(
        "sss-computing",
        ship_id,
        &computer.shear_force(),
        &computer.bending_moment(),
    )?;

    let flooding_angle = Curve::new_linear(&data.flooding_angle).value(mean_draught);

    let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
        center_draught_shift.clone(), // отстояние центра величины погруженной части судна
        rad_long,                     // продольный метацентрические радиус
        rad_cross,                    // поперечный метацентрические радиус
        tanks,
        Rc::clone(&mass), // все грузы судна
    ));

    // Длинна по ватерлинии при текущей осадке
    let length_wl = Curve::new_linear(&data.waterline_length).value(mean_draught);
    // Ширина по ватерлинии при текущей осадке
    let breadth = Curve::new_linear(&data.waterline_breadth).value(mean_draught);
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
        breadth,
        mean_draught,
        Rc::clone(&metacentric_height),
    ));

    let roll_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(RollingAmplitude::new(
        data.keel_area,
        Rc::clone(&metacentric_height),
        volume,    // Объемное водоизмещение (1)
        length_wl, // длинна по ватерлинии при текущей осадке
        breadth,
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
        desks.iter().find(|v| v.is_timber()).is_some(),
        bulk.iter().find(|v| v.moment() != 0.).is_some(),
        load_mass.iter().find(|v| v.value(None) != 0.).is_some(),
        icing_stab.is_some(),
        flooding_angle,
        data.length,
        breadth,
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
            breadth,
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
    send_stability_data("sss-computing", ship_id, criterion.create());// TODO errors
    elapsed.insert("Write stability result", time.elapsed());

    for (key, e) in elapsed {
        println!("{}:\t{:?}", key, e);
    }
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
