#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use crate::{
        load::{ILoad, LoadSpace},
        mass::Mass,
        math::{
            bound::Bound, curve::Curve, inertia_shift::InertiaShift, mass_moment::MassMoment,
            pos_shift::PosShift, position::Position,
        },
        trim::Trim,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        rc::Rc,
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn trim() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Trim";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        // отстояние центра величины погруженной части судна
        let center_shift = PosShift::new(
            Curve::new(vec![(0., 1.), (10., 1.)]),
            Curve::new(vec![(0., 0.), (10., 0.)]),
            Curve::new(vec![(0., 0.), (10., 0.)]),
        );
        // поперечный метацентрические радиус
        let rad_trans = Curve::new(vec![(0., 0.), (10., 1.)]);

        let mass = Mass::new(
            vec![Rc::new(Box::new(LoadSpace::new(
                Bound::new(-10., 10.),
                Position::new(0., 0., 0.),
                10.,
            )))],
            vec![Bound::new(-10., 10.)],
        );

        let result = Trim::new(
            1.,           // плотность окружающей воды
            mass,         // все грузы судна
            20.,          // длинна судна
            center_shift, // отстояние центра величины погруженной части судна
            rad_trans,    // поперечный метацентрические радиус
        )
        .value();

        let target = 1.;

        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        testDuration.exit();
    }
}
