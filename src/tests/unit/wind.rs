#[cfg(test)]

mod tests {
    use crate::{
        mass::FakeMass, math::Position, wind::Wind, DeltaMH, SurfaceMoment
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    #[ignore = "TODO"]
    fn wind() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Wind";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Wind::new(
            252.,
            0.52,
            1089.79,
            6.51,
            9.81,
            Rc::new(FakeMass::new(
                2044.10,
                vec![0.],
                Position::new(1.05, 0., 5.32),
                DeltaMH::new(0., 0.),
                Position::new(0., 0., 0.,), 
                SurfaceMoment::new(0., 0.,),
            )),
        )
        .arm_wind_dynamic();
        let target = 0.2115;

        assert!(
            (result - target).abs() < result.abs() * 0.01, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
