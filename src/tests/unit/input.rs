#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::data::parse_input::*;
    
    #[test]
    fn ship() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Parse ship";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let data = r#"
        {
            "ship_length": 200.0, 
            "center_waterline": [[0.0, 0.0], [10.0, 1.0]],
            "rad_trans": [[0.0, 0.0], [10.0, 2.0]],
            "mean_draught": [[0.0, 0.0], [10.0, 3.0]],
            "center_shift": [[0.0, 2.0, 0.0, 0.0], [10.0, 2.0, 0.0, 0.0]]
        }"#;        
    
        let result = ParsedShipData::parse(&data).expect("parse error");
        let target = ParsedShipData {
            ship_length: 200.,
            center_waterline: vec![ (0.0, 0.0), (10.0, 1.0)],
            rad_trans: vec![ (0.0, 0.0), (10.0, 2.0)],
            mean_draught: vec![ (0.0, 0.0), (10.0, 3.0)],
            center_shift: vec![(0.0, 2.0, 0.0, 0.0), (10.0, 2.0, 0.0, 0.0),],           
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }

    #[test]
    fn frames() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Parse frames";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let data = r#"
        {
            "frames": [ 
                {
                    "index": 0, 
                    "immersion_area": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]
                }
            ]
        }"#;        
    
        let result = ParsedFramesData::parse(&data).expect("parse error");
        let target = ParsedFramesData { frames: vec![ FrameData {
            index: 0,
            immersion_area: vec![ (0.0, 0.0), (1.0, 1.0), (5.0, 10.0),],            
        },] };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }

    #[test]
    fn loads() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Parse loads";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let data = r#"
        {
            "load_space": [ 
                {
                    "mass": 10.0,
                    "bound": [-10.0, 0.0, 0.0, 5.0], 
                    "center": [0.0, 0.0, 1.0]
                }
            ],
        
            "tanks": [
                {
                    "density": 0.7,
                    "volume": 10.0,
                    "bound": [10.0, 20.0, 5.0, 10.0], 
                    "center": [[0.0, 15.0, 7.5, -1.0], [10.0, 15.0, 7.5, 2.0]],
                    "free_surf_inertia": [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]]       
                }
            ]
        }"#;        
    
        let result = ParsedLoadsData::parse(&data).expect("parse error");
        let target = ParsedLoadsData {
            load_space: vec![ LoadSpaceData { 
                mass: 10.0, 
                bound: (-10.0, 0.0, 0.0, 5.0), 
                center: (0.0, 0.0, 1.0), 
            }, ],
            tanks: vec![ TankData { 
                density: 0.7, 
                volume: 10.0, 
                bound: (10.0, 20.0, 5.0, 10.0), 
                center: vec![(0.0, 15.0, 7.5, -1.0), (10.0, 15.0, 7.5, 2.0),],
                free_surf_inertia: vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0),],
            }, ],           
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}
