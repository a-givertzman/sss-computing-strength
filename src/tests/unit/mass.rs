#[cfg(test)]

mod tests {
    use crate::{
        load::{ILoad, LoadSpace},
        mass::{IMass, Mass},
        math::{
            bound::Bound, curve::Curve, inertia_shift::inertia_shift::InertiaShift, pos_shift::PosShift, position::Position
        },
        tank::Tank,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        rc::Rc,
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for Mass {} //for static
    static mut MASS: Option<Mass> = None;

    fn init_once() {
        INIT.call_once(|| {
            let center = PosShift::new(
                Curve::new(vec![(0., 1.), (10., 1.)]),
                Curve::new(vec![(0., 0.), (10., 0.)]),
                Curve::new(vec![(0., 0.), (10., 0.)]),
            );

            let free_surf_inertia = InertiaShift::new(
                Curve::new(vec![(0., 0.), (10., 1.)]),
                Curve::new(vec![(0., 0.), (10., 1.)]),
            );

            let loads: Vec<Rc<Box<dyn ILoad>>> = vec![
                Rc::new(Box::new(LoadSpace::new(
                    10.,
                    Bound::new(-10., 0.),
                    Position::new(-5., 0., 0.),
                ))),
                Rc::new(Box::new(LoadSpace::new(
                    20.,
                    Bound::new(0., 10.),
                    Position::new(5., 0., 0.),
                ))),
                Rc::new(Box::new(Tank::new(
                    2.,
                    10.,
                    Bound::new(-5., 5.),
                    center,
                    free_surf_inertia,
                ))),
            ];

            let bounds = vec![
                Bound::new(-10., -5.),
                Bound::new(-5., 0.),
                Bound::new(0., 5.),
                Bound::new(5., 10.),
            ];

            unsafe {
                MASS.replace(Mass::new(loads, bounds));
            }
        })
    }

    #[test]
    fn sum() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Mass sum";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        /*     let loads: Vec<Rc<Box<dyn ILoad>>> = vec![
                    Rc::new(Box::new(LoadSpace::new(
                        Bound::new(-10., 0.),
                        Position::new(-5., 0., 0.),
                        10.,
                    ))),
                    Rc::new(Box::new(LoadSpace::new(
                        Bound::new(0., 10.),
                        Position::new(5., 0., 0.),
                        20.,
                    ))),
                    Rc::new(Box::new(LoadSpace::new(
                        Bound::new(-5., 5.),
                        Position::new(0., 0., 0.),
                        10.,
                    ))),
                ];

                let bounds = vec![
                    Bound::new(-10., -5.),
                    Bound::new(-5., 0.),
                    Bound::new(0., 5.),
                    Bound::new(5., 10.),
                ];
        */
        let result = unsafe { MASS.clone().unwrap().sum() };
        let target = 50.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn values() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Mass values";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { MASS.clone().unwrap().values() };
        let target = vec![5., 15., 20., 10.];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn delta_m_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Mass delta_m_h";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { MASS.clone().unwrap().delta_m_h() };
        let target = 0.04; // valie from curve 1. * density 2. / mass sum 50.
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
