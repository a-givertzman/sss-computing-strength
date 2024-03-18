#[cfg(test)]

mod tests {
    use crate::{bending_moment::BendingMoment, shear_force::FakeShearForce};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn bending_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test BendingMoment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = BendingMoment::new(Box::new(FakeShearForce::new(vec![
            0.0, 5.0, 10., 15.0, 10.0, 5.0, 0.0, -5.0, -10.0, -15.0, -15.0, 0.0,
        ])), 2.)
        .values();
        let target = Vec::from([0.0, 5.0, 20.0, 45.0, 70.0, 85.0, 90.0, 85.0, 70.0, 45.0, 15.0, 0.0]);

        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
