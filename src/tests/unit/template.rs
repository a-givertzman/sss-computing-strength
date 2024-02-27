#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::max_test_duration::TestDuration,
    }; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    #[test]
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        initEach();
        println!("");
        let self_id = "test Template";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}
