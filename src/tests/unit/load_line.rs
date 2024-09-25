#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{f64::consts::PI, rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        data::structs::LoadLineParsedData, draught::FakeDraught, LoadLine, IParameters,
        ParameterID, Parameters, Position,
    };

    #[test]
    fn load_line_zero() {
        // дифферент 0, крен 0
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine zero";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 3.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 1.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn load_line_roll_right() {
        // дифферент 0, крен 10
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine roll right";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. - (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. + (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn load_line_roll_left() {
        // дифферент 0, крен -10
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine roll left";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, -10.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. + (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. - (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn load_line_bow() {
        // дифферент в нос, 
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., 0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. - 1.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. + 1.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn load_line_stern() {
        // дифферент в корму
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine stern";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. + 1.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. - 1.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn load_line_roll_bow() {
        // дифферент в нос, крен 10 градусов,
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine roll bow";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., 0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. - 1. - (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. + 1. + (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
   #[test]
    fn load_line_roll_stern() {
        // дифферент в корму, крен 10 градусов
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test LoadLine roll stern";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(LoadLineParsedData {
            name: "left_back_mark".to_owned(),
            pos: Position::new(-10., -1., 2.),
        });
        data.push(LoadLineParsedData {
            name: "rigth_forward_mark".to_owned(),
            pos: Position::new(10., 1., 2.),
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = LoadLine::new(
            Rc::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 2. + 1. - (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 2. - 1. + (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }  
}
