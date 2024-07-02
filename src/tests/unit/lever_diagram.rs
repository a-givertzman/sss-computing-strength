#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        math::*,
        stability::{lever_diagram::*, metacentric_height::*},
        FakeShipMoment, FakeParameters,
    };

    static INIT: Once = Once::new();

    unsafe impl Sync for LeverDiagram {} //for static
    static mut LEVER_DIAGRAM: Option<LeverDiagram> = None;

    fn init_once() {
        INIT.call_once(|| {
            let mass = Rc::new(FakeShipMoment::new(
                1000.0,
                vec![0.],
                Position::new(0., 2., 0.),
                Position::new(0., 0., 0.),
            ));

            let center_draught_shift = Position::new(0., 1., 0.);

            let metacentric_height: Rc<dyn IMetacentricHeight> =
                Rc::new(FakeMetacentricHeight::new(0., 0., 0., 1.));

            let pantocaren = vec![
                (
                    1.,
                    vec![
                        (0., 0.),
                        (15., 1.),
                        (30., 2.),
                        (45., 3.),
                        (60., 2.),
                        (75., 1.),
                        (90., 0.),
                    ],
                ),
                (
                    10.,
                    vec![
                        (0., 0.),
                        (15., 1.),
                        (30., 2.),
                        (45., 3.),
                        (60., 2.),
                        (75., 1.),
                        (90., 0.),
                    ],
                ),
            ];
            let lever_diagram = LeverDiagram::new(
                mass,
                center_draught_shift,
                Curve2D::from_values_linear(pantocaren),
                2.,
                metacentric_height,
                Rc::new(FakeParameters{}),
            );
            lever_diagram.max_angles();
            unsafe {
                LEVER_DIAGRAM.replace(lever_diagram);
            }
        })
    }

    #[test]
    fn angle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test LeverDiagram angle";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let angle = 30.0;
        let angle_rad = angle * std::f64::consts::PI / 180.;
        let moment = 2. - 1.*angle_rad.sin() - (2.-1.)*angle_rad.cos();
        let result = unsafe { LEVER_DIAGRAM.clone().unwrap().angle(moment) };
        let target = vec![angle, 90. - angle];
        result.iter().zip(target.iter()).for_each(|(r, t)| {
            assert!((r - t).abs() < 0.001, "\nresult: {:?}\ntarget: {:?}", r, t)
        });

        test_duration.exit();
    }

    #[test]
    fn lever_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test LeverDiagram lever_moment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let angle = 30.0;
        let result = unsafe { LEVER_DIAGRAM.clone().unwrap().lever_moment(angle) };
        let angle_rad = angle * std::f64::consts::PI / 180.;
        let target = 2. - 1.*angle_rad.sin() - (2.-1.)*angle_rad.cos();
        assert!((result - target).abs() < 0.001, "\nresult: {:?}\ntarget: {:?}", result, target);

        test_duration.exit();
    }

    #[test]
    fn dso_lever_max() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test LeverDiagram dso_lever_max";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { LEVER_DIAGRAM.clone().unwrap().dso_lever_max(15., 90.,) };
        let angle = 45.0;
        let angle_rad = angle * std::f64::consts::PI / 180.;
        let target = 3. - 1.*angle_rad.sin() - (2.-1.)*angle_rad.cos();
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }

    #[test]
    fn max_angles() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test LeverDiagram max_angles";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { LEVER_DIAGRAM.clone().unwrap().max_angles() };
        let angle_rad = 45.0 * std::f64::consts::PI / 180.;
        let target = vec![(45., 3. - 1.*angle_rad.sin() - (2.-1.)*angle_rad.cos()),];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
