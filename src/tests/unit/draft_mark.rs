#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{f64::consts::PI, rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        data::structs::DraftMarkParsedData, draught::FakeDraught, DraftMark, IParameters,
        ParameterID, Parameters, Position,
    };

    #[test]
    fn draft_mark_zero() {
        // дифферент 0, крен 0, сверяем марки осадки нос, корма, мидель
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark zero";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(DraftMarkParsedData {
            name: "stern".to_owned(),
            data: vec![
                Position::new(-10., 1., 1.),
                Position::new(-10., 1., 2.),
                Position::new(-10., 1., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "mid".to_owned(),
            data: vec![
                Position::new(0., 1., 1.),
                Position::new(0., 1., 2.),
                Position::new(0., 1., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "bow".to_owned(),
            data: vec![
                Position::new(10., 1., 1.),
                Position::new(10., 1., 2.),
                Position::new(10., 1., 3.),
            ],
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2.;
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[1].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[2].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn draft_mark_roll_right() {
        // дифферент 0, крен 10, сверяем марки осадки нос, корма, мидель правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark roll right";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(DraftMarkParsedData {
            name: "stern".to_owned(),
            data: vec![
                Position::new(-10., 2., 1.),
                Position::new(-10., 2., 2.),
                Position::new(-10., 2., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "mid".to_owned(),
            data: vec![
                Position::new(0., 2., 1.),
                Position::new(0., 2., 2.),
                Position::new(0., 2., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "bow".to_owned(),
            data: vec![
                Position::new(10., 2., 1.),
                Position::new(10., 2., 2.),
                Position::new(10., 2., 3.),
            ],
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. + 2. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[1].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[2].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn draft_mark_roll_left() {
        // дифферент 0, крен 10, сверяем марки осадки нос, корма, мидель левый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark roll left";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(DraftMarkParsedData {
            name: "stern".to_owned(),
            data: vec![
                Position::new(-10., -2., 1.),
                Position::new(-10., -2., 2.),
                Position::new(-10., -2., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "mid".to_owned(),
            data: vec![
                Position::new(0., -2., 1.),
                Position::new(0., -2., 2.),
                Position::new(0., -2., 3.),
            ],
        });
        data.push(DraftMarkParsedData {
            name: "bow".to_owned(),
            data: vec![
                Position::new(10., -2., 1.),
                Position::new(10., -2., 2.),
                Position::new(10., -2., 3.),
            ],
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. - 2. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[1].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        assert!(
            (result[2].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn draft_mark_1_bow() {
        // дифферент в нос, крен 10 градусов, сверяем марку осадки в носу правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark bow";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(DraftMarkParsedData {
            name: "bow".to_owned(),
            data: vec![
                Position::new(10., 2., 1.),
                Position::new(10., 2., 2.),
                Position::new(10., 2., 3.),
                Position::new(10., 2., 4.),
            ],
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., 0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 3. + 2. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn draft_mark_stern() {
        // дифферент в корму, крен 10 градусов, сверяем марку осадки в носу правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark stern";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        data.push(DraftMarkParsedData {
            name: "bow".to_owned(),
            data: vec![
                Position::new(10., 2., 1.),
                Position::new(10., 2., 2.),
                Position::new(10., 2., 3.),
                Position::new(10., 2., 4.),
            ],
        });
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 1. + 2. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn draft_mark_roll_stern() {
        // дифферент в корму, крен -10 градусов, сверяем марку осадки в корме левый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark roll stern";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(DraftMarkParsedData {
            name: "stern".to_owned(),
            data: vec![
                Position::new(-10., -2., 1.),
                Position::new(-10., -2., 2.),
                Position::new(-10., -2., 3.),
                Position::new(-10., -2., 4.),
            ],
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, -10.);
        let result = DraftMark::new(
            Rc::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 3. + 2. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 .2 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
}
