#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{math::*, stability::{metacentric_height::*, trim::*}, trim::{FakeTrim, ITrim}, FakeMass, FakeParameters, FakeShipMoment};

    #[test]
    fn trim_stability() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Trim";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let ship_length = 118.39;
        let result = Trim::new(
            ship_length,
            1.6562565987303715,
            -0.862,
            Position::new(-0.27987870364183104, 0., 0.843131172736385),
            Rc::new(FakeMetacentricHeight::new(
                616.8739594823264,
                4.994720390444311,
                    4.976147740632071,
                    5.090317945969997,
            )),
            Rc::new(FakeMass::new(
                2354.10,
                vec![0.],
            )),
            Rc::new(FakeShipMoment::new(
                Position::new(1.05, 0., 5.32),
                Position::new(2146.305, 0., 10874.612,), 
            )),
            Rc::new(FakeParameters{}),
        )
        .value();
        let target = FakeTrim::from_angle(-0.23518307533354152, ship_length).value();

        assert!(
            (result - target).abs() < result.abs() * 0.001, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
