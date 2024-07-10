#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, load::*};

    static INIT: Once = Once::new();

    static mut TANK: Option<Tank> = None;

    fn init_once() {
        INIT.call_once(|| {
            let tank_center_shift = PosShift::new(
                Curve::new_linear(&vec![(0., 2.), (10., 2.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 0.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 0.)]),
            );
            let tank_free_surf_inertia = InertiaShift::new(
                Curve::new_linear(&vec![(0., 0.), (10., 1.)]),
                Curve::new_linear(&vec![(0., 0.), (10., 1.)]),
            );

            unsafe {
                TANK = Some(Tank::new(
                    2.,
                    10.,
                    Bound::new(0., 4.),
                    tank_center_shift,
                    tank_free_surf_inertia,
                ));
            }
        })
    }

    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Tank mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().mass(None) };
        let target = 20.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn center() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Tank center";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().mass_shift() };
        let target = Position::new(2., 0., 0.);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn moment_surface() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Tank moment_surface";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().moment_surface() };
        let target = SurfaceMoment::new(2., 2.);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
