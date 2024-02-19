#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{rc::Rc, sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{load::{ILoad, LoadSpace}, mass::Mass, math::{bound::Bound, mass_moment::MassMoment, position::Position}};
    
    #[test]
    fn sum() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Mass sum";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let loads: Vec<Rc<Box<dyn ILoad>>>  = vec![Rc::new(Box::new(LoadSpace::new( Bound::new(-10., 0.), Position::new( -5., 0., 0.), 10.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(0., 10.), Position::new( 5., 0., 0.), 20.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(-5., 5.), Position::new( 0., 0., 0.), 10.))),];

        let bounds = vec![  Bound::new(-10., -5.),
                                        Bound::new(-5., 0.),
                                        Bound::new(0., 5.),
                                        Bound::new(5., 10.),];

        let result = Mass::new( loads, &bounds).sum();
        let target = 40.;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    fn values() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Mass values";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let loads: Vec<Rc<Box<dyn ILoad>>>  = vec![Rc::new(Box::new(LoadSpace::new( Bound::new(-10., 0.), Position::new( -5., 0., 0.), 10.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(0., 10.), Position::new( 5., 0., 0.), 10.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(-5., 5.), Position::new( 0., 0., 0.), 20.))),];

        let bounds = vec![  Bound::new(-10., -5.),
                                        Bound::new(-5., 0.),
                                        Bound::new(0., 5.),
                                        Bound::new(5., 10.),];

        let result = Mass::new( loads, &bounds).values();
        let target = vec![5., 15., 15., 5.];
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }

    #[test]
    fn moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Mass moment";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let loads: Vec<Rc<Box<dyn ILoad>>>  = vec![Rc::new(Box::new(LoadSpace::new( Bound::new(-10., 0.), Position::new( -5., 0., 0.), 10.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(0., 10.), Position::new( 5., 0., 0.), 10.))),
                                                    Rc::new(Box::new(LoadSpace::new( Bound::new(0., 10.), Position::new( 5., 0., 0.), 20.))),];

        let bounds = vec![  Bound::new(-10., -5.),
                                        Bound::new(-5., 0.),
                                        Bound::new(0., 5.),
                                        Bound::new(5., 10.),];

        let result = Mass::new( loads, &bounds).moment();
        let target = MassMoment::new(100., 0., 0.);
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}