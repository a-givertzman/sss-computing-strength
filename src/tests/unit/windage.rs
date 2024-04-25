#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, windage::*, FakeArea, ILoad, LoadSpace};

    static INIT: Once = Once::new();

    unsafe impl Sync for Windage {} //for static
    static mut WINDAGE: Option<Windage> = None;

    fn init_once() {
        INIT.call_once(|| {
            unsafe {
                WINDAGE.replace(Windage::new(
                    Rc::new(FakeIcingStab::new()),
                    Rc::new(FakeArea::new()),
                    1000.,
                     Position::new(
                        0.,
                        0.,
                        2.,
                    ),
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

        let result = unsafe { WINDAGE.clone().unwrap().a_v() };
        let target = 1055.;
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

        let result = unsafe { WINDAGE.clone().unwrap().z_v() };
        let target = 2220./1055. - 1.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
