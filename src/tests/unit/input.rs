#[cfg(test)]

mod tests {
    use std::time::Duration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::data::parse_input::*;
    
    #[test]
    fn input() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Parse request";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let data = r#"
        {
            "project_name": "YURIY ARSHENEVSKIY",
            "ship_name": "YURIY ARSHENEVSKIY",
            "n_parts": 20,
            "water_density": 1.025
        }"#;        
    
        let result = ParsedInputData::parse(&data).expect("parse error");
        let target = ParsedInputData {
            project_name: "YURIY ARSHENEVSKIY".to_string(),
            ship_name: "YURIY ARSHENEVSKIY".to_string(),
            n_parts: 20,
            water_density: 1.025,     
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }

    #[test]
    fn ship() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Parse ship";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let data = r#"
        {
            "ship_length": 200.0, 
            "center_waterline": [[0.0, 0.0], [10.0, 1.0]],
            "rad_long": [[0.0, 0.0], [10.0, 2.0]],
            "mean_draught": [[0.0, 0.0], [10.0, 3.0]],
            "center_shift": [[0.0, 2.0, 0.0, 0.0], [10.0, 2.0, 0.0, 0.0]]
        }"#;        
    
        let result = ParsedShipData::parse(&data).expect("parse error");
        let target = ParsedShipData {
            ship_length: 200.,
            center_waterline: vec![ (0.0, 0.0), (10.0, 1.0)],
            rad_long: vec![ (0.0, 0.0), (10.0, 2.0)],
            mean_draught: vec![ (0.0, 0.0), (10.0, 3.0)],
            center_shift: vec![(0.0, 2.0, 0.0, 0.0), (10.0, 2.0, 0.0, 0.0),],           
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }

    #[test]
    fn frames() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Parse frames";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let data = r#"
        {
            "frames": [ 
                {
                    "index": 0, 
                    "delta_x": 0,
                    "immersion_area": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]
                },
                {
                    "index": 1, 
                    "delta_x": 10,
                    "immersion_area": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]
                }
            ]
        }"#;        
    
        let result = ParsedFramesData::parse(&data).unwrap();
        let target = ParsedFramesData { frames: vec![ 
            FrameData {
                index: 0,
                delta_x: 0.,
                immersion_area: vec![ (0.0, 0.0), (1.0, 1.0), (5.0, 10.0),],            
            },
            FrameData {
                index: 1,
                delta_x: 10.,
                immersion_area: vec![ (0.0, 0.0), (1.0, 1.0), (5.0, 10.0),],            
            },
        ] };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }

    #[test]
    fn loads() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Parse loads";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let data = r#"
        {
            "load_space": [ 
                {
                    "mass": 10.0,
                    "bound": [-10.0, 0.0, 0.0, 5.0], 
                    "center": [0.0, 0.0, 1.0]
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
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }

    #[test]
    fn tanks() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Parse tanks";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let data = r#"
        {
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
    
        let result = ParsedTanksData::parse(&data).expect("parse error");
        let target = ParsedTanksData {
            tanks: vec![ TankData { 
                density: 0.7, 
                volume: 10.0, 
                bound: (10.0, 20.0, 5.0, 10.0), 
                center: vec![(0.0, 15.0, 7.5, -1.0), (10.0, 15.0, 7.5, 2.0),],
                free_surf_inertia: vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0),],
            }, ],           
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}
