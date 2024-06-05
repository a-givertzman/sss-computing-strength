#[cfg(test)]

mod tests {
    use crate::{
        math::{bound::Bound, moment::Moment, position::Position},
        load::*,
    };
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::time::Duration;
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Load mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = LoadMass::new(
            20., 
            Bound::new(-1., 3.),
            Some(Position::new(1., 0., 0.)), 
            LoadType::None,
        ).value(Some(Bound::new(1., 3.)));
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
        println!("");
        let self_id = "test Load moment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = LoadMass::new(
            20.,
            Bound::new(-1., 3.), 
            Some(Position::new(1., 0., 0.)), 
            LoadType::None,
        ).moment();
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
