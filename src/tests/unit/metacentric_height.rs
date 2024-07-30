#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        math::Position, stability::metacentric_height::MetacentricHeight, FakeMass, FakeParameters,
        FakeShipMoment, IMetacentricHeight,
    };

    static INIT: Once = Once::new();

    unsafe impl Sync for MetacentricHeight {} //for static
    static mut HEIGHT: Option<MetacentricHeight> = None;

    fn init_once() {
        INIT.call_once(|| unsafe {
            HEIGHT.replace(MetacentricHeight::new(
                Position::new(-1., 0., 2.),
                1000.,
                100.,
                Rc::new(Vec::new()),
                Rc::new(FakeMass::new(1000.0, vec![0.])),
                Rc::new(FakeShipMoment::new(
                    Position::new(1.0, 0., 2.),
                )),
                Rc::new(FakeParameters {}),
            ));
        })
    }

    #[test]
    fn h_long_fix() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test MetacentricHeight h_long_fix";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { HEIGHT.clone().unwrap().h_long_fix().unwrap() };
        let target = 1000.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn h_trans_fix() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test MetacentricHeight h_trans_fix";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { HEIGHT.clone().unwrap().h_trans_fix().unwrap() };
        let target = 100.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn z_g_fix() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test MetacentricHeight z_g_fix";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { HEIGHT.clone().unwrap().z_g_fix().unwrap() };
        let target = 2.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
