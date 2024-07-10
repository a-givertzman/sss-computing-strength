#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::*, strength::{total_force::*, volume::*}, FakeMass};

    #[test]
    fn total_force() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test TotalForce";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let gravity_g = 9.81;
        let result = TotalForce::new(
            Rc::new(FakeMass::new(
                30.,
                vec![20.; 10],
            )),
            1.0,
            FakeVolume::new(vec![5., 25., 25., 25., 25., 25., 25., 25., 15., 5.]),
            gravity_g,
        )
        .values();
        let mut target = Vec::from([15., -5., -5., -5., -5., -5., -5., -5., 5., 15.]);
        target.mul_single(gravity_g);

        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}
