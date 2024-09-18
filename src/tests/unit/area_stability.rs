#[cfg(test)]

mod tests {
    use crate::area::HAreaStability;
    use crate::icing_timber::{IcingTimberBound, IcingTimberType};
    use crate::stability::{Area, IArea};
    use crate::{Bound, Desk, Moment, Position};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::sync::Arc;
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static INIT: Once = Once::new();

    unsafe impl Sync for Area {} //for static
    static mut AREA: Option<Arc<dyn IArea>> = None;

    static SHIP_LENGTH: f64 = 10.;
    static SHIP_WIDTH: f64 = 2.;
    static SHIP_HEIGHT: f64 = 2.;
    static SHIP_AREA_V: f64 = 5.;
    static SHIP_AREA_H: f64 = 10.;
    static MVX_CS_DMIN1: f64 = 10.;
    static MVZ_CS_DMIN1: f64 = 20.;
    static DESC_MASS: f64 = 3.;
    static DESC_LENGTH: f64 = 6.;
    static DESC_WIDTH: f64 = 2.;
    static DESC_HEIGHT: f64 = 1.;

    fn init_once() {
        INIT.call_once(|| unsafe {
            AREA.replace(Arc::new(Area::new(
                SHIP_AREA_V,
                MVX_CS_DMIN1,
                MVZ_CS_DMIN1,
                vec![HAreaStability::new(
                    SHIP_AREA_H,
                    Position::new(0., 0., SHIP_HEIGHT),
                )],
                Rc::new(vec![Rc::new(Desk::new(
                    DESC_MASS,
                    Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                    Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                    Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                    DESC_LENGTH * DESC_HEIGHT,
                    Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                    DESC_LENGTH * DESC_WIDTH,
                    true,
                ))]),
                IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::Full),
            )));
        })
    }

    #[test]
    fn area_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test area stability area_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { AREA.as_ref().unwrap().area_v().unwrap() };
        let target = SHIP_AREA_V + DESC_LENGTH * DESC_HEIGHT;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn moment_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test area stability moment_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { AREA.as_ref().unwrap().moment_v().unwrap() };
        let target = Moment::new(MVX_CS_DMIN1, 0., MVZ_CS_DMIN1)
            + Moment::from_pos(
                Position::new(0., 0., DESC_HEIGHT / 2. + SHIP_HEIGHT),
                DESC_LENGTH * DESC_HEIGHT,
            );
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn moment_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test area stability moment_h";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { AREA.as_ref().unwrap().moment_h().unwrap() };
        let target = Moment::from_pos(Position::new(0., 0., SHIP_HEIGHT), SHIP_AREA_H)
            + Moment::from_pos(
                Position::new(0., 0., DESC_HEIGHT),
                DESC_LENGTH * DESC_WIDTH,
            );
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn moment_timber_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test area stability moment_timber_h";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { AREA.as_ref().unwrap().moment_timber_h().unwrap() };
        let target = Moment::from_pos(
            Position::new(0., 0., DESC_HEIGHT + SHIP_HEIGHT),
            DESC_LENGTH * DESC_WIDTH,
        );
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn delta_moment_timber_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!();
        let self_id = "test area stability delta_moment_timber_h";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { AREA.as_ref().unwrap().delta_moment_timber_h().unwrap() };
        let target = Moment::from_pos(Position::new(0., 0., DESC_HEIGHT), DESC_LENGTH * DESC_WIDTH);
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
