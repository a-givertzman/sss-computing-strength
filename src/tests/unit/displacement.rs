#[cfg(test)]

mod tests {
    use crate::{
        displacement::*,
        frame::Frame,
        math::{bound::Bound, curve::Curve},
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Displacement value";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let frames = vec![
            Frame::new(-10., Curve::new(&vec![(0., 0.), (10., 0.)])),
            Frame::new(10., Curve::new(&vec![(0., 0.), (10., 40.)])),
        ];

        let result = Displacement::new(frames,).value(Bound::new(-10., 0.), 10., 10.);
        let target = 100.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
