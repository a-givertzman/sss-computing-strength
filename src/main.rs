#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{io, process, rc::Rc};
use aquamarine::aquamarine;
use api_tools::client::{
    api_query::{ApiQuery, ApiQueryKind, ApiQuerySql},
    api_request::ApiRequest,
};
use data::parse_input::ParsedInputData;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::*;
use testing::entities::test_value::Value;

use crate::{
    bending_moment::BendingMoment, data::parse_input::ParsedShipData, displacement::Displacement, draught::Draught, frame::Frame, load::ILoad, mass::{IMass, Mass}, math::{bound::Bound, curve::Curve, inertia_shift::inertia_shift::InertiaShift, pos_shift::PosShift}, shear_force::{IShearForce, ShearForce}, tank::Tank, total_force::TotalForce, trim::Trim
};

mod bending_moment;
mod data;
mod displacement;
mod draught;
mod frame;
mod load;
mod mass;
mod math;
mod shear_force;
mod tank;
mod tests;
mod total_force;
mod trim;

#[aquamarine]
/// ```mermaid
///    flowchart TB
///    %% создаем узел
///       A --> B 
///    %% Кстати да, комментарии задаются двумя знаками процента
///    %% Описываем действия для клика по узлу
///
///    click A "http://www.github.com"
/// ```
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    debug!("Test the debugging...");
    debug!("Test the testing...");
    let value = Value::Bool(false);
    debug!("\t bool value: {:?}", value);
    let value = Value::Int(444);
    debug!("\t int value: {:?}", value);
    let value = Value::Float(55.55);
    debug!("\t float value: {:?}", value);
    let value = Value::String("66.77".to_string());
    debug!("\t string value: {:?}", value);

/*    let data = read().unwrap_or_else(|err| {
        error!("Parsing arguments: {err}");
        process::exit(1);
    });

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new("database", "sql")),
        false,
    );
    let mut request = ApiRequest::new(
        "parent",
        "address",
        "auth_token",
        query.clone(),
        false,
        false,
    );
   // let reply = request.fetch(&query, false);

    let reply_data = request.fetch(&query, false).unwrap_or_else(|err| {
        error!("request.fetch: {err}");
        process::exit(1);
    });
    let string = String::from_utf8(reply_data).unwrap_or_else(|err| {
        error!("String::from_utf8: {err}");
        process::exit(1);
    });
    let ship_data = ParsedShipData::parse(&string.to_lowercase().trim().to_owned()).unwrap_or_else(|err| {
        error!("ParsedShipData::parse: {err}");
        process::exit(1);
    });

    // длинна судна
    let ship_length = 118.39;
    let n = 20;
    let delta_x = ship_length / n as f64;
    let start_x = -ship_length / 2.;
    // вектор разбиения судна на отрезки
    let bounds = (0..n as usize)
        .map(|v| {
            Bound::new(
                start_x + delta_x * v as f64,
                start_x + delta_x * (v as f64 + 1.),
            )
        })
        .collect::<Vec<_>>();
    // ускорение свободного падения
    let gravity_g = 9.81;
    // плотность окружающей воды
    let water_density = 1.025;
    // отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new(vec![(0., 0.), (10., 1.)]);
    // продольный метацентрический радиус
    let rad_long = Curve::new(vec![(0., 0.), (10., 1.)]);
    // средняя осадка
    let mean_draught = Curve::new(vec![(0., 0.), (1000., 1.), (10000., 10.)]);
    // отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new(vec![(0., 2.), (10., 2.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
    );
    //координаты центра объема жидкости в цистерне в системе координат судна
    let tank_center_draught_shift = PosShift::new(
        Curve::new(vec![(0., 2.), (10., 2.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
    );
    //момент инерции площади свободной поверхности жидкости
    let tank_free_surf_inertia = InertiaShift::new(
        Curve::new(vec![(0., 0.), (10., 1.)]),
        Curve::new(vec![(0., 0.), (10., 1.)]),
    );
    // все грузы судна
    let loads: Vec<Rc<Box<dyn ILoad>>> = vec![Rc::new(Box::new(Tank::new(
        2.,
        10.,
        Bound::new(-5., 5.),
        tank_center_draught_shift,
        tank_free_surf_inertia,
    )))];
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(loads, bounds.clone()));
    let frames = vec![
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
    ];

    let shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        Draught::new(
            ship_length,
            water_density,
            bounds,
            Rc::clone(&mass),
            center_waterline_shift,
            mean_draught,
            Displacement::new(frames, ship_length),
            Trim::new(
                water_density,
                ship_length,
                center_draught_shift, // отстояние центра величины погруженной части судна
                rad_long,             // продольный метацентрические радиус
                Rc::clone(&mass),     // все грузы судна
            ),
        ),
        gravity_g,
    ));
    let bending_moment = BendingMoment::new(&shear_force);
    dbg!(&shear_force.values(), &bending_moment.values());
    */


    
}

/// Чтение данных из стандартного потока ввода
pub fn read() -> Result<ParsedInputData, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(ParsedInputData::parse(
        &input.to_lowercase().trim().to_owned(),
    )?)
}

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
