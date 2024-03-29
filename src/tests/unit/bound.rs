#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::math::bound::*;
    
    #[test]
    fn intersect() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Bound intersect";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (Bound::new(2., 4.), Bound::new(0., 1.,), None),
            (Bound::new(2., 4.), Bound::new(4., 5.,), None),
            (Bound::new(2., 4.), Bound::new(1., 3.,), Some(Bound::new(2., 3.))),
            (Bound::new(2., 4.), Bound::new(1., 5.,), Some(Bound::new(2., 4.))),
            (Bound::new(2., 4.), Bound::new(3., 5.,), Some(Bound::new(3., 4.))),
            (Bound::new(2., 4.), Bound::new(2., 3.,), Some(Bound::new(2., 3.))),
        ];
        for (left, right, target) in test_data {
            let result = left.intersect(&right);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }

    #[test]
    fn part_ratio() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Bound part_ratio";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (Bound::new(2., 4.), Bound::new(0., 1.,), 0.),
            (Bound::new(2., 4.), Bound::new(4., 5.,), 0.),
            (Bound::new(2., 4.), Bound::new(1., 3.,), 0.5),
            (Bound::new(2., 4.), Bound::new(1., 5.,), 1.),
            (Bound::new(2., 4.), Bound::new(3., 5.,), 0.5),
            (Bound::new(2., 4.), Bound::new(2., 3.,), 0.5),
        ];
        for (left, right, target) in test_data {
            let result = left.part_ratio(&right);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }

    #[test]
    fn center() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Bound center";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let result = Bound::new(-2., 4.).center();   
        let target = 1.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}