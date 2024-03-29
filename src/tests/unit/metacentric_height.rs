#[cfg(test)]

mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{math::{DeltaMH, Position, SurfaceMoment}, stability::metacentric_height::MetacentricHeight, strength::mass::FakeMass};

    static INIT: Once = Once::new();

    unsafe impl Sync for MetacentricHeight {} //for static
    static mut HEIGHT: Option<MetacentricHeight> = None;

    fn init_once() {
        INIT.call_once(|| {
            let mass = Rc::new(FakeMass::new(
                2044.10,
                vec![0.],
                Position::new(1.05, 0., 5.32),
                DeltaMH::new(0., 0.),
                Position::new(0., 0., 0.,), 
                SurfaceMoment::new(0., 0.,),
            ));

            unsafe {
                HEIGHT.replace(MetacentricHeight::new(
                    Position::new(-0.194609657, 0., 0.735524704),
                    696.702572991,
                    100.,
                    mass,               
                ));
            }
        })
    }

    #[test]
    #[ignore = "TODO"]
    fn h_long() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test MetacentricHeight h_long";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        //TODO

        test_duration.exit();
    }

    #[test]
    #[ignore = "TODO"]
    fn h_cross() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test MetacentricHeight h_cross";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        //TODO

        test_duration.exit();
    }

    #[test]
    #[ignore = "TODO"]
    fn z_g_fix() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test MetacentricHeight z_g_fix";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        //TODO

        test_duration.exit();
    }
}
