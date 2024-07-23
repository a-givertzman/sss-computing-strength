#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{icing_stab::FakeIcingStab, math::*, windage::*};

    static INIT: Once = Once::new();

    unsafe impl Sync for Windage {} //for static
    static mut WINDAGE: Option<Windage> = None;

    fn init_once() {
        INIT.call_once(|| {
            unsafe {
                WINDAGE.replace(Windage::new(
                    Rc::new(FakeIcingStab::new(
                        0.03,
                        0.03,
                        0.015,
                        0.1,
                        0.2,
                        0.3,
                        true,
                    )),
                    Rc::new(crate::stability::FakeArea::new(
                        1000.,
                        Moment::from_pos(Position::new(0., 0., 4.,), 1000.),
                        Moment::from_pos(Position::new(0., 0., 2.,), 500.),
                        Moment::from_pos(Position::new(0., 0., 4.,), 500.),
                        Moment::from_pos(Position::new(0., 0., 2.,), 500.),
                    )),
                    500.,
                    Moment::from_pos(Position::new(0., 0., 1.,), 500.),
                    1.,
                ));
            }
        })
    }

    #[test]
    fn a_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Windage a_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { WINDAGE.clone().unwrap().a_v().unwrap() };
        let target = 600.; //1000.*(1.+0.1) - 500
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn z_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Windage z_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { WINDAGE.clone().unwrap().z_v().unwrap() };
        let target = 6.833333333333333; // (1000.*4.*(1.+0.3) - 500.*1.)/600. - 1.
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
