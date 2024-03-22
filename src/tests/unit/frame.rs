#[cfg(test)]

mod tests {
    use std::time::Duration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{frame::Frame, math::curve::Curve};
    
    #[test]
    fn frame() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Frame";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Frame::new(0., Curve::new_linear(&vec![(0., 0.), (2., 2.)])).area(1.);
        let target = 1.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}