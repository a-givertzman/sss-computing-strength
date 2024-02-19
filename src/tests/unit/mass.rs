#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{rc::Rc, sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{load::{ILoad, LoadSpace}, mass::Mass, math::{bound::Bound, curve::Curve, inertia_shift::InertiaShift, mass_moment::MassMoment, pos_shift::PosShift, position::Position}, tank::Tank};
    
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
    fn delta_m_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Mass delta_m_h";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        
        let center = PosShift::new(Curve::new(vec![(0., 2.), (10., 2.)]),
                                                Curve::new(vec![(0., 0.), (10., 0.)]),
                                                Curve::new(vec![(0., 0.), (10., 0.)]));
        
        let free_surf_inertia = InertiaShift::new(Curve::new(vec![(0., 0.), (10., 1.)]),
                                                                Curve::new(vec![(0., 0.), (10., 1.)]));

        let loads: Vec<Rc<Box<dyn ILoad>>>  = vec![Rc::new(Box::new(Tank::new( 2., 10., Bound::new(-5., 5.), center, free_surf_inertia))),];

        let bounds = vec![  Bound::new(-10., 10.),];

        let result = Mass::new( loads, &bounds).delta_m_h();
        let target = 0.1;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        testDuration.exit();
    }
}