#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        math::*, stability::acceleration::*, FakeMetacentricHeight, FakeRollingAmplitude,
        FakeRollingPeriod,
    };

    #[test]
    fn acceleration() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Acceleration";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Acceleration::new(
            1.,
            1.,
            Rc::new(FakeCurve::new(1., 1.)),
            Rc::new(FakeRollingPeriod::new(0., 1.)),
            Rc::new(FakeRollingAmplitude::new(0., 1.)),
            Rc::new(FakeMetacentricHeight::new(
                0.,
                1.,
                1.,
                0.,
                DeltaMH {
                    long: 0.,
                    trans: 0.,
                },
            )),
        )
        .calculate()
        .unwrap();

        let target = 0.3 / 0.0105;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
