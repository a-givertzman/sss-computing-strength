#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use crate::{
    area::{HAreaStability, HAreaStrength, VerticalArea},
    data::structs::NavigationArea,
    icing::{FakeIcing, IIcingStab, IcingMass, IcingStab},
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
/*



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

    // шпангоуты
    let frames: Vec<Frame> = data
        .theoretical_frame
        .iter()
        .map(|v| {
            Frame::new(
                v.x - data
                    .theoretical_frame
                    .last()
                    .expect("frames last error: no frame")
                    .x
                    / 2.,
                Curve::new_linear(&v.immersion_area),
            )
        })
        .collect();

    // длинна судна99
    //let ship_length = frames.last().unwrap().shift_x() - frames.first().unwrap().shift_x();
    //let ship_length = 120.0<<<<<<< MetacentricHeight
    //let n_parts = 20; //data.n_parts as usize;
    // вектор разбиения судна на отрезки
    //let bounds = Rc::new(Bounds::from_n(ship_length, n_parts));
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
    for index in 0..frames.len() - 1 {
        let bound = Bound::new(frames[index].shift_x(), frames[index + 1].shift_x());
        if let Some(mass) = data.load_constant.data().get(&index) {
            let load_mass = Rc::new(LoadMass::new(
                *mass,
                bound,
                Some(Position::new(
                    bound.center(),
                    const_shift.y(),
                    const_shift.z(),
                )),
            ));
            loads_const.push(load_mass);
        }
    }

    data.load_spaces.iter().for_each(|v| {
        if let Some(mass_shift) = v.mass_shift {
            let load = Rc::new(LoadMass::new(
                v.mass,
                Bound::from(v.bound_x),
                Some(Position::new(mass_shift.0, mass_shift.1, mass_shift.2)),
            ));
            load_mass.push(load);
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
    // Давление ветра +
    // Добавка на порывистость ветра для
    // неограниченного района плавания
    let (p_v, m) = data
        .navigation_area_param
        .get_area(&NavigationArea::Unlimited)
        .expect("main error no area data!");
    // Критерии остойчивости
    let mut criterion = Criterion::new(
        data.ship_type,
        data.navigation_area,
        desks.iter().find(|v| v.is_timber()).is_some(),
        bulk.iter().find(|v| v.moment() != 0.).is_some(),
        load_mass.iter().find(|v| v.value(None) != 0.).is_some(),
        flooding_angle,
        data.length,
        breadth,
        mean_draught,
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
        Rc::new(Grain::new(
            flooding_angle,
            Rc::clone(&bulk),
            Rc::clone(&mass),
            Rc::clone(&lever_diagram),
        )),
    );
    elapsed.insert("Completed", time.elapsed());

    let time = Instant::now();
    // criterion.create().iter().for_each(|v| println!("{v}"));
    send_stability_data("sss-computing", criterion.create());// TODO errors
*/
    let mut wd = vec![
        (-2685.79200000000,	0.00),
        (-2337.17400000000,	4.03),
        (-2284.38600000000,	0.98),       
        (-500.885400000000,	9.43),
        (1217.98200000000,	12.64),
        (1223.90400000000,	16.79),
        (2885.79600000000,	20.11),
        (2945.55600000000,	21.94),
        (4081.30200000000,	16.97),
        (4091.75400000000,	24.28),        
        (4768.43400000000,	17.96),
        (4790.20800000000,	33.20),
        (4896.71200000000,	18.08),
        (4988.58000000000,	33.20),        
        (6237.87000000000,	19.19),
        (6831.19000000000,	19.80),
        (6891.32000000000,	20.28),
        (7544.35000000000,	21.13),
        (7549.60000000000,	23.81),
        (7906.11000000000,	24.41),
        (7921.44000000000,	32.19),
        (8218.17000000000,	32.55),
        (8273.82000000000,	30.73),
        (10352.6800000000,	34.01),
        (10407.2800000000,	31.70),
        (11059.8200000000,	32.31),
        (11113.0200000000,	29.27),
        (11765.0700000000,	29.64),
        (11800.2800000000,	47.49),
        (12067.1200000000,	32.67),
        (12096.8700000000,	47.73),        
        (12422.6500000000,	32.79),
        (15252.4700000000,	26.60),
        (15338.7800000000,	40.32),
        (15889.6800000000,	19.43),        
        (15903.8200000000,	26.60),        
        (16659.4000000000,	19.43),
        (16661.0800000000,	20.28),
        (18082.1500000000,	20.28),
        (18138.0100000000,	18.58),
        (22934.5500000000,	18.83),
        (23113.3300000000,	19.43),
        (27256.6300000000,	18.70),
        (28914.0900000000,	18.46),
        (33473.5400000000,	18.58),
        (33478.0900000000,	20.89),
        (34188.5900000000,	20.89),
        (34245.6400000000,	19.80),
        (41230.3100000000,	18.70),
        (42414.0800000000,	18.46),
        (48157.5100000000,	18.46),
        (55025.7000000000,	18.34),
        (56741.8200000000,	17.85),
        (60589.0900000000,	17.13),
        (65795.2700000000,	14.94),
        (69221.,	12.27),
        (70827.8000000000,	10.83),
        (72420.8000000000,	9.61),
        (77024.6000000000,	7.27),
        (77037.8000000000,	11.73),
        (79395.8000000000,	9.66),
        (81223.4000000000,	7.95),
        (83109.8000000000,	6.25),
        (84760.4000000000,	4.66),
        (85880.,	3.33),
        (87176.0000000000,	1.87),
        (87821.,	0.00),   
    ];
    wd.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut bd: Vec<(f64, f64)> = vec![
        (0.00,	0.00),
        (1615.38600000000,	0.21),
        (2538.46200000000,	1.52),
        (3865.38600000000,	4.14),
        (4442.31000000000,	5.45),
        (4783.09800000000,	6.09),
        (5135.20900000000,	6.83),
        (5726.75800000000,	8.29),
        (6200.00,	9.27),
        (6554.90000000000,	9.88),
        (8329.61000000000,	13.78),
        (9690.13000000000,	16.84),
        (11464.7700000000,	20.62),
        (13180.2600000000,	23.67),
        (14836.6000000000,	26.60),
        (16729.6100000000,	29.04),
        (18918.3000000000,	31.12),
        (20870.3900000000,	32.59),
        (22940.8500000000,	33.20),
        (24656.3400000000,	33.45),
        (26076.0800000000,	33.21),
        (27791.5700000000,	32.72),
        (30512.6800000000,	31.51),
        (34239.4100000000,	29.57),
        (37492.9400000000,	27.99),
        (41160.5900000000,	26.04),
        (45656.3400000000,	23.73),
        (49442.2200000000,	21.67),
        (53878.8900000000,	19.36),
        (57842.2200000000,	17.30),
        (61214.0500000000,	15.23),
        (63402.8100000000,	13.65),
        (65769.0200000000,	11.82),
        (67898.5600000000,	10.00),
        (69225.2000000000,	8.55),
        (70227.2000000000,	7.58),
        (73234.4000000000,	5.02),
        (74237.00,	4.17),
        (76538.00,	2.70),
        (78663.2000000000,	1.71),
        (80079.8000000000,	1.10),
        (82856.00,	0.34),
        (86225.00,	0.07),
    ];
    bd.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
    let bounds = Rc::new(Bounds::from_n((wd.last().unwrap().0 - wd[0].0)/1000., wd.len()));
    dbg!(&bounds);
    let mut load_mass: Vec<Rc<dyn ILoadMass>> = Vec::new();

    let delta_l = bounds.length()/2.;
    let fix_l = wd.last().unwrap().0/2000.;
    for i in 0..wd.len()-1 {
            let (x1, x2) = (wd[i].0/1000.0 - delta_l, wd[i+1].0/1000.0 - delta_l);
    //        let m = (x2 - x1)*(wd[i+1].1 + wd[i].1)/2.;
            let m = (x2 - x1)*wd[i].1;
            let dx = (x1 + x2)/2.;
            println!("{i} {x1} {x2} {m} {dx}");
            let load = Rc::new(LoadMass::new(
                m,
                Bound::new(x1, x2),
                Some(Position::new(dx, 0., 0.)),
            ));
            println!("{}", load);
            load_mass.push(load);
    }
    for i in 0..bd.len()-1 {
            let (x1, x2) = (bd[i].0/1000.0 - delta_l, bd[i+1].0/1000.0 - delta_l);
    //        let m: f64 = (x2 - x1)*(bd[i+1].1 + bd[i].1)/2.;
            let m: f64 = (x2 - x1)*bd[i].1;
            let dx = (x1 + x2)/2.;
            println!("{i} {x1} {x2} {m} {dx}");
            let load = Rc::new(LoadMass::new(
                -m,
                Bound::new(x1, x2),
                Some(Position::new(dx, 0., 0.)),
            ));
            println!("{}", load);
            load_mass.push(load);
    }
        // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    let mass: Rc<dyn IMass> = Rc::new(Mass::new(
        Rc::new(Vec::new()),
        Position::new(0.,0.,0.,),
        Rc::new(FakeIcing::new(0.,Moment::new(0.,0.,0.,),)),
        Rc::new(load_mass),
        Rc::clone(&bounds),
    ));
    let mut shear_force = ShearForce::new(TotalForce::new(
        Rc::clone(&mass),
        1.025,
        Volume::new(
            0.,
            0.,
            Rc::new(FakeDisplacement::new(0.)),
            0.,
            Rc::clone(&bounds),
        ),
        9.8,
    ));
    let shear_force_values = shear_force.values();
    let bending_moment_values = BendingMoment::new(Box::new(shear_force), bounds.delta()).values();
    
//    println!("mass");
//    mass.values().iter().for_each(|v| println!("{v};"));
    mass.values().iter().zip(bounds.iter().map(|v| v.center()).collect::<Vec<f64>>()).for_each(|v| println!("{} {};", v.1+fix_l, v.0));
    println!("shear_force");
//    shear_force_values.iter().zip(bounds.iter().map(|v| v.center()).collect::<Vec<f64>>()).for_each(|v| println!("{} {};", v.1, v.0));
 //   shear_force_values.iter().for_each(|v| println!("{v};"));
    println!("bending_moment");
 //   bending_moment_values.iter().zip(bounds.iter().map(|v| v.center()).collect::<Vec<f64>>()).for_each(|v| println!("{} {};", v.1, v.0));
 //   bending_moment_values.iter().for_each(|v| println!("{v};"));
   // dbg!(49.- delta_l);


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
