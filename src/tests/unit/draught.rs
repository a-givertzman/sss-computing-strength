#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{rc::Rc, sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{displacement::Displacement, draught::Draught, frame::Frame, load::{ILoad, LoadSpace}, mass::Mass, math::{bound::Bound, curve::Curve, inertia_shift::InertiaShift, mass_moment::MassMoment, pos_shift::PosShift, position::Position}, trim::Trim};
    
    #[test]
    fn draught() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Draught";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let ship_length = 20.;

         // отстояние центра величины погруженной части судна
        let center_shift = PosShift::new(
            Curve::new(vec![(0., 1.), (10., 1.)]),
            Curve::new(vec![(0., 0.), (10., 0.)]),
            Curve::new(vec![(0., 0.), (10., 0.)]),
        );
        // поперечный метацентрические радиус
        let rad_trans = Curve::new(vec![(0., 0.), (10., 1.)]);

        // отстояние центра тяжести ватерлинии по длине от миделя
        let center_waterline = Curve::new(vec![(0., 0.), (10., 1.)]);

        // средняя осадка
        let mean_draught = Curve::new(vec![(0., 0.), (1000., 1.), (10000., 10.)]);

        let bounds = vec![Bound::new(-10., 10.)];
        let mass = Mass::new(
            vec![Rc::new(Box::new(LoadSpace::new(
                Bound::new(-10., 10.),
                Position::new(0., 0., 0.),
                10.,
            )))],
            bounds.clone(),
        );

        let frames = vec![
            Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
            Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
            Frame::new(Curve::new(vec![(0., 0.), (10., 10.)])),
        ];


        let result = Draught::new(
            Trim::new(
                1., // плотность окружающей воды
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
        ).values();

        let target = vec![1.];
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}