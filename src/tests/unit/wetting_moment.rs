#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{Bound, IWettingMoment, LoadMass, LoadingType, Moment, Position, WettingMoment};

    #[test]
    fn wetting_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test WettingMoment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = WettingMoment::new(
            0.1,
            Rc::new(vec![Rc::new(
                LoadMass::new(
                    10.,
                    Bound::new(-1., 1.).unwrap(),
                    Some(Position::new(0., 0., 1.)),
                    LoadingType::Cargo,
                )
                .unwrap(),
            )]),
        )
        .moment();
        let target = Moment::from_pos(Position::new(0., 0., 1.), 1.);

        assert!(
            (result.clone() - target.clone()).len() < 0.001, //TODO
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
