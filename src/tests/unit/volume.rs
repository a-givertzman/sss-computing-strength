#[cfg(test)]

mod tests {
    use std::time::Duration;
    use std::rc::Rc;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{draught::FakeDraught, Bounds, Curve, Displacement, Frame, IVolume, Volume};
    
    #[test]
    fn volume() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Volume";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let frames = vec![
            Frame::new(-2., Curve::new_linear(&vec![(0., 1.), (1., 1.)])),
            Frame::new(-1., Curve::new_linear(&vec![(0., 2.), (1., 2.)])),
            Frame::new(0., Curve::new_linear(&vec![(0., 2.), (1., 2.)])),
            Frame::new(1., Curve::new_linear(&vec![(0., 2.), (1., 2.)])),
            Frame::new(2., Curve::new_linear(&vec![(0., 1.), (1., 1.)])),
        ];

        let result = Volume::new(
            Rc::new(Displacement::new(frames)),
            Box::new(FakeDraught::new(1., 0.)),
            Rc::new(Bounds::from_n(4., 4)),
        ).values();

        let target = vec![2., 2., 2., 2.,];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}