#[cfg(test)]

mod tests {
    use crate::{
        math::{
            bound::Bound, curve::Curve, inertia::inertia_shift::InertiaShift,
            pos_shift::PosShift, position::Position,
        }, metacentric_height::{FakeMetacentricHeight, IMetacentricHeight}, stability_arm::StabilityArm, tank::Tank, Bounds, Curve2D
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for StabilityArm {} //for static
    static mut STABILITY_ARM: Option<StabilityArm> = None;

    fn init_once() {
        INIT.call_once(|| {
            let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(FakeMetacentricHeight::new(
                100.,
                10.,
                5.,
                1.,
            ));

            let pantocaren = vec![(1., vec![(0., 0.), (30., 2.), (45., 3.), (60., 2.), (90., 0.),]),];
            let mut stability_arm = StabilityArm::new(Curve2D::from_values_linear(pantocaren), 1., metacentric_height);
            stability_arm.diagram();
            unsafe {
                STABILITY_ARM.replace(stability_arm);
            }
        })
    }

    #[test]
    #[ignore = "TODO"]
    fn angle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test StabilityArm angle";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { STABILITY_ARM.clone().unwrap().angle(45.) };
        let target = 50.;
        assert!(
            result[0] == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    #[ignore = "TODO"]
    fn diagram() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test StabilityArm diagram";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { STABILITY_ARM.clone().unwrap().diagram() };
        let target = vec![(0., 0.), (30., 2.), (45., 3.), (60., 2.), (90., 0.),];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    #[ignore = "TODO"]
    fn angle_static_roll() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test StabilityArm angle_static_roll";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { STABILITY_ARM.clone().unwrap().angle_static_roll() };
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
