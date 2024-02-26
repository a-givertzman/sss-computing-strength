
#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::vec_f64::*;

    #[test]
    fn sum_above() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 sum_above";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = VecF64::new(vec![1., 2., 3.,]).sum_above();
        let target = VecF64::new(vec![0., 1., 3., 6.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    fn integral_sum() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 integral_sum";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = VecF64::new(vec![0., 1., 2., 3.,]).integral_sum();
        let target = VecF64::new(vec![0., 1., 4., 9.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    pub fn add() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 add";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = VecF64::new(vec![1., 0.,]) + VecF64::new(vec![1., 1.,]);
        let target = VecF64::new(vec![2., 1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        let result = VecF64::new(vec![0., 1.,]) + VecF64::new(vec![-1., 0.,]);
        let target = VecF64::new(vec![-1., 1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    pub fn sub() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 sub";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let target = VecF64::new(vec![2., 1.,]) - VecF64::new(vec![1., 2.,]);
        let result = VecF64::new(vec![1., -1.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        let target = VecF64::new(vec![0., 1.,]) - VecF64::new(vec![1., -2.,]);
        let result = VecF64::new(vec![-1., 3.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit(); 
    }

    #[test]
    pub fn div() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 div";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let target = VecF64::new(vec![2., 1.,]) / VecF64::new(vec![1., 2.,]);
        let result = VecF64::new(vec![2., 0.5,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        let target = VecF64::new(vec![0., 1.,]) / VecF64::new(vec![1., -2.,]);
        let result = VecF64::new(vec![0., -0.5,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    pub fn mul() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test VecF64 mul";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let target = VecF64::new(vec![2., 1.,]) * VecF64::new(vec![1., 0.,]);
        let result = VecF64::new(vec![2., 0.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        let target = VecF64::new(vec![0., 1.,]) * VecF64::new(vec![0., -2.,]);
        let result = VecF64::new(vec![0., -2.,]);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}
