
#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::math::vec::*;

    #[test]
    fn sum_above() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec sum_above";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = Vec::from([1., 2., 3.,]).sum_above();
        let target = Vec::from([0., 1., 3., 6.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    fn integral_sum() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec integral_sum";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = Vec::from([0., 1., 2., 3.,]).integral_sum();
        let target = Vec::from([0., 1., 4., 9.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }


    #[test]
    pub fn shift() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec shift";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut result = Vec::from([1., 0.,]);
        result.shift(1.);
        let target = Vec::from([2., 1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    pub fn div_single() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec div_single";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut target = Vec::from([10., -5.,]);
        target.div_single(-5.);
        let result = Vec::from([-2., 1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    pub fn mul_single() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec mul_single";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut target = Vec::from([2., 1.,]);
        target.mul_single(-2.);
        let result = Vec::from([-4., -2.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    pub fn add_vec() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec add_vec";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut result = Vec::from([1., 0.,]);
        result.add_vec(&Vec::from([1., 1.,]));
        let target = Vec::from([2., 1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    pub fn sub_vec() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec sub_vec";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut target = Vec::from([2., 1.,]);
        target.sub_vec(&Vec::from([1., 2.,]));
        let result = Vec::from([1., -1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit(); 
    }

    #[test]
    pub fn div_vec() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec div_vec";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut target = Vec::from([2., 1.,]);
        target.div_vec(&Vec::from([1., 2.,]));
        let result = Vec::from([2., 0.5,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    pub fn mul_vec() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Vec mul_vec";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let mut target = Vec::from([2., 1.,]);
        target.mul_vec(&Vec::from([1., 0.,]));
        let result = Vec::from([2., 0.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}
