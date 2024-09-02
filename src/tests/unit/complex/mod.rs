/// Комплексный тест с реальными значениями
mod input_data;

#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{bending_moment::BendingMoment, displacement::Displacement, draught::Draught, frame::Frame, load::ILoad, mass::{IMass, Mass}, math::{bound::Bound, curve::Curve, inertia_shift::*, pos_shift::PosShift}, shear_force::{IShearForce, ShearForce}, tank::Tank, tests::unit::complex::input_data, total_force::TotalForce, trim::Trim};
    
    #[test]
    #[ignore = "TODO"]
    fn complex() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Complex";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();


        let input_data = input_data::input_data();

        // длинна судна
        let ship_length = ship.ship_length;
        let n =  input_data.n_parts;
        let delta_x = ship.ship_length / n as f64;
        let start_x = -ship.ship_length / 2.;

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
        let water_density = input_data.water_density;
        // отстояние центра тяжести ватерлинии по длине от миделя
        let center_waterline_shift = Curve::new(ship.center_waterline);
        // продольный метацентрический радиус
        let rad_long = Curve::new(ship.rad_long);
        // средняя осадка
        let mean_draught = Curve::new(ship.mean_draught);
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


        frames.frames.sort_by(|a, b| a.index.partial_cmp(&b.index).expect("sort error"));
        let mut delta_x = 0.;
        let frames = frames.frames.into_iter().map(|f|  {
            delta_x += f.delta_x;
            Frame::new(delta_x, Curve::new(f.immersion_area))            
        }).collect();

        let shear_force = ShearForce::new(TotalForce::new(
            Rc::clone(&mass),
            Draught::new(
                ship_length,
                water_density,
                bounds,
                Rc::clone(&mass),
                center_waterline_shift,
                mean_draught,
                Displacement::from_frames(frames),
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




        let result = Frame::new(0., Curve::new(vec![(0., 0.), (2., 2.)])).area(1.);
        let target = 1.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}


