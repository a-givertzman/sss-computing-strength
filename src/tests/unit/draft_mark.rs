#[cfg(test)]

mod tests {
    use std::{collections::HashMap, f64::consts::PI, rc::Rc, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{draught::FakeDraught, DraftMark, IParameters, ParameterID, Parameters};

    #[test]
    fn draft_mark_zero() {// дифферент 0, крен 0, сверяем марки осадки нос, корма, мидель
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        points.insert("stern".to_owned(), vec![(-10., 1., 1.,), (-10., 1., 2.,), (-10., 1., 3.,),]);
        points.insert("mid".to_owned(), vec![(0., 1., 1.,), (0., 1., 2.,), (0., 1., 3.,),]);
        points.insert("bow".to_owned(), vec![(10., 1., 1.,), (10., 1., 2.,), (10., 1., 3.,),]);
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., 0.)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 2.;
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[1].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[2].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    #[test]
    fn draft_mark_roll_right() {// дифферент 0, крен 10, сверяем марки осадки нос, корма, мидель правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        points.insert("stern".to_owned(), vec![(-10., 2., 1.,), (-10., 2., 2.,), (-10., 2., 3.,),]);
        points.insert("mid".to_owned(), vec![(0., 2., 1.,), (0., 2., 2.,), (0., 2., 3.,),]);
        points.insert("bow".to_owned(), vec![(10., 2., 1.,), (10., 2., 2.,), (10., 2., 3.,),]);
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., 0.)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 2. + 2.*(10.*PI/180.).sin();
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[1].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[2].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    #[test]
    fn draft_mark_roll_left() {// дифферент 0, крен 10, сверяем марки осадки нос, корма, мидель левый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        points.insert("stern".to_owned(), vec![(-10., -2., 1.,), (-10., -2., 2.,), (-10., -2., 3.,),]);
        points.insert("mid".to_owned(), vec![(0., -2., 1.,), (0., -2., 2.,), (0., -2., 3.,),]);
        points.insert("bow".to_owned(), vec![(10., -2., 1.,), (10., -2., 2.,), (10., -2., 3.,),]);
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., 0.)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 2. - 2.*(10.*PI/180.).sin();
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[1].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        assert!((result[2].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    #[test]
    fn draft_mark_1_bow() { // дифферент в нос, крен 10 градусов, сверяем марку осадки в носу правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        points.insert("bow".to_owned(), vec![(10., 2., 1.,), (10., 2., 2.,), (10., 2., 3.,), (10., 2., 4.,),]);
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., 0.1)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 3. + 2.*(10.*PI/180.).sin();
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    #[test]
    fn draft_mark_2_bow() { // дифферент в корму, крен 10 градусов, сверяем марку осадки в носу правый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        points.insert("bow".to_owned(), vec![(10., 2., 1.,), (10., 2., 2.,), (10., 2., 3.,), (10., 2., 4.,),]);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., -0.1)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 1. + 2.*(10.*PI/180.).sin();
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    #[test]
    fn draft_mark_1_stern() { // дифферент в корму, крен -10 градусов, сверяем марку осадки в корме левый борт
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test DraftMark";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut points = HashMap::new();
        points.insert("stern".to_owned(), vec![(-10., -2., 1.,), (-10., -2., 2.,), (-10., -2., 3.,), (-10., -2., 4.,)]);
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, -10.);
        let result = DraftMark::new(
            Box::new(FakeDraught::new(2., -0.1)),
            points,
            Rc::new(parameters),
        ).calculate().unwrap();
        let target = 3. + 2.*(10.*PI/180.).sin();
        assert!((result[0].1.2 - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}