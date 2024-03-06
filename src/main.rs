//!Расчет изгибающих моментов и срезывающих сил при общем изгибе корпуса судна на тихой воде.
//!Подробности расчетов приведены в [/design/Статище](./../../../design/Статище(3).docx)
//!Входные данные:
//!   - n: количество отрезков разбиения корпуса судна по х,
//!   - water_density: плотность воды,
//!   - ship_length: длинна корпуса судна,
//!   - center_waterline: кривая отстояния центра тяжести ватерлинии по длине от миделя,
//!   - rad_long: кривая продольного метацентрического радиуса,
//!   - mean_draught: кривая средней осадки,
//!   - center_shift: кривая отстояния центра величины погруженной части судна,
//!   - массив шпангоутов судна [(index, immersion_area)], где:
//!      - index: порядковый номер шпангоута,
//!      - immersion_area: кривая погружаемой площади,
//!   - массив данных по твердым грузам в составе:
//!      - mass: общая масса груза,
//!      - bound: границы груза,
//!      - center: центер масс;
//!   - массив данных по цистернам в составе:
//!      - density: плотность жидкости в цистерне,
//!      - volume: объем жидкости в цистерне,
//!      - bound: границы цистерны, (x1, x2, y1, y2),
//!      - center: кривая координат центра объема жидкости в цистерне
//!         в системе координат судна (volume, x, y, z),   
//!      - free_surf_inertia: кривая момента инерции площади свободной  
//!         поверхности жидкости (volume, x - поперечный, y - продольный).
//!   Выходные данные:
//!   - массив значений срезывающих сил,
//!   - массив значений изгибающих моментов.
//!
//!   Общее описание и порядок расчетов:
//!   1. Вычисляется общая масса судна путем суммирования всех нагрузок. Из общей массы по кривой водоизмещения с учетом плотности воды вычисляется объемное водоизмещение $\nabla = \Delta/\rho$.
//!   2. Исходя из объемного водоизмещения по таблицам элементов теоретического чертежа судна на ровный киль определяются:
//!      - отстояние центра величины погруженной части судна:
//!         - по длине от миделя $x_c$;
//!         - по ширине от ДП $y_c$;
//!         - по высоте от ОП $z_c$.
//!      - отстояние центра тяжести ватерлинии по длине от миделя $x_f$;
//!      - поперечный $r$ и продольный $R$ метацентрические радиусы, м;
//!      - среднюю осадку $d$;
//!   Для промежуточных значений определяется линейной интерполяцией. С учетом поправки на влияние свободной поверхности жидкости в цистернах вычисляется дифферент судна.
//!   3. Из дифферента и средней осадки вычисляется осадка носа и кормы. Из них методом линейной интерполяции вычисляется распределение осадки по каждой шпации.
//!   4. Вычисляется вытесненную массу воды для каждой шпации. Погруженная площадь $S_{start}, S_{end}$ теоретических шпангоутов берется из кривых. $L_{start}, L_{end}$ - расстояние от кормы до шпангоутов, ограничивающих шпацию. Вытесненная масса воды Buoyancy вычисляется как среднее значение погруженной площади умноженное на плотность воды $\gamma$ и на разницу расстояний до теоретических шпангоутов: $$V_i = (S_{start_i} + S_{end_i})/2*(L_{end_i}-L_{start_i})*\gamma$$
//!   5. Вычисляется результирующая сила TotalForce для каждой шпации как разницу веса вытесненной воды и массы приходящейся на каждую шпацию, умноженную на гравитационную постоянную g: $Ft_i = (m_i - V_i)*g$.
//!   6. Вычисляется срезающуя сила ShearForce для каждой шпации через интегрирование. Интегрирование проводим путем вычисления суммы сверху: $Fs_i = Fs_{i-1} + Ft_i, Fs_0 = 0$.
//!   7. Вычисляется изгибающий момент BendingMoment для каждой шпации как интегриральнуа сумма срезающей силы:
//!      $M_i = M_{i-1} + Fs_{i-1} + Fs_i, M_0 = 0$.

use std::{
    fmt::{self, Display},
    io, process,
    rc::Rc,
};

use api_tools::client::{
    api_query::{ApiQuery, ApiQueryKind, ApiQuerySql},
    api_request::ApiRequest,
};
use data::input_api_server::get_data;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use error::Error;
use log::*;
use testing::entities::test_value::Value;

use crate::{
    bending_moment::BendingMoment,
    bound::Bound,
    curve::Curve,
    displacement::Displacement,
    draught::Draught,
    frame::Frame,
    inertia_shift::InertiaShift,
    load::{ILoad, LoadSpace},
    mass::{IMass, Mass},
    math::*,
    pos_shift::PosShift,
    position::Position,
    shear_force::{IShearForce, ShearForce},
    tank::Tank,
    total_force::TotalForce,
    trim::Trim,
};

mod bending_moment;
mod data;
mod displacement;
mod draught;
mod error;
mod frame;
mod load;
mod mass;
mod math;
mod shear_force;
mod tank;
mod tests;
mod total_force;
mod trim;

fn main() -> Result<(), Error> {
    // data::input_api_server::create_test_db("test")?;
    // data::input_db::create_test_db("test");

    let data = get_data("test", 1)?;
    dbg!(&data);

    /*
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
    */

    /*    let data = read().unwrap_or_else(|err| {
             error!("Parsing arguments: {err}");
             process::exit(1);
         });
    */

    // длинна судна
    let ship_length = data.ship_length;
    let n = data.n_parts as usize;
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
    let water_density = data.water_density;
    // отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new(&data.center_waterline);
    // продольный метацентрический радиус
    let rad_long = Curve::new(&data.rad_long);
    // средняя осадка
    let mean_draught = Curve::new(&data.mean_draught);
    // отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new(&data.center_shift_x),
        Curve::new(&data.center_shift_y),
        Curve::new(&data.center_shift_z),
    );
    // шпангоуты
    let frames = data
        .frames
        .iter()
        .map(|v| {
            assert!(
                v.delta_x >= 0. && v.delta_x <= ship_length,
                "frame delta_x {} >= 0. && delta_x {} <= ship_length {}",
                v.delta_x,
                v.delta_x,
                ship_length
            );
            Frame::new(v.delta_x - ship_length / 2., Curve::new(&v.immersion_area))
        })
        .collect();
    // Грузы
    let mut loads: Vec<Rc<Box<dyn ILoad>>> = Vec::new();
    data.load_spaces.iter().for_each(|v| {
        loads.push(Rc::new(Box::new(LoadSpace::new(
            v.mass,
            Bound::new(v.bound.0, v.bound.1),
            Position::new(v.center.0, v.center.1, v.center.2),
        ))));
    });
    // Цистерны
    data.tanks.iter().for_each(|v| {
        loads.push(Rc::new(Box::new(Tank::new(
            v.density,
            v.volume,
            Bound::new(v.bound.0, v.bound.1),
            PosShift::new(
                Curve::new(&v.center_x),
                Curve::new(&v.center_y),
                Curve::new(&v.center_z),
            ),
            InertiaShift::new(
                Curve::new(&v.free_surf_inertia_x),
                Curve::new(&v.free_surf_inertia_y),
            ),
        ))));
    });

    let mass: Rc<dyn IMass> = Rc::new(Mass::new(loads, bounds.clone()));

    let shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        Draught::new(
            ship_length,
            water_density,
            bounds,
            Rc::clone(&mass),
            center_waterline_shift,
            mean_draught,
            Displacement::new(frames),
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
