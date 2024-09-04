#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{f64::consts::PI, rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        data::structs::ScrewParsedData, draught::FakeDraught, Screw, IParameters,
        ParameterID, Parameters, Position,
    };

    #[test]
    fn screw_zero() {
        // дифферент 0, крен 0
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw zero";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 50.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 50.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn screw_roll_right() {
        // дифферент 0, крен 10
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw roll right";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 10.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 50. - 50. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 50. + 50. * (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn screw_roll_left() {
        // дифферент 0, крен 10
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw roll left";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, -10.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., 0.)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 50. + 50. * (10. * PI / 180.).sin();
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 50. - 50. * (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn screw_pitch_bow() {
        // дифферент в нос
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw pitch bow";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., 0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 0.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 0.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn screw_pitch_stern() {
        // дифферент в корму
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw pitch stern";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, 0.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 100.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 100.;
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
    #[test]
    fn screw_1_stern() {
        // дифферент в корму, крен -10 градусов
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test Screw";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let mut data = Vec::new();
        data.push(ScrewParsedData {
            name: "left_screw".to_owned(),
            pos: Position::new(-10., -1., 2.),
            d: 1.,
        });
        data.push(ScrewParsedData {
            name: "rigth_screw".to_owned(),
            pos: Position::new(-10., 1., 2.),
            d: 1.,
        });
        let parameters = Parameters::new();
        parameters.add(ParameterID::Roll, -10.);
        let result = Screw::new(
            Box::new(FakeDraught::new(2., -0.1)),
            data,
            Rc::new(parameters),
        )
        .calculate()
        .unwrap();
        let target = 100.;
        assert!(
            (result[0].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        let target = 100. - 50. * (10. * PI / 180.).sin();
        assert!(
            (result[1].1 - target).abs() < 0.0001,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );
        test_duration.exit();
    }
}
