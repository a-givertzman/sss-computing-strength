#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::bound::*;
    
    #[test]
    fn intersect() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Bound intersect";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let bound = Bound::new(2., 4.);
        assert_eq!(bound.intersect(&Bound::new(0., 1.,)), None);
        assert_eq!(bound.intersect(&Bound::new(4., 5.,)), None);
        assert_eq!(bound.intersect(&Bound::new(1., 3.,)), Some(Bound::new(2., 3.)));
        assert_eq!(bound.intersect(&Bound::new(1., 5.,)), Some(Bound::new(2., 4.)));
        assert_eq!(bound.intersect(&Bound::new(3., 5.,)), Some(Bound::new(3., 4.)));
        assert_eq!(bound.intersect(&Bound::new(2., 3.,)), Some(Bound::new(2., 3.)));

        testDuration.exit();
    }

    #[test]
    fn part() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Bound part";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let bound = Bound::new(2., 4.);
        assert_eq!(bound.part(&Bound::new(0., 1.,)), 0.);
        assert_eq!(bound.part(&Bound::new(4., 5.,)), 0.);
        assert_eq!(bound.part(&Bound::new(1., 3.,)), 0.5);
        assert_eq!(bound.part(&Bound::new(1., 5.,)), 1.);
        assert_eq!(bound.part(&Bound::new(3., 5.,)), 0.5);
        assert_eq!(bound.part(&Bound::new(2., 3.,)), 0.5);

        testDuration.exit();
    }

    #[test]
    fn center() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Bound center";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let bound = Bound::new(-2., 4.);
        assert_eq!(bound.center(), 1.);

        testDuration.exit();
    }

    #[test]
    fn add() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Bound add";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut bound = Bound::new(2., 4.);
        bound.add(-5.);
        assert_eq!(bound, Bound::new(-3., -1.));

        testDuration.exit();
    }
}