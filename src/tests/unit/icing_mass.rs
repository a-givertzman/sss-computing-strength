#[cfg(test)]

mod tests {
    use crate::icing::{FakeIcingStab, IcingMass};
    use crate::math::Bounds;
    use crate::math::{
        bound::Bound, curve::Curve, inertia::inertia_shift::InertiaShift, pos_shift::PosShift,
        position::Position,
    };

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for IcingMass {} //for static
    static mut ICING: Option<IcingMass> = None;

    fn init_once() {
        INIT.call_once(|| {          
            unsafe {
                ICING.replace(IcingMass::new(
                    FakeIcingStab::new(0.3, 0.15),
                    area_h,
                    area_v: Vec<Area>,    
                    loads_cargo,
                ));
            }
        })
    }

    #[test]
    #[ignore = "TODO"]
    fn icing() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test Icing";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { ICING.clone().unwrap().sum() };
        let target = 50.;
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
