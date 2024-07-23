#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, strength::trim::*, trim::ITrim, Displacement, FakeMass, Frame};

    #[test]
    fn trim_strength() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Trim";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let frame_area_data = vec![
            crate::data::structs::ParsedFrameData{ x: -10., immersion_area: vec![(0., 0.), (2., 10.)], },
            crate::data::structs::ParsedFrameData{ x: 10.,  immersion_area: vec![(0., 0.), (2., 10.)], },
        ];
        let ship_length = 20.;
        let result = Trim::new(
            ship_length,
            1.0,   
            0.,
            1.0,
            Rc::new(FakeMass::new(
                100.0,
                vec![24., 25., 25., 26.,],
            )),
            Rc::new(Displacement::new(frame_area_data,).unwrap()), 
            Rc::new(Bounds::from_n(ship_length, 4).unwrap()),
        ).value().unwrap();
        let target_mean_draught = 0.8;
        let target_trim = 0.06;

        assert!(
            (result.0 - target_mean_draught).abs() < result.0.abs() * 0.001, 
            "\ntrim: result: {:?}\n target_mean_draught: {:?}",
            result.0,
            target_mean_draught
        );

        assert!(
            (result.1 - target_trim).abs() < result.1.abs() * 0.001, 
            "\ntrim: result: {:?}\n target_trim: {:?}",
            result.1,
            target_trim
        );

        test_duration.exit();
    }
}
