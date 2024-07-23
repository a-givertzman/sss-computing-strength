#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::stability::{metacentric_height::*, rolling_period::*};

    #[test]
    fn rolling_period() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test RollingPeriod";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = RollingPeriod::new(
            10.,
            4.,
            1.,
            Rc::new(FakeMetacentricHeight::new(0., 0., 1., 0.)),
        )
        .calculate()
        .unwrap();
        let target = 3.686;
        assert!(
            (result - target).abs() < 0.001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
