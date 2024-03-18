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

use data::input_api_server::*;
use error::Error;
use futures::executor::block_on;
use log::info;
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};
use crate::{
    bending_moment::BendingMoment, bound::Bound, curve::Curve, displacement::Displacement, draught::Draught, frame::Frame, inertia_shift::InertiaShift, load::{ILoad, LoadSpace}, mass::{IMass, Mass}, math::*, metacentric_height::{IMetacentricHeight, MetacentricHeight}, pos_shift::PosShift, position::Position, shear_force::{IShearForce, ShearForce}, stability_arm::StabilityArm, tank::Tank, total_force::TotalForce, trim::Trim
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
mod metacentric_height;
mod stability_arm;

fn main() -> Result<(), Error> {
//    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");

    // data::input_api_server::create_test_db("test")?;
    // data::input_db::create_test_db("test");

    let mut elapsed = HashMap::new();

    let time = Instant::now();
    let _data = get_data("test_ship", 1)?;
    elapsed.insert("ParsedShipData sync", time.elapsed());

    let time = Instant::now();
    let data = async_get_data("test_ship", 1);
    let data = block_on(data)?;
    elapsed.insert("ParsedShipData async", time.elapsed());

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
                Curve::new(&v.immersion_area),
            )
        })
        .collect();

    // длинна судна99
    let ship_length = frames.last().unwrap().shift_x() - frames.first().unwrap().shift_x();
    //let ship_length = 120.0;
    let n_parts = 20;//data.n_parts as usize;
    // вектор разбиения судна на отрезки    
    // let bounds = Bounds::from_n(data.ship_length, n_parts);
    let bounds = Rc::new(Bounds::from_n(ship_length, n_parts));
    //  let half_length = bounds.length()/2.;

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

    // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(
        loads_const,
        const_shift,
        loads_cargo,
        Rc::clone(&bounds),
    ));
    // Суммарная масса судна и грузов
    let mass_sum = mass.sum(); 
    // Объемное водоизмещение (1)
    let volume = mass_sum / data.water_density;
    // Отстояние центра величины погруженной части судна
    let center_draught_shift = PosShift::new(
        Curve::new(&data.center_draught_shift_x),
        Curve::new(&data.center_draught_shift_y),
        Curve::new(&data.center_draught_shift_z),
    )
    .value(volume);
    // Продольный метацентрические радиус
    let rad_long = Curve::new(&data.rad_long).value(volume);
    // Поперечный метацентрические радиус
    let rad_cross = Curve::new(&data.rad_cross).value(volume);
    //отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline_shift = Curve::new(&data.center_waterline).value(volume);
    // Средняя осадка
    let mean_draught = Curve::new(&data.mean_draught).value(volume);
    // Распределение осадки
    let displacement = Rc::new(Displacement::new(frames));
/*
    let shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        data.water_density,
        Draught::new(
            center_waterline_shift,
            mean_draught,
            Rc::clone(&displacement),
            Trim::new(
                bounds.length(),
                center_draught_shift, // отстояние центра величины погруженной части судна
                Rc::clone(&metacentric_height),
                Rc::clone(&mass),           // все грузы судна
            ).value(),
            bounds,
        ),
        gravity_g,
    ));
*/
    
    // Для расчета прочности дифферент находится подбором 
    // как условие для схождения изгибающего момента в 0
    let mut trim = 0.;
    let mut delta = 1.;    
    for i in 0..20 {
        let value = *BendingMoment::new(Box::new(ShearForce::new(TotalForce::new(
            Rc::clone(&mass),
            data.water_density,
            Draught::new(
                center_waterline_shift,
                mean_draught,
                Rc::clone(&displacement),
                trim,
                Rc::clone(&bounds),
            ),
            gravity_g,
        ))), ship_length/n_parts as f64).values().last().expect("ShearForce values error: no last value");
        dbg!(i, value, trim, delta);
        if value.abs() < 0.1 {
            break; 
        }
        trim -= value.signum()*delta;
        delta *= 0.5;
    }

    let shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        data.water_density,
        Draught::new(
            center_waterline_shift,
            mean_draught,
            Rc::clone(&displacement),
            trim,
            Rc::clone(&bounds),
        ),
        gravity_g,
    ));


    // dbg!(&shear_force.values());
    let mut bending_moment = BendingMoment::new(Box::new(shear_force), ship_length/n_parts as f64);
    let bending_moment_values = bending_moment.values();
    // let shear_force_values = shear_force.values();
    dbg!(&bending_moment_values.len());

    let metacentric_height: Rc<RefCell<dyn IMetacentricHeight>> = Rc::new(RefCell::new(MetacentricHeight::new(
        center_draught_shift, // отстояние центра величины погруженной части судна
        rad_long,                  // продольный метацентрические радиус
        rad_cross,                 // поперечный метацентрические радиус
        Rc::clone(&mass),              // все грузы судна
    )));
   // let mut stability_arm = StabilityArm::new(Curve2D::new());

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
