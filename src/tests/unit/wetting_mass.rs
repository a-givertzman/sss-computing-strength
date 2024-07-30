#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{Bound, IWettingMass, LoadMass, LoadingType, Position, WettingMass};

    #[test]
    fn wetting_mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test WettingMass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = WettingMass::new(
            0.1,
            Rc::new(vec![Rc::new(
                LoadMass::new(
                    20.,
                    Bound::new(-1., 1.).unwrap(),
                    Some(Position::new(0., 0., 1.)),
                    LoadingType::Cargo,
                )
                .unwrap(),
            )]),
        )
        .mass(&Bound::Value(0., 1.))
        .unwrap();
        let target = 1.;

        assert!(
            (result - target).abs() < result.abs() * 0.001, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
