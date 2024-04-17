#[cfg(test)]

mod tests {
    use crate::icing::FakeIcing;
    use crate::math::Bounds;
    use crate::math::{
        curve::Curve, inertia::inertia_shift::InertiaShift, pos_shift::PosShift,
        position::Position,
    };
    use crate::{load::*, mass::*, Moment};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for Mass {} //for static
    static mut MASS: Option<Mass> = None;

    fn init_once() {
        INIT.call_once(|| {
            let center = PosShift::new(
                Curve::new_linear(&vec![(0., 1.), (10., 1.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 0.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 0.)]),
            );

            let free_surf_inertia = InertiaShift::new(
                Curve::new_linear(&vec![(0., 0.), (10., 1.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 1.)]),
            );

            let loads_const: Vec<Rc<Box<dyn ILoad>>> = vec![
                Rc::new(Box::new(LoadSpace::from(
                    10.,
                    Some(Position::new(-5., 0., 0.)),
                    (-10., 0.),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ))),
                Rc::new(Box::new(LoadSpace::from(
                    20.,
                    Some(Position::new(5., 0., 0.)),
                    (0., 10.),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ))),
            ];

            let loads_cargo: Vec<Rc<Box<dyn ILoad>>> = vec![Rc::new(Box::new(LoadSpace::from(
                20.,
                Some(Position::new(0., 0., 0.)),
                (-5., 5.),
                None,
                None,
                None,
                None,
                Some(1.0),
                Some(2.0),
            )))];

            unsafe {
                MASS.replace(Mass::new(
                    loads_const,
                    Position::new(0., 0., 0.),
                    Rc::new(FakeIcing::new(0., Moment::new(0., 0., 0.,),)),
                    Rc::new(loads_cargo),
                    Rc::new(Bounds::from_n(20., 4)),
                ));
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

        let result = unsafe { MASS.clone().unwrap().delta_m_h().long() };
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
