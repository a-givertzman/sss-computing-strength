#[cfg(test)]

mod tests {
    use crate::{
        icing::{FakeIcingStab, IIcingMass, IcingMass},
        Moment,
    };

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for IcingMass {} //for static
    static mut ICING: Option<IcingMass> = None;

    fn init_once() {
        INIT.call_once(|| unsafe {
            ICING.replace(IcingMass::new(
                Rc::new(FakeIcingStab::new(0.03, 0.04, 0.015, 0.1, 0.05, 0.2, true)),
                Rc::new(crate::strength::FakeArea::new(50., 50., 50.)),
                Rc::new(crate::stability::FakeArea::new(
                    50.,
                    Moment::new(0., 0., 100.),
                    Moment::new(0., 0., 200.),
                    Moment::new(0., 0., 300.),     
                    Moment::new(0., 0., 100.),  
                )),
            ));
        })
    }

    #[test]
    fn icing_mass() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test IcingMass mass";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { ICING.clone().unwrap().mass(None) };
        let target =  50.*0.04 + 50.*1.05*0.015;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn icing_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test IcingMass moment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { ICING.clone().unwrap().moment() };
        let target = Moment::new(0., 0., 200.*0.03+300.*0.01+100.*0.04+100.*1.05*0.015);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
