#[cfg(test)]

mod tests {
    use crate::area::{HAreaStrength, VerticalArea};
    use crate::icing_timber::{IcingTimberBound, IcingTimberType};
    use crate::strength::{Area, IArea};
    use crate::{Bound, Desk, Position};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    #[test]
    fn area_v() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!();
        let self_id = "test area strength area_v";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::Full,
            ),
        );

        // full
        let result = area.area_v(&Bound::Full).unwrap(); 
        let target = ship_area_v + desc_length*desc_height;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // half
        let result = area.area_v(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = (ship_area_v + desc_length*desc_height)*0.5;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::Full,
            ),
        );

        // full
        let result = area.area_h(&Bound::Full).unwrap(); 
        let target = ship_area_h;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // half
        let result = area.area_h(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = (ship_area_h)*0.5;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::Full,
            ),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap(); 
        let target = desc_length*desc_width;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // half
        let result = area.area_timber_h(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = (desc_length*desc_width)*0.5;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::HalfLeft,
            ),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap(); 
        let target = desc_length*desc_width/2.;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // half
        let result = area.area_timber_h(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = desc_length*desc_width/4.;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::HalfRight,
            ),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap(); 
        let target = desc_length*desc_width/2.;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // half
        let result = area.area_timber_h(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = desc_length*desc_width/4.;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);

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

        let ship_length = 10.;
        let ship_width = 2.;   
        let ship_area_v = 5.;
        let ship_area_h = 10.;  
        let desc_mass = 3.;
        let desc_length = 6.;
        let desc_width = 2.;
        let desc_height = 1.;

        let area = Area::new(
            vec![VerticalArea::new(ship_area_v, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            vec![HAreaStrength::new(ship_area_h, Bound::new(-ship_length/2., ship_length/2.,).unwrap()),],
            Rc::new(vec![Rc::new(Desk::new(
                desc_mass,
                Position::new(0., 0., 0.),
                Bound::new(-desc_length/2., desc_length/2.,).unwrap(),
                Bound::new(-desc_width/2., desc_width/2.,).unwrap(),
                desc_length*desc_height,
                Position::new(0., 0., desc_height/2.),
                desc_length*desc_width,
                true,
            )),],),
            IcingTimberBound::new(
                ship_width,
                ship_length,
                IcingTimberType::Bow,
            ),
        );

        // full
        let result = area.area_timber_h(&Bound::Full).unwrap(); 
        let target = desc_width*(3. - 10./6.);
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // back part of ship
        let result = area.area_timber_h(&Bound::new(-ship_length/2., 0.,).unwrap()).unwrap(); 
        let target = 0.;
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        // forward part of ship
        let result = area.area_timber_h(&Bound::new( 0., ship_length/2.,).unwrap()).unwrap(); 
        let target = desc_width*(3. - 10./6.);
        assert!((result - target).abs() < 0.0001, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}
