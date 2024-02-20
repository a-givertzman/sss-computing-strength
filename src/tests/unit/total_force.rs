#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use crate::total_force::TotalForce;
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test TotalForce";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = TotalForce::new(
            Vec::from([20.; 10]),
            Vec::from([5., 25., 25., 25., 25., 25., 25., 25., 15., 5.]),
            9.81,
        ).values();
        let target = Vec::from([15., -5., -5., -5., -5., -5., -5., -5., 5., 15.]);
       
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        testDuration.exit();
    }
}
