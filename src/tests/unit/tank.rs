#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{load::{ILoad, LoadSpace}, math::{bound::Bound, curve::Curve, inertia_shift::InertiaShift, mass_moment::MassMoment, pos_shift::PosShift, position::Position, surface_moment::SurfaceMoment}, tank::Tank};
    
    static INIT: Once = Once::new();
    
    static mut TANK: Option<Tank> = None;

    fn initOnce() {
        INIT.call_once(|| {
                let tank_center_shift = PosShift::new(
                    Curve::new(vec![(0., 2.), (10., 2.)]),
                    Curve::new(vec![(0., 0.), (10., 0.)]),
                    Curve::new(vec![(0., 0.), (10., 0.)]),
                );
                let tank_free_surf_inertia = InertiaShift::new(
                    Curve::new(vec![(0., 0.), (10., 1.)]),
                    Curve::new(vec![(0., 0.), (10., 1.)]),
                );
        
                unsafe { TANK = Some(Tank::new(
                    2.,
                    10.,
                    Bound::new(0., 4.),
                    tank_center_shift,
                    tank_free_surf_inertia,
                )); }
            }
        )
    }

    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        println!("");
        let self_id = "test Tank mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().mass(None) };
        let target = 20.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    fn center() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        println!("");
        let self_id = "test Tank center";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().center() };
        let target = Position::new(2., 0., 0.);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    fn moment_surface() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        println!("");
        let self_id = "test Tank moment_surface";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { TANK.clone().unwrap().moment_surface() };
        let target = SurfaceMoment::new(2., 2.,);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}