#[cfg(test)]

mod tests {
    use crate::{
        load::*,
        math::{bound::Bound, moment::Moment, position::Position},
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Load mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = LoadMass::new(
            20.,
            Bound::new(-1., 3.).unwrap(),
            Some(Position::new(1., 0., 0.)),
            LoadingType::Ballast,
        )
        .unwrap()
        .value(&Bound::new(1., 3.).unwrap())
        .unwrap();
        let target = 10.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Load moment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = LoadMass::new(
            20.,
            Bound::new(-1., 3.).unwrap(),
            Some(Position::new(1., 0., 0.)),
            LoadingType::Ballast,
        )
        .unwrap()
        .moment();
        let target = Moment::new(20., 0., 0.);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
