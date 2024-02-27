#[cfg(test)]

mod tests {
    use crate::{
        mass::FakeMass,
        math::{curve::FakeCurve, pos_shift::FakePosShift, position::Position},
        trim::Trim,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn trim() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Trim";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Trim::new(
            1.025,
            118.39,
            FakePosShift::new(Position::new(-0.194609657, 0., 0.735524704)),
            FakeCurve::new(696.702572991),
            Rc::new(FakeMass::new(
                2044.10,
                vec![0.],
                Position::new(1.05, 0., 5.32),
                0.,
            )),
        )
        .value();
        let target = 0.2115;

        assert!(
            (result - target).abs() < result.abs() * 0.00005, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
