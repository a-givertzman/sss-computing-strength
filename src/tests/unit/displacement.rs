#[cfg(test)]

mod tests {
    use crate::math::bound::Bound;
    use crate::strength::displacement::*;
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Displacement value";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let frame_area_data = vec![
            crate::data::structs::ParsedFrameData {
                x: -10.,
                immersion_area: vec![(0., 0.), (10., 0.)],
            },
            crate::data::structs::ParsedFrameData {
                x: 10.,
                immersion_area: vec![(0., 0.), (10., 40.)],
            },
        ];

        let result = Displacement::new(frame_area_data)
            .unwrap()
            .value(Bound::new(-10., 0.).unwrap(), 10., 10.)
            .unwrap();
        let target = 100.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
