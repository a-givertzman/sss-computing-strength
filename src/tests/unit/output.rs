#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::data::serialize_out::*;
    
    #[test]
    fn serizlize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Serizlize out";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let data = OutData {
            shear_force: vec![ (-10.0, 0.0), (-5.0, -10.0), (0.0, 0.0), (5.0, 10.0), (10.0, 0.0)],
            bending_moment: vec![(-10.0, 0.0), (-5.0, 5.0), (0.0, 10.0), (5.0, 5.0), (10.0, 0.0)]           
        };        
    
        let result = OutData::serialize(&data).expect("serialize error");

        let target = r#"
        {
            "shear_force": [[-10.0, 0.0], [-5.0, -10.0], [0.0, 0.0], [5.0, 10.0], [10.0, 0.0]],
            "bending_moment": [[-10.0, 0.0], [-5.0, 5.0], [0.0, 10.0], [5.0, 5.0], [10.0, 0.0]]
        }
        "#;
        let target = target.split(' ').map(|s| s.trim()).collect::<String>();

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}
