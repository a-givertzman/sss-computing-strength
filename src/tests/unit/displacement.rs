#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{displacement::*, frame::Frame, math::{bound::Bound, curve::Curve}};
    
    #[test]
    fn value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Displacement value";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut frames = Vec::new();
        frames.push(Frame::new(
            Curve::new(vec![(0., 0.), (10., 10.)])
        ));
        frames.push(Frame::new(
            Curve::new(vec![(0., 0.), (10., 20.)])
        ));
        frames.push(Frame::new(
            Curve::new(vec![(0., 0.), (10., 30.)])
        ));

        let result = Displacement::new(frames, 20.).value(Bound::new(-5., 5.), 10.);
        let target = 200.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}