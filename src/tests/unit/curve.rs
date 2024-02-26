#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::math::curve::Curve;

    #[test]
    fn sum_above() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Curve value";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let curve = Curve::new(vec![
            (0., 0.),
            (2., 2.),
        ]);
        let test_data = [(curve.value(-1.), 0.),
            (curve.value(0.), 0.),
            (curve.value(1.), 1.),
            (curve.value(2.), 2.),
            (curve.value(3.), 2.),];
        for (result, target) in test_data {
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }

        test_duration.exit();
    }
}

