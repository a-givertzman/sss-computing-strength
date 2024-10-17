#[cfg(test)]

mod tests {
    use crate::area::{HAreaStrength, VerticalArea};
    use crate::icing_timber::{IcingTimberBound, IcingTimberType};
    use crate::strength::{Area, IArea};
    use crate::{Bound, Desk, Position};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    static SHIP_LENGTH: f64 = 10.;
    static SHIP_WIDTH: f64 = 2.;
    static SHIP_HEIGHT: f64 = 2.;
    static SHIP_AREA_V: f64 = 5.;
    static SHIP_AREA_H: f64 = 10.;
    static DESC_MASS: f64 = 3.;
    static DESC_LENGTH: f64 = 6.;
    static DESC_WIDTH: f64 = 2.;
    static DESC_HEIGHT: f64 = 1.;

    #[test]
    fn area_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., 0.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::Full),
        );

        // full
        let result = area.area_v(&Bound::Full).unwrap();
        let target = SHIP_AREA_V + DESC_LENGTH * DESC_HEIGHT;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // half
        let result = area
            .area_v(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = (SHIP_AREA_V + DESC_LENGTH * DESC_HEIGHT) * 0.5;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn area_h() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_h";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., DESC_HEIGHT / 2.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::Full),
        );

        // full
        let result = area.area_h(&Bound::Full).unwrap();
        let target = SHIP_AREA_H;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // half
        let result = area
            .area_h(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = (SHIP_AREA_H) * 0.5;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn area_timber_h_full() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_timber_h_full";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., DESC_HEIGHT / 2.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::Full),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap();
        let target = DESC_LENGTH * DESC_WIDTH;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // half
        let result = area
            .area_timber_h(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = (DESC_LENGTH * DESC_WIDTH) * 0.5;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn area_timber_h_half_left() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_timber_h_half_left";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., DESC_HEIGHT / 2.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::HalfLeft),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap();
        let target = DESC_LENGTH * DESC_WIDTH / 2.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // half
        let result = area
            .area_timber_h(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = DESC_LENGTH * DESC_WIDTH / 4.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn area_timber_h_half_right() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_timber_h_half_right";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., DESC_HEIGHT / 2.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::HalfRight),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap();
        let target = DESC_LENGTH * DESC_WIDTH / 2.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // half
        let result = area
            .area_timber_h(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = DESC_LENGTH * DESC_WIDTH / 4.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn area_timber_h_bow() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_timber_h_bow";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let area = Area::new(
            vec![VerticalArea::new(
                SHIP_AREA_V,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            vec![HAreaStrength::new(
                SHIP_AREA_H,
                Bound::new(-SHIP_LENGTH / 2., SHIP_LENGTH / 2.).unwrap(),
            )],
            Rc::new(vec![Rc::new(Desk::new(
                DESC_MASS,
                Position::new(0., 0., DESC_HEIGHT / 2.),
                Bound::new(-DESC_LENGTH / 2., DESC_LENGTH / 2.).unwrap(),
                Bound::new(-DESC_WIDTH / 2., DESC_WIDTH / 2.).unwrap(),
                Bound::new(SHIP_HEIGHT, SHIP_HEIGHT + DESC_HEIGHT).unwrap(),
                DESC_LENGTH * DESC_HEIGHT,
                Position::new(0., 0., SHIP_HEIGHT + DESC_HEIGHT / 2.),
                DESC_LENGTH * DESC_WIDTH,
                true,
            ))]),
            IcingTimberBound::new(SHIP_WIDTH, SHIP_LENGTH, IcingTimberType::Bow),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap();
        let target = DESC_WIDTH * (3. - 10. / 6.);
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // back part of ship
        let result = area
            .area_timber_h(&Bound::new(-SHIP_LENGTH / 2., 0.).unwrap())
            .unwrap();
        let target = 0.;
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        // forward part of ship
        let result = area
            .area_timber_h(&Bound::new(0., SHIP_LENGTH / 2.).unwrap())
            .unwrap();
        let target = DESC_WIDTH * (3. - 10. / 6.);
        assert!(
            (result - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
}
