#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use crate::{
        load::{ILoad, LoadSpace},
        mass::Mass,
        math::{
            bound::Bound, curve::Curve, inertia_shift::InertiaShift, mass_moment::MassMoment,
            pos_shift::PosShift, position::Position,
        },
        trim::Trim,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use log::{debug, info, warn};
    use std::{
        rc::Rc,
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn trim() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Trim";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let result = Trim::new(
            118.39,
            Position::new(-0.194609657, 0., 0.735524704),
            696.702572991,           
            0.,         
            2044.10,      
            Position::new(1.05, 0., 5.32),
        )
        .value();
        let target = 0.2115;

        assert!(
            (result-target).abs() < result.abs() * 0.00005, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        testDuration.exit();
    }
}
