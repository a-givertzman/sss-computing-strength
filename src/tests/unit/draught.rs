#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{mass::*, math::*, stability::{metacentric_height::*, trim::*}, FakeParameters};

    #[test]
    fn draught() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Draught";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Draught::new(
            118.39,
            2.4,
            1.,
            Position::new(-0.194609657, 0., 0.735524704),
            Rc::new(FakeMetacentricHeight::new(
                696.702572991,
                    100.,
                    100.,
                    100.,
            )),
            Rc::new(FakeMass::new(
                2044.10,
                vec![0.],
                Position::new(1.05, 0., 5.32),
                Position::new(0., 0., 0.,), 
            )),
            Rc::new(FakeParameters{}),
        )
        .value();
        let target = 0.2115;

        assert!(
            (result - target).abs() < result.abs() * 0.01, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
