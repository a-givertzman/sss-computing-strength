#[cfg(test)]

mod tests {
    use std::{rc::Rc, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{metacentric_height::FakeMetacentricHeight, rolling_period::{IRollingPeriod, RollingPeriod}};
    
    #[test]
    fn rolling_period() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test RollingPeriod";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = RollingPeriod::new(
            4.,
            1.,
            10.,  
            Rc::new(FakeMetacentricHeight::new(
                100.,
                1.,
                0.,
                1.,
            )),
        ).calculate();        
        let target = 3.686;
        assert!((result - target).abs() < 0.001, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}