#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use crate::{bending_moment::BendingMoment, shear_force::FakeShearForce};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn bending_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test BendingMoment";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = BendingMoment::new(&FakeShearForce::new(vec![
            0.0, 5.0, 10., 15.0, 10.0, 5.0, 0.0, -5.0, -10.0, -15.0, -15.0, 0.0,
        ]))
        .values();
        let target = Vec::from([0.0, 5.0, 20.0, 45.0, 70.0, 85.0, 90.0, 85.0, 70.0, 45.0, 15.0, 0.0]);

        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        testDuration.exit();
    }
}
