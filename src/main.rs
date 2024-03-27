#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use crate::{
    bound::Bound,
    computer::Computer,
    curve::Curve,
    displacement::Displacement,
    frame::Frame,
    inertia::InertiaShift,
    load::{ILoad, LoadSpace},
    mass::{IMass, Mass},
    math::*,
    metacentric_height::{IMetacentricHeight, MetacentricHeight},
    pos_shift::PosShift,
    position::Position,
    rolling_amplitude::RollingAmplitude,
    rolling_period::RollingPeriod,
    stability_arm::StabilityArm,
    tank::Tank,
    wind::Wind,
};
use data::input_api_server::*;
use error::Error;
use futures::executor::block_on;
use log::info;
use std::{collections::HashMap, rc::Rc, time::Instant};

mod bending_moment;
mod computer;
mod data;
mod displacement;
mod draught;
mod error;
mod frame;
mod load;
mod mass;
mod math;
mod metacentric_height;
mod rolling_amplitude;
mod rolling_period;
mod shear_force;
mod stability_arm;
mod tank;
mod tests;
mod total_force;
mod trim;
mod wind;

fn main() -> Result<(), Error> {
    //    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");

    // data::input_api_server::create_test_db("test")?;
    // data::input_db::create_test_db("test");

    let mut elapsed = HashMap::new();

    let time = Instant::now();
    let data = get_data("sss-computing", 1)?;
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

    /*   (0..n as usize)
    .map(|v| {
        Bound::new(
            start_x + delta_x * v as f64,
            start_x + delta_x * (v as f64 + 1.),
        )
    })
    .collect::<Vec<_>>();*/

    // ускорение свободного падения
    let gravity_g = 9.81;
    // плотность окружающей воды
    //    let water_density = data.water_density;
    // отстояние центра тяжести ватерлинии по длине от миделя
    //    let center_waterline_shift = Curve::new(&data.center_waterline);
    // продольный метацентрический радиус
    //    let rad_long = Curve::new(&data.rad_long);
    // средняя осадка
    //    let mean_draught = Curve::new(&data.mean_draught);
    // отстояние центра величины погруженной части судна
    /*    let center_draught_shift = PosShift::new(
        Curve::new(&data.center_shift_x),
        Curve::new(&data.center_shift_y),
        Curve::new(&data.center_shift_z),
    );*/
    // шпангоуты
    let frames: Vec<Frame> = data
        .frames
        .iter()
        .map(|v| {
            Frame::new(
                v.x - data.frames.last().expect("frames last error: no frame").x / 2.,
                Curve::new_linear(&v.immersion_area),
            )
        })
        .collect();

    // длинна судна99
    let ship_length = frames.last().unwrap().shift_x() - frames.first().unwrap().shift_x();
    //let ship_length = 120.0<<<<<<< MetacentricHeight
    let n_parts = 20; //data.n_parts as usize;
                      // вектор разбиения судна на отрезки
    let bounds = Rc::new(Bounds::from_n(ship_length, n_parts));

    // Постоянная масса судна
    let mut loads_const: Vec<Rc<Box<dyn ILoad>>> = Vec::new();
    let const_shift = Position::new(
        data.const_mass_shift_x,
        data.const_mass_shift_y,
        data.const_mass_shift_z,
    );
    for index in 0..frames.len() - 1 {
        let bound = Bound::new(frames[index].shift_x(), frames[index + 1].shift_x());
        if let Some(mass) = data.load_constant.data().get(&index) {
            loads_const.push(Rc::new(Box::new(LoadSpace::new(
                *mass,
                bound,
                Position::new(bound.center(), const_shift.y(), const_shift.z()),
                0.,
                0.,
            ))));
        }
    }
    // Грузы судна
    let mut loads_cargo: Vec<Rc<Box<dyn ILoad>>> = Vec::new();
    data.load_spaces.iter().for_each(|v| {
        if v.mass != 0. {
            loads_cargo.push(Rc::new(Box::new(LoadSpace::new(
                v.mass,
                Bound::new(v.bound.0, v.bound.1),
                Position::new(v.center.0, v.center.1, v.center.2),
                v.m_f_s_y,
                v.m_f_s_x,
            ))));
        }
    });

    // Цистерны
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

    // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(
        loads_const,
        const_shift,
        loads_cargo,
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
    computer.shear_force().len();
    /*    println!("shear_force:");
        computer.shear_force().iter().for_each(|v| { println!("{v};") });
        println!("bending_moment:");
        computer.bending_moment().iter().for_each(|v| { println!("{v};") });
    */


    let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
        center_draught_shift.clone(), // отстояние центра величины погруженной части судна
        rad_long,                     // продольный метацентрические радиус
        rad_cross,                    // поперечный метацентрические радиус
        Rc::clone(&mass),             // все грузы судна
    ));

    let mut stability_arm = StabilityArm::new(
        Curve2D::from_values_catmull_rom(data.pantocaren),
        mean_draught,
        Rc::clone(&metacentric_height),
    );
    dbg!(stability_arm.angle_static_roll());
/*    stability_arm
        .diagram()
        .iter()
        .for_each(|(k, v)| println!("{k} {v};"));
*/
    // Предполагаемое давление ветра +
    // Добавка на порывистость ветра
    let (p_v, m) = data
        .navigation_area_param
        .get_area(&data.navigation_area_name)
        .expect("main error no area data!");

    let mut wind = Wind::new(
        p_v,
        m,
        data.windage,
        data.windage_shift_z,
        gravity_g,
        Rc::clone(&mass),
    );

    // Коэффициент полноты судна
    let c_b = 0.9; //TODO: нужна формула расчета, должен дать регистр

    let roll_amplitude = RollingAmplitude::new(
        c_b, // Коэффициент полноты судна
        data.keel_area,
        Rc::clone(&mass),
        data.length,  //TODO нужна длинна по ватерлинии при текущей осадке
        data.breadth,
        mean_draught,
        Curve::new_linear(&data.coefficient_k.data()),
        Curve::new_linear(&data.multipler_x1.data()),
        Curve::new_linear(&data.multipler_x2.data()),
        Curve::new_linear(&data.multipler_s.get_area(&data.navigation_area_name)),
        RollingPeriod::new(
            data.breadth,
            mean_draught,
            data.length,  //TODO нужна длинна по ватерлинии при текущей осадке
            Rc::clone(&metacentric_height),
        ),
    );

    dbg!(wind.arm_wind_dynamic(), roll_amplitude.calculate());

    elapsed.insert("Completed", time.elapsed());
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
