#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{stability::grain::*, Bulk, FakeLeverDiagram, FakeMass, FakeParameters};

    #[test]
    fn grain() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Grain";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Grain::new(
            1.,
            Rc::new(vec![Rc::new(Bulk::new(1. / 1.025, 100.).unwrap())]),
            Rc::new(FakeMass::new(1000., vec![0.])),
            Rc::new(FakeLeverDiagram::new(
                vec![15., 75.],
                1.5,
                vec![(0., 0., 0.)],
                2.,
                30.,
                vec![(1.5, 25.)],
            )),
            Rc::new(FakeParameters {}),
        )
        .area()
        .unwrap();

        let target = 2.;
        assert!(
            (result - target).abs() < target*0.01,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
