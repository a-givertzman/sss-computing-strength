#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, stability::wind::*, strength::mass::*};

    #[test]
    fn wind() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Wind";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Wind::new(
            200.,
            0.50,
            1000.0,
            5.0,
            9.81,
            Rc::new(FakeMass::new(
                1000./9.81,
                vec![0.],
                Position::new(0., 0., 0.),
                DeltaMH::new(0., 0.),
                Position::new(0., 0., 0.,), 
                SurfaceMoment::new(0., 0.,),
            )),
        )
        .arm_wind_dynamic();
        let target = 1.5;

        assert!(
            (result - target).abs() < result.abs() * 0.01, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
