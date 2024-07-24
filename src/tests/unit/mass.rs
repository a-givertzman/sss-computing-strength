#[cfg(test)]

mod tests {
    use crate::math::Bounds;
    use crate::math::position::Position;
    use crate::{load::*, Bound, FakeIcingMass, FakeParameters, FakeResults, FakeWettingMass, IMass, Mass,};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for Mass {} //for static
    static mut MASS: Option<Box<dyn IMass>> = None;

    fn init_once() {
        INIT.call_once(|| {
            let loads_const: Rc<Vec<Rc<LoadMass>>> = Rc::new(vec![
                Rc::new(LoadMass::new(
                    10.,
                    Bound::new(-10., 0.).unwrap(),
                    Some(Position::new(-5., 0., 0.)),     
                    LoadingType::Hull,         
                )),
                Rc::new(LoadMass::new(
                    20.,
                    Bound::new(0., 10.).unwrap(),
                    Some(Position::new(5., 0., 0.)),
                    LoadingType::Hull,  
                )),
            ]);

            let loads_cargo: Rc<Vec<Rc<LoadMass>>> = Rc::new(vec![
                Rc::new(LoadMass::new(
                20.,
                Bound::new(-5., 5.).unwrap(),
                Some(Position::new(0., 0., 0.)),
                LoadingType::Cargo,  
            ))]);

            unsafe {
                MASS.replace(Box::new(Mass::new(
                    loads_const,
                    Rc::new(FakeIcingMass::new(0.,)),
                    Rc::new(FakeWettingMass::new(0.,)),
                    loads_cargo,
                    Rc::new(Bounds::from_n(20., 4).unwrap()),
                    Rc::new(FakeResults{}),
                    Rc::new(FakeParameters{}),
                )));
            }
        })
    }

    #[test]
    fn sum() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test Mass sum";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { MASS.as_ref().unwrap().sum().unwrap() };
        let target = 50.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn values() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test Mass values";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { MASS.as_ref().unwrap().values().unwrap() };
        let target = vec![5., 15., 20., 10.];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
