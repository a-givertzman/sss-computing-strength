#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{frame::*, math::curve::Curve};
    
    #[test]
    fn area() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Frame area";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = Frame::new(Curve::new(vec![(0., 0.), (2., 2.)]), 1.).area(1.);
        let target = 1.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}