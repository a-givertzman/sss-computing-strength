#[cfg(test)]

mod tests {
    use crate::math::*;
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn from_n() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Bounds create from_n";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Bounds::from_n(20., 4);
        let target = Bounds::new(vec![
            Bound::new(-10., -5.),
            Bound::new(-5., 0.),
            Bound::new(0., 5.),
            Bound::new(5., 10.),
        ]);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn from_frames() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Bounds create from_frames";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Bounds::from_frames(&vec![(-10., -5.), (-5., 0.), (0., 5.), (5., 10.),]);
        let target = Bounds::new(vec![
            Bound::new(-10., -5.),
            Bound::new(-5., 0.),
            Bound::new(0., 5.),
            Bound::new(5., 10.),
        ]);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
