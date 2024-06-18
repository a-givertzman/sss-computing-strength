#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, stability::circulation::*, FakeLeverDiagram, FakeMass};

    #[test]
    fn circulation() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Circulation heel_lever";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Circulation::new(
            10.,
            40.,
            4.,
            Rc::new(FakeMass::new(
                1000.,
                vec![1000.],
                Position::new(0., 0., 0.),
                Moment::new(0., 0., 0.),
            )),
            Rc::new(FakeLeverDiagram::new(
                vec![0.],
                1.,
                0.5,
                1.,
                vec![(30., 60.)],
            )),
        )
        .heel_lever(10.);

        let target = 1.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
