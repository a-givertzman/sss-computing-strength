#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        math::*,
        stability::{rolling_amplitude::*, rolling_period::*},
        FakeMetacentricHeight,
    };

    #[test]
    fn rolling_amplitude() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test RollingAmplitude";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = RollingAmplitude::new(
            Some(1.),
            Rc::new(FakeMetacentricHeight::new(0., 0., 0., 0.1)),
            18.,
            10.,
            2.,
            2., 
            1.,
            Rc::new(Curve::new_linear(&vec![(0., 0.7), (1., 0.7)]).unwrap()),
            Rc::new(Curve::new_linear(&vec![(0., 0.8), (1., 0.8)]).unwrap()),
            Rc::new(Curve::new_linear(&vec![(0., 1.), (1., 1.)]).unwrap()),
            Rc::new(Curve::new_linear(&vec![(0., 0.1), (10., 0.1)]).unwrap()),
            Rc::new(FakeRollingPeriod::new(5., 1.)),
        )
        .unwrap()
        .calculate()
        .unwrap()
        .1;

        let target = 8.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
