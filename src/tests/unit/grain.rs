#[cfg(test)]

mod tests {
    use std::{rc::Rc, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, stability::grain::*};

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
            30.,
            Rc::new(FakeBulk::new()),
            Rc::new(FakeMass::new(
                2044.10,
                vec![0.],
                Position::new(1.05, 0., 5.32),
                Position::new(0., 0., 0.,), 
            )),  
            Rc::new(FakeLeverDiagram::new(
                angle: Vec<f64>,
                lever_moment: f64,
                vec![0.],
                dso_area: f64,
                vec![0.],
                theta_max: f64,
                theta_last: f64,
                vec![0.],
            )), 
        ).area();

        let target = 0.3/0.0105;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}