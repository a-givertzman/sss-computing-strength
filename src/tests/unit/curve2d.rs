#[cfg(test)]

mod tests {
    use std::time::Duration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::math::curve2d::{Curve2D, ICurve2D};

    #[test]
    #[ignore = "Checked bihavior on empty input array"]
    fn curve_except() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Curve2D value";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let curve = Curve2D::from_values_linear(vec![
            // (2., 2.)
        ]);
        let test_data = [
            (curve.value(-1., -1.), 2.),
            (curve.value(0., 0.), 2.),
            (curve.value(1., 1.), 2.),
            (curve.value(2., 2.), 2.),
            (curve.value(3., 3.), 2.),];
        for (result, target) in test_data {
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }

    #[test]
    fn curve2d() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Curve2D value";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let curve = Curve2D::from_values_linear(vec![
            (0., vec!((0., 0.), (0., 0.),)), 
            (2., vec!((0., 2.), (2., 2.),)),
            (-2., vec!((0., -2.), (2., -2.),))
        ]);
        let test_data = [
            (curve.value(0., 0.), 0.),
            (curve.value(2., 0.), 2.),
            (curve.value(-2., 0.), -2.),
            (curve.value(3., -1.), 2.),
            (curve.value(-3., 3.), -2.),
            (curve.value(-1., -1.), -1.),];
        for (result, target) in test_data {
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
}

