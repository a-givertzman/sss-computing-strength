#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{load::{ILoad, LoadSpace}, math::{bound::Bound, mass_moment::MassMoment, position::Position}};
    
    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Tank";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = 
        let target = 
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}