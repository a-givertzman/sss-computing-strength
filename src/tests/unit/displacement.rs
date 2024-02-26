#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use crate::{
        displacement::*,
        frame::Frame,
        math::{bound::Bound, curve::Curve},
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Displacement value";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut frames = vec![
            Frame::new(Curve::new(vec![(0., 0.), (10., 0.)])),
            Frame::new(Curve::new(vec![(0., 0.), (10., 40.)])),
        ];

        let result = Displacement::new(frames, 20.).value(Bound::new(-10., 0.), 10.);
        let target = 100.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        testDuration.exit();
    }
}
