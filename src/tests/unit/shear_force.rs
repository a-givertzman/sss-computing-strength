#[cfg(test)]

mod tests {
    use crate::{
        shear_force::{IShearForce, ShearForce}, total_force::FakeTotalForce,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn shear_force() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test ShearForce";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result =
            ShearForce::new(FakeTotalForce::new(vec![15., -5., -5., -5., -5., -5., -5., -5., 5., 15.])).values();
        let target = Vec::from([
            0.0, 15.0, 10.0, 5.0, 0.0, -5.0, -10.0, -15.0, -20.0, -15.0, 0.0,
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
