#[cfg(test)]

mod tests {
    use std::time::Duration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{load::{ILoad, LoadSpace}, math::{bound::Bound, mass_moment::MassMoment, position::Position}};
    
    #[test]
    fn mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test Load mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = LoadSpace::new( 20., Bound::new(-1., 3.), Position::new( 1., 0., 0.)).mass(Some(Bound::new(1., 3.)));
        let target = 10.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let result = LoadSpace::new( 20., Bound::new(-1., 3.), Position::new( 1., 0., 0.),).moment_mass();
        let target = MassMoment::new(20., 0., 0.);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }
}