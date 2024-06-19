#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{mass::*, math::*, strength::trim::*, Displacement, Frame};

    #[test]
    fn trim() {
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

        let result = Trim::new(
            1.025,   
            0.,
            1.,
            Rc::new(FakeMass::new(
                    100.0,
                    vec![21., 25., 25., 29.,],
                    Position::new(0., 0., 0.),
                    Position::new(0., 0., 0.,),
            )),
            Rc::new(Displacement::new(frames,)), 
            Rc::new(Bounds::from_n(20., 4)),
        ).value();
        let target = 0.1956;

        assert!(
            (result - target).abs() < result.abs() * 0.01, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
