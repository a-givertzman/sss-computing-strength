#[cfg(test)]

mod tests {
    use crate::{icing_stab::FakeIcingStab, IIcingMoment, IcingMoment, Moment};

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn icing_moment() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test IcingMass moment";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let (area_v, moment_v, moment_h, moment_timber_h, delta_moment_timber_h) =
            (50., 0., 200., 300., 100.);
        let result = IcingMoment::new(
            Rc::new(FakeIcingStab::new(0.03, 0.04, 0.015, 0.1, 0.05, 0.2, true)),
            Rc::new(crate::stability::FakeArea::new(
                area_v,
                Moment::new(0., 0., moment_v),
                Moment::new(0., 0., moment_h),
                Moment::new(0., 0., moment_timber_h),
                Moment::new(0., 0., delta_moment_timber_h),
            )),
        )
        .moment()
        .unwrap();
        let target = Moment::new(
            0.,
            0.,
            moment_v * (1. + 0.05) * 0.015
                + moment_h * 0.03
                + moment_timber_h * 0.01
                + delta_moment_timber_h * 0.04,
        );
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
