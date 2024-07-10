#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, strength::trim::*, Displacement, FakeMass, Frame};

    #[test]
    fn trim_strength() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Trim";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let frames = vec![
            Frame::new(-10., Curve::new_linear(&vec![(0., 0.), (2., 40.)])),
            Frame::new(10., Curve::new_linear(&vec![(0., 0.), (2., 40.)])),
        ];

        let ship_length = 20.;
        let result = Trim::new(
            ship_length,
            1.025,   
            0.,
            0.3,
            Rc::new(FakeMass::new(
                100.0,
                vec![24., 25., 25., 26.,],
            )),
            Rc::new(Displacement::new(frames,)), 
            Rc::new(Bounds::from_n(ship_length, 4)),
        ).value();
        let target = 0.0225;

        assert!(
            (result - target).abs() < result.abs() * 0.001, 
            "\ntrim: result: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
