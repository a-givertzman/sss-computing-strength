#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{draught::{Draught, IDraught}, trim::FakeTrim};

    #[test]
    fn draught() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Draught";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let ship_length = 118.39;
        let result = Draught::new(
            ship_length,
            1.6562565987303715,
            -0.862,
            Box::new(FakeTrim::from_angle(-0.3013717957692749, ship_length)),
            None,
        )
        .value(59.194);
        let target = 1.34;

        assert!(
            (result - target).abs() < result.abs() * 0.001, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
