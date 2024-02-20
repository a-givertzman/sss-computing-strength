use std::rc::Rc;

use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::debug;
use testing::entities::test_value::Value;

use crate::{
    bending_moment::BendingMoment,
    displacement::Displacement,
    draught::Draught,
    frame::Frame,
    load::ILoad,
    mass::Mass,
    math::{bound::Bound, curve::Curve, inertia_shift::InertiaShift, pos_shift::PosShift},
    shear_force::ShearForce,
    tank::Tank,
    total_force::TotalForce,
    trim::Trim,
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

    // длинна судна
    let ship_length = 200.;
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
    //ускорение свободного падения
    let gravity_g = 9.81;
    // плотность окружающей воды
    let water_density = 1.025;
    // отстояние центра тяжести ватерлинии по длине от миделя
    let center_waterline = Curve::new(vec![(0., 0.), (10., 1.)]);
    // поперечный метацентрический радиус
    let rad_trans = Curve::new(vec![(0., 0.), (10., 1.)]);
    // средняя осадка
    let mean_draught = Curve::new(vec![(0., 0.), (1000., 1.), (10000., 10.)]);

    // отстояние центра величины погруженной части судна
    let center_shift = PosShift::new(
        Curve::new(vec![(0., 2.), (10., 2.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
        Curve::new(vec![(0., 0.), (10., 0.)]),
    );
    //координаты центра объема жидкости в цистерне в системе координат судна
    let tank_center_shift = PosShift::new(
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
        tank_center_shift,
        tank_free_surf_inertia,
    )))];

    let mass = Mass::new(loads, bounds.clone());
    let frames = vec![
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
    ];

    let shear_force_values = ShearForce::new(
        TotalForce::new(
            mass.values(),
            Draught::new(
                Trim::new(
                    water_density, // плотность окружающей воды
                    mass,         // все грузы судна
                    ship_length,   // длинна судна
                    center_shift,  // отстояние центра величины погруженной части судна
                    rad_trans,     // поперечный метацентрические радиус
                ),
                Displacement::new(frames, ship_length),
                ship_length,
                bounds,
                center_waterline,
                mean_draught,
            ).values(),
            gravity_g,
        )
        .values(),
    )
    .values();
    let bending_moment_values = BendingMoment::new(&shear_force_values).values();
    dbg!(&shear_force_values, &bending_moment_values);
}
