#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{data::structs::{NavigationArea, NavigationAreaData}, stability::wind::*, windage::FakeWindage, FakeMass, Parameters};

    #[test]
    fn wind() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Wind";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = Wind::new(
            NavigationAreaData{area: NavigationArea::R2Rsn45, p_v: 200., m: 0.50},
            Rc::new(FakeWindage::new(1000.,5.)),
            9.81,
            Rc::new(FakeMass::new(
                1000./9.81,
                vec![0.],
            )),
            Rc::new(Parameters::new()),
        )
        .arm_wind_dynamic().unwrap();
        let target = 1.5;

        assert!(
            (result - target).abs() < result.abs() * 0.01, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
