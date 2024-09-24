/// Комплексный тест отход море 100% запасов

#[cfg(test)]
mod tests {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{collections::HashMap, rc::Rc, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        area::{HAreaStability, HAreaStrength, VerticalArea},
        draught::Draught,
        icing_stab::{IIcingStab, IcingStab},
        icing_timber::IcingTimberBound,
        stability, strength,
        tests::unit::complex::input_data,
        windage::Windage,
        Acceleration, Bound, Bounds, Circulation, Computer, CriterionID, CriterionStability, Curve,
        Displacement, DraftMark, Grain, ICurve, IGrain, ILeverDiagram, IMetacentricHeight,
        IParameters, IPosShift, IResults, IRollingAmplitude, IRollingPeriod, IWind, LeverDiagram,
        Loads, MetacentricHeight, Moment, ParameterID, Parameters, PosShift, Position, Results,
        RollingAmplitude, RollingPeriod, Stability, WettingMass, WettingMoment, Wind,
    };

    #[test]
    fn complex_66() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let self_id = "test complex";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(60));
        test_duration.run().unwrap();

        let precision = 1. / 20.; // 5%
        let data = input_data::input_data_66();
        let results: Rc<dyn IResults> = Rc::new(Results::new());
        let parameters: Rc<dyn IParameters> = Rc::new(Parameters::new());
        // ускорение свободного падения
        let gravity_g = 9.81;
        // вектор разбиения судна на отрезки
        let bounds = Rc::new(Bounds::from_frames(&data.bounds).unwrap());
        // коллекция различных типов грузов
        let mut loads = Loads::new(
            &data.load_constants,
            Position::new(
                data.const_mass_shift_x,
                data.const_mass_shift_y,
                data.const_mass_shift_z,
            ),
            &data.cargoes,
            &data.compartments,
        );
        // параметры обледенения поверхностей судна
        let icing_stab: Rc<dyn IIcingStab> = Rc::new(IcingStab::new(
            data.icing_stab,
            data.icing_m_timber,
            data.icing_m_v_full,
            data.icing_m_v_half,
            data.icing_m_h_full,
            data.icing_m_h_half,
            data.icing_coef_v_area_full,
            data.icing_coef_v_area_half,
            data.icing_coef_v_area_zero,
            data.icing_coef_v_moment_full,
            data.icing_coef_v_moment_half,
            data.icing_coef_v_moment_zero,
        ));
        // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
        let mut area_const_v = Vec::new();
        for v in data.area_v_str.iter() {
            area_const_v.push(VerticalArea::new(
                v.value,
                Bound::new(v.bound_x1, v.bound_x2).unwrap(),
            ));
        }
        let mut area_const_h = Vec::new();
        for v in data.area_h_str.iter() {
            area_const_h.push(HAreaStrength::new(
                v.value,
                Bound::new(v.bound_x1, v.bound_x2).unwrap(),
            ));
        }
        let ship_mass: Rc<dyn strength::IMass> = Rc::new(strength::Mass::new(
            loads.loads_const().unwrap(),
            Rc::new(strength::IcingMass::new(
                Rc::clone(&icing_stab),
                Rc::new(crate::strength::Area::new(
                    area_const_v,
                    area_const_h,
                    loads.desks().unwrap(),
                    IcingTimberBound::new(data.width, data.length_loa, data.icing_timber_stab),
                )),
            )),
            Rc::new(WettingMass::new(
                data.wetting_timber,
                loads.load_timber().unwrap(),
            )),
            loads.load_variable().unwrap(),
            Rc::clone(&bounds),
            Rc::clone(&results),
            Rc::clone(&parameters),
        ));
        // Объемное водоизмещение (1)
        let volume = ship_mass.sum().unwrap() / data.water_density;
        // Средняя осадка
        let mean_draught = Curve::new_linear(&data.mean_draught)
            .unwrap()
            .value(volume)
            .unwrap();
        parameters.add(ParameterID::DraughtMean, mean_draught);
        // Момент площади горизонтальных поверхностей и площади парусности судна для расчета остойчивости
        let area_stability: Rc<dyn crate::stability::IArea> = Rc::new(crate::stability::Area::new(
            Curve::new_linear(&data.area_v_stab.area())
                .unwrap()
                .value(data.draught_min)
                .unwrap(),
            Curve::new_linear(&data.area_v_stab.moment_x())
                .unwrap()
                .value(data.draught_min)
                .unwrap(),
            Curve::new_linear(&data.area_v_stab.moment_z())
                .unwrap()
                .value(data.draught_min)
                .unwrap(),
            data.area_h_stab
                .iter()
                .map(|v| {
                    HAreaStability::new(v.value, Position::new(v.shift_x, v.shift_y, v.shift_z))
                })
                .collect(),
            loads.desks().unwrap(),
            IcingTimberBound::new(data.width, data.length_loa, data.icing_timber_stab),
        ));
        // Момент массы нагрузки на корпус судна
        let ship_moment: Rc<dyn stability::IShipMoment> = Rc::new(stability::ShipMoment::new(
            Rc::clone(&ship_mass),
            loads.loads_const().unwrap(),
            loads.shift_const(),
            Rc::new(stability::IcingMoment::new(
                Rc::clone(&icing_stab),
                Rc::clone(&area_stability),
            )),
            Rc::new(WettingMoment::new(
                data.wetting_timber,
                loads.load_timber().unwrap(),
            )),
            loads.load_variable().unwrap(),
            Rc::clone(&parameters),
        ));
        // Отстояние центра величины погруженной части судна
        let center_draught_shift = PosShift::new(
            Curve::new_linear(&data.center_draught_shift_x).unwrap(),
            Curve::new_linear(&data.center_draught_shift_y).unwrap(),
            Curve::new_linear(&data.center_draught_shift_z).unwrap(),
        )
        .value(volume)
        .unwrap();
        parameters.add(ParameterID::CenterVolumeZ, center_draught_shift.z());
        // Продольный метацентрические радиус
        let rad_long = Curve::new_linear(&data.rad_long)
            .unwrap()
            .value(volume)
            .unwrap();
        parameters.add(ParameterID::MetacentricLongRad, rad_long);
        // Поперечный метацентрические радиус
        let rad_trans = Curve::new_linear(&data.rad_trans)
            .unwrap()
            .value(volume)
            .unwrap();
        parameters.add(ParameterID::MetacentricTransRad, rad_trans);
        // Отстояние центра тяжести ватерлинии по длине от миделя
        let center_waterline_shift = Curve::new_linear(&data.center_waterline)
            .unwrap()
            .value(volume)
            .unwrap();
        // Площадь ватерлинии
        let area_wl = Curve::new_linear(&data.waterline_area)
            .unwrap()
            .value(volume)
            .unwrap();
        // Число тонн на 1 см осадки
        parameters.add(ParameterID::TonesPerCm, 0.01 * area_wl * data.water_density);
        // Для расчета прочности дифферент находится подбором
        // как условие для схождения изгибающего момента в 0
        Computer::new(
            gravity_g,
            data.water_density,
            data.length_lbp,
            center_waterline_shift,
            mean_draught,
            Rc::clone(&ship_mass),
            Rc::new(Displacement::new(data.frame_area).unwrap()),
            Rc::clone(&bounds),
            Rc::clone(&results),
        )
        .calculate()
        .unwrap();
        /*
        let map_results: HashMap<String, Vec<f64>> =
            results.take_data().into_iter().map(|v| v).collect();

        let mut displacement_result = map_results.get("value_displacement").unwrap().clone();
        displacement_result = displacement_result
            .into_iter()
            .rev()
            .map(|v| v * 1.025)
            .collect(); // переводим м^3 в тонны
                        //displacement_result.remove(0);
        let mut displacement_target = vec![
            23.41, 67.76, 98.35, 107.89, 112.13, 115.65, 119.04, 122.78, 126.55, 130.33, 134.17,
            137.98, 141.71, 145.46, 149.23, 153.00, 156.77, 156.75, 114.95, 40.45,
        ];
        displacement_target.insert(0, displacement_target.iter().sum());

        let mut mass_values_result = map_results.get("value_mass_sum").unwrap().clone();
        mass_values_result = mass_values_result.into_iter().rev().collect();
        // mass_values_result.remove(0);
        let mut mass_values_target = vec![
            139.14, 182.25, 188.06, 137.06, 83.66, 70.46, 70.46, 70.46, 70.46, 70.46, 70.46, 70.46,
            70.46, 70.46, 70.46, 70.46, 70.46, 235.42, 286.06, 257.20,
        ];
        mass_values_target.insert(0, mass_values_target.iter().sum());

        let mut total_force_result = map_results.get("value_total_force").unwrap().clone();
        total_force_result = total_force_result.into_iter().rev().collect();
        // total_force_result.remove(0);
        let total_force_target = vec![
            115.73, 114.50, 89.71, 29.17, -28.47, -45.19, -48.58, -52.32, -56.09, -59.87, -63.71,
            -67.52, -71.25, -75.00, -78.77, -82.54, -86.31, 78.67, 171.10, 216.74,
        ];
        total_force_result.insert(0, total_force_result.iter().sum());

        let mut shear_force_result = map_results.get("value_shear_force").unwrap().clone();
        shear_force_result = shear_force_result.into_iter().rev().collect();
        let shear_force_target = vec![
            0., 1135., 2259., 3139., 3425., 3145., 2702., 2226., 1712., 1162., 575., -50., -713.,
            -1412., -2147., -2920., -3730., -4577., -3805., -2126., 0.,
        ];

        let mut bending_moment_result = map_results.get("value_bending_moment").unwrap().clone();
        bending_moment_result = bending_moment_result.into_iter().rev().collect();
        let bending_moment_target = vec![
            0., 3292., 13135., 28786., 47820., 66874., 83832., 98122., 109541., 117877., 122913.,
            124434., 122222., 116061., 105740., 91045., 71760., 47672., 23366., 6166., 0.,
        ];

        println!("displacement: index: result  target");
        displacement_result
            .iter()
            .zip(displacement_target.iter())
            .enumerate()
            .for_each(|(i, (r, t))| {
                println!("{i}: {r} {t}");
            });
        println!("mass_values: index: result  target");
        mass_values_result
            .iter()
            .zip(mass_values_target.iter())
            .enumerate()
            .for_each(|(i, (r, t))| {
                println!("{i}: {r} {t}");
            });
        println!("total_force: index: result  target");
             total_force_result
                    .iter()
                    .zip(total_force_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        println!("{i}: {r} {t}");
                    });
                println!("shear_force: index: result  target");
                shear_force_result
                    .iter()
                    .zip(shear_force_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        println!("{i}: {r} {t}");
                    });
                println!("bending_moment: index: result  target");
                bending_moment_result
                    .iter()
                    .zip(bending_moment_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        println!("{i}: {r} {t}");
                    });

                displacement_result
                    .iter()
                    .zip(displacement_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        assert!(
                            (r - t).abs() < t.abs()*precision,
                            "\ndisplacement i:{i} result:{r} target:{t}"
                        );
                    });
                mass_values_result
                    .iter()
                    .zip(mass_values_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        assert!(
                            (r - t).abs() < t.abs()*precision,
                            "\nmass_values i:{i} result:{r} target:{t}"
                        );
                    });
               total_force_result
                    .iter()
                    .zip(total_force_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        assert!(
                            (r - t).abs() < t.abs()*precision,
                            "\total_force i:{i} result:{r} target:{t}"
                        );
                    });
                shear_force_result
                    .iter()
                    .zip(shear_force_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        assert!(
                            (r - t).abs() < t.abs()*precision,
                            "\nshear_force i:{i} result:{r} target:{t}"
                        );
                    });
                bending_moment_result
                    .iter()
                    .zip(bending_moment_target.iter())
                    .enumerate()
                    .for_each(|(i, (r, t))| {
                        assert!(
                            (r - t).abs() < t.abs()*precision,
                            "\nbending_moment i:{i} result:{r} target:{t}"
                        );
                    });
        */

        /*        let mass_icing_result = parameters.get(ParameterID::MassIcing).unwrap();
                let mass_icing_target = 67.5;
                assert!(
                    (mass_icing_result - mass_icing_target).abs() < mass_icing_target.abs()*precision,
                    "\nmass_icing result:{mass_icing_result} target:{mass_icing_target}"
                );
        */

        // Угол заливания отверстий
        let flooding_angle = Curve::new_linear(&data.flooding_angle)
            .unwrap()
            .value(mean_draught)
            .unwrap();
        parameters.add(ParameterID::AngleOfDownFlooding, flooding_angle);
        // Угол входа в воду кромки палубы
        let entry_angle = Curve::new_linear(&data.entry_angle)
            .unwrap()
            .value(mean_draught)
            .unwrap();
        parameters.add(ParameterID::OpenDeckEdgeImmersionAngle, entry_angle);
        // метацентрическая высота
        let metacentric_height: Rc<dyn IMetacentricHeight> = Rc::new(MetacentricHeight::new(
            center_draught_shift.clone(), // отстояние центра величины погруженной части судна
            rad_long,                     // продольный метацентрические радиус
            rad_trans,                    // поперечный метацентрические радиус
            loads.tanks().unwrap(),
            Rc::clone(&ship_mass),
            Rc::clone(&ship_moment),
            Rc::clone(&parameters),
        ));
        // Момент кренящий на 1 градус MH1deg, т∙м
        parameters.add(
            ParameterID::MomentRollPerDeg,
            ship_mass.sum().unwrap()
                * metacentric_height.h_trans_fix().unwrap()
                * (std::f64::consts::PI / 180.).sin(),
        );
        // Длинна по ватерлинии при текущей осадке
        let length_wl = Curve::new_linear(&data.waterline_length)
            .unwrap()
            .value(mean_draught)
            .unwrap();
        // Ширина по ватерлинии при текущей осадке
        let breadth_wl = Curve::new_linear(&data.waterline_breadth)
            .unwrap()
            .value(mean_draught)
            .unwrap();
        // Отстояние по вертикали центра площади проекции подводной части корпуса
        let volume_shift = Curve::new_linear(&data.volume_shift)
            .unwrap()
            .value(mean_draught)
            .unwrap();
        // Диаграмма плечей остойчивости
        let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
            Rc::clone(&ship_moment),
            center_draught_shift.clone(),
            data.pantocaren.clone(),
            mean_draught,
            Rc::clone(&metacentric_height),
            Rc::clone(&parameters),
        ));
        // влияние ветра на остойчивость
        let wind: Rc<dyn IWind> = Rc::new(Wind::new(
            data.navigation_area.clone(),
            Rc::new(Windage::new(
                Rc::clone(&icing_stab),
                Rc::clone(&area_stability),
                Curve::new_linear(&data.delta_windage_area)
                    .unwrap()
                    .value(mean_draught)
                    .unwrap(),
                Moment::new(
                    Curve::new_linear(&data.delta_windage_moment_x)
                        .unwrap()
                        .value(mean_draught)
                        .unwrap(),
                    0.,
                    Curve::new_linear(&data.delta_windage_moment_z)
                        .unwrap()
                        .value(mean_draught)
                        .unwrap(),
                ),
                volume_shift,
            )),
            gravity_g,
            Rc::clone(&ship_mass),
            Rc::clone(&parameters),
        ));
        // период качки судна
        let roll_period: Rc<dyn IRollingPeriod> = Rc::new(RollingPeriod::new(
            length_wl,
            data.width,
            mean_draught,
            Rc::clone(&metacentric_height),
        ));
        // амплитуда качки судна
        let coefficient_k: Rc<dyn ICurve> =
            Rc::new(Curve::new_linear(&data.coefficient_k.data()).unwrap());
        let multipler_x1: Rc<dyn ICurve> =
            Rc::new(Curve::new_linear(&data.multipler_x1.data()).unwrap());
        let multipler_x2: Rc<dyn ICurve> =
            Rc::new(Curve::new_linear(&data.multipler_x2.data()).unwrap());
        let multipler_s: Rc<dyn ICurve> =
            Rc::new(Curve::new_linear(&data.multipler_s.get_area(&data.navigation_area.area)).unwrap());
        let coefficient_k_theta: Rc<dyn ICurve> =
            Rc::new(Curve::new_linear(&data.coefficient_k_theta.data()).unwrap());
        let roll_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(
            RollingAmplitude::new(
                data.keel_area,
                Rc::clone(&metacentric_height),
                volume,     // Объемное водоизмещение (1)
                length_wl,  // длинна по ватерлинии при текущей осадке
                data.width, // ширина полная
                breadth_wl, // ширина по ватерлинии при текущей осадке
                mean_draught,
                Rc::clone(&coefficient_k),
                Rc::clone(&multipler_x1),
                Rc::clone(&multipler_x2),
                Rc::clone(&multipler_s),
                Rc::clone(&roll_period),
            )
            .unwrap(),
        );
        // Критерии остойчивости
        /*      let mut criterion_computer_results = CriterionComputer::new(
                 data.overall_height,
                 data.ship_type,
                 Curve::new_linear(&data.h_subdivision)
                     .unwrap()
                     .value(mean_draught)
                     .unwrap(),
                 data.navigation_area,
                 loads.desks().unwrap().iter().any(|v| v.is_timber()),
                 loads.bulks().unwrap().iter().any(|v| v.moment() != 0.),
                 !loads.load_variable().unwrap().is_empty(),
                 icing_stab.is_some(),
                 flooding_angle,
                 data.length_lbp,
                 data.moulded_depth,
                 mean_draught,
                 volume,
                 length_wl,
                 data.width,
                 breadth_wl,
                 data.velocity,
                 Rc::clone(&ship_moment),
                 Rc::clone(&ship_mass),
                 loads.bulks().unwrap(),
                 Rc::clone(&coefficient_k),
                 Rc::clone(&multipler_x1),
                 Rc::clone(&multipler_x2),
                 Rc::clone(&multipler_s),
                 Rc::clone(&coefficient_k_theta),
                 data.keel_area,
                 rad_trans,
                 center_draught_shift.clone(),
                 data.pantocaren.clone(),
                 Rc::clone(&wind),
                 Rc::clone(&metacentric_height),
             )
             .unwrap()
             .calculate()
             .unwrap();
        */
        let criterion_res: HashMap<usize, f64> = CriterionStability::new(
            data.ship_type,
            data.navigation_area.area,
            data.width,
            data.moulded_depth,
            Curve::new_linear(&data.h_subdivision)
                .unwrap()
                .value(mean_draught)
                .unwrap(),
            loads.desks().unwrap().iter().any(|v| v.is_timber()),
            loads.bulks().unwrap().iter().any(|v| v.moment() != 0.),
            !loads.load_variable().unwrap().is_empty(),
            icing_stab.is_some(),
            flooding_angle,
            data.length_lbp,
            Rc::clone(&wind),
            Rc::clone(&lever_diagram),
            Rc::new(Stability::new(
                flooding_angle,
                Rc::clone(&lever_diagram),
                Rc::clone(&roll_amplitude),
                Rc::clone(&wind),
                Rc::clone(&parameters),
            )),
            Rc::clone(&metacentric_height),
            Rc::new(Acceleration::new(
                data.width,
                mean_draught,
                Rc::new(Curve::new_linear(&data.coefficient_k_theta.data()).unwrap()),
                Rc::clone(&roll_period),
                Rc::clone(&roll_amplitude),
                Rc::clone(&metacentric_height),
            )),
            Rc::new(
                Circulation::new(
                    data.velocity,
                    length_wl,
                    mean_draught,
                    Rc::clone(&ship_mass),
                    Rc::clone(&ship_moment),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&parameters),
                )
                .unwrap(),
            ),
            Box::new(Grain::new(
                flooding_angle,
                loads.bulks().unwrap(),
                Rc::clone(&ship_mass),
                Rc::clone(&lever_diagram),
                Rc::clone(&parameters),
            )),
        )
        .unwrap()
        .create()
        .into_iter()
        .map(|v| (v.criterion_id, v.result))
        .collect();

        /*    let mut grain = Grain::new(
                    flooding_angle,
                    loads.bulks().unwrap(),
                    Rc::clone(&ship_mass),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&parameters),
                );

                let grain_angle_result = grain.angle().unwrap().0;
                let grain_angle_target = 12.;
                assert!(
                    grain_angle_result < grain_angle_target,
                    "\ngrain_angle result:{grain_angle_result} target:{grain_angle_target}"
                );
                let grain_area_result = grain.area().unwrap();
                let grain_area_target = 0.075;
                assert!(
                    grain_area_result < grain_area_target,
                    "\ngrain_area grain_angle_result:{grain_angle_result} grain_angle_target:{grain_angle_target} grain_area_result:{grain_area_result} grain_area_target:{grain_area_target}"
                );
        */

        /*
                // Для расчета и записи осадок в параметры
                let _ = DraftMark::new(
                    Rc::new(
                        Draught::new(
                            data.length_lbp,
                            center_waterline_shift,
                            // Дифферент для остойчивости
                            Rc::new(
                                stability::Trim::new(
                                    data.length_lbp,
                                    mean_draught,
                                    center_draught_shift.clone(),
                                    Rc::clone(&metacentric_height),
                                    Rc::clone(&ship_mass),
                                    Rc::clone(&ship_moment),
                                    Rc::clone(&parameters),
                                )
                                .unwrap(),
                            ),
                            Some(Rc::clone(&parameters)),
                        )
                        .unwrap(),
                    ),
                    data.draft_mark,
                    Rc::clone(&parameters),
                )
                .calculate();

                let x_g_result = parameters.get(ParameterID::CenterMassX).unwrap();
                let x_g_target = -3.53;
                assert!(
                    (x_g_result - x_g_target).abs() < x_g_target.abs() * precision,
                    "\nx_g result:{x_g_result} target:{x_g_target}"
                );

                let z_g_result = parameters.get(ParameterID::CenterMassZ).unwrap();
                let z_g_target = 5.07;
                assert!(
                    (z_g_result - z_g_target).abs() < z_g_target.abs() * precision,
                    "\nz_g result:{z_g_result} target:{z_g_target}"
                );
                //OCAДKA HA MИДEЛE, M
                let d_result = parameters.get(ParameterID::DraughtMean).unwrap();
                let d_target = 1.65;
                assert!(
                    (d_result - d_target).abs() < d_target.abs() * precision,
                    "\nd result:{d_result} target:{d_target}"
                );
                //OCAДKA HOCOM, M
                let d_b_result = parameters.get(ParameterID::DraughtBow).unwrap();
                let d_b_target = 1.34;
                assert!(
                    (d_b_result - d_b_target).abs() < d_b_target.abs() * precision,
                    "\nd_b result:{d_b_result} target:{d_b_target}"
                );
                //OCAДKA KOPMOЙ, M
                let d_s_result = parameters.get(ParameterID::DraughtStern).unwrap();
                let d_s_target = 1.96;
                assert!(
                    (d_s_result - d_s_target).abs() < d_s_target.abs() * precision,
                    "\nd_s result:{d_s_result} target:{d_s_target}"
                );
                //ПOПEPEЧHAЯ MЦB C УЧETOM ПOПPABOK, M
                let h_fix_result = parameters.get(ParameterID::MetacentricTransHeight).unwrap();
                let h_fix_target = 4.887;
                assert!(
                    (h_fix_result - h_fix_target).abs() < h_fix_target.abs() * precision,
                    "\nh_fix result:{h_fix_result} target:{h_fix_target}"
                );
                //ЧИCЛO TOHH HA 1 CM. OCAДKИ
                let t_per_sm_result = parameters.get(ParameterID::TonesPerCm).unwrap();
                let t_per_sm_target = 14.92;
                assert!(
                    (t_per_sm_result - t_per_sm_target).abs() < t_per_sm_target.abs() * precision,
                    "\nt_per_sm result:{t_per_sm_result} target:{t_per_sm_target}"
                );
                //MOMEHT, KPEHЯЩИЙ HA 1 ГPAДУC, TM
                let roll_per_deg_result = parameters.get(ParameterID::MomentRollPerDeg).unwrap();
                let roll_per_deg_target = 200.82;
                assert!(
                    (roll_per_deg_result - roll_per_deg_target).abs()
                        < roll_per_deg_target.abs() * precision,
                    "\nroll_per_deg result:{roll_per_deg_result} target:{roll_per_deg_target}"
                );
                //MOMEHT, ДИФФEPEHTУЮЩИЙ HA 1 CM., TM
                let moment_trim_result = parameters.get(ParameterID::MomentTrimPerCm).unwrap();
                let moment_trim_target = 123.37;
                assert!(
                    (moment_trim_result - moment_trim_target).abs() < t_per_sm_target.abs() * precision,
                    "\nmoment_trim result:{moment_trim_result} target:{moment_trim_target}"
                );
                //УГOЛ MAKCИMУMA 1, ГPAД
                let heel_max_result = criterion_res
                    .get(&(CriterionID::HeelMaximumLC as usize))
                    .unwrap();
                let heel_max_target = 24.80_f64;
                assert!(
                    (heel_max_result - heel_max_target).abs() < heel_max_target.abs() * precision,
                    "\nheel_max result:{heel_max_result} target:{heel_max_target}"
                );
                //УГOЛ ЗAKATA, ГPAД.
                let sunset_angle_result = parameters.get(ParameterID::SunsetAngle).unwrap();
                let sunset_angle_target = 67.98;
                assert!(
                    (sunset_angle_result - sunset_angle_target).abs()
                        < sunset_angle_target.abs() * precision,
                    "\nsunset_angle result:{sunset_angle_result} target:{sunset_angle_target}"
                );
                //MAKCИMAЛЬHOE ПЛEЧO, M
                let max_lc_result = criterion_res
                    .get(&(CriterionID::MaximumLC as usize))
                    .unwrap();
                let max_lc_target = 1.626_f64;
                assert!(
                    (max_lc_result - max_lc_target).abs() < max_lc_target.abs() * precision,
                    "\nmax_lc result:{max_lc_result} target:{max_lc_target}"
                );
                //УГOЛ KPEHA, ГPAД.
                let roll_result = parameters.get(ParameterID::Roll).unwrap();
                let roll_target = -0.3166;
                assert!(
                    (roll_result - roll_target).abs() < roll_target.abs() * precision,
                    "\nroll result:{roll_result} target:{roll_target}"
                );
                //ДИHAMИЧECKИЙ УГOЛ KPEHA, ГPAД.
                /*       let dynamic_angle_result = parameters
                          .get(ParameterID::DynamicWindageHeelingAngle)
                          .unwrap();
                      let dynamic_angle_target = 20.30;
                      assert!(
                          (dynamic_angle_result - dynamic_angle_target).abs()
                              < dynamic_angle_target.abs() * precision,
                          "\ndynamic_angle result:{dynamic_angle_result} target:{dynamic_angle_target}"
                      );
                */      //AMПЛИTУДA KAЧKИ, ГPAД.
                let roll_amplitude_result = parameters.get(ParameterID::RollAmplitude).unwrap();
                let roll_amplitude_target = 19.00;
                assert!(
                    (roll_amplitude_result - roll_amplitude_target).abs()
                        < roll_amplitude_target.abs() * precision,
                    "\nroll_amplitude result:{roll_amplitude_result} target:{roll_amplitude_target}"
                );
                //ДABЛEHИE BETPA, KГ/KB.M
                let wind_pressure_result = parameters.get(ParameterID::WindPressure).unwrap();
                let wind_pressure_target = 25.69*9.81;
                assert!(
                    (wind_pressure_result - wind_pressure_target).abs()
                        < wind_pressure_target.abs() * precision,
                    "\nwind_pressure result:{wind_pressure_result} target:{wind_pressure_target}"
                );
                //ПЛOЩAДЬ ПAPУCHOCTИ, KB.M
                let windage_area_result = parameters.get(ParameterID::WindageArea).unwrap();
                let windage_area_target = 1036.07;
                assert!(
                    (windage_area_result - windage_area_target).abs()
                        < windage_area_target.abs() * precision,
                    "\nwindage_area result:{windage_area_result} target:{windage_area_target}"
                );
                //BOЗBЫШEHИE ЦEHTPA ПAPУCHOCTИ HAД BЛ, M
                let windage_area_lever_result = parameters.get(ParameterID::WindageAreaLever).unwrap();
                let windage_area_lever_target = 4.86;
                assert!(
                    (windage_area_lever_result - windage_area_lever_target).abs() < windage_area_lever_target.abs()*precision,
                    "\nwindage_area_lever result:{windage_area_lever_result} target:{windage_area_lever_target}"
                );
                //ПEPИOД БOPTOBOЙ KAЧKИ, C
                let roll_pariod_result = parameters.get(ParameterID::RollPeriod).unwrap();
                let roll_pariod_target = 6.17;
                assert!(
                    (roll_pariod_result - roll_pariod_target).abs() < roll_pariod_target.abs() * precision,
                    "\nroll_pariod result:{roll_pariod_result} target:{roll_pariod_target}"
                );
                //KPИTEPИЙ ПOГOДЫ
                let wheather_result = criterion_res
                    .get(&(CriterionID::Wheather as usize))
                    .unwrap();
                let wheather_target = 3.20_f64;
                assert!(
                    (wheather_result - wheather_target).abs() < wheather_target.abs() * precision,
                    "\nwheather result:{wheather_result} target:{wheather_target}"
                );
                //KPИTEPИЙ УCKOPEHИЯ
                let acceleration_result = criterion_res
                    .get(&(CriterionID::Acceleration as usize))
                    .unwrap();
                let acceleration_target = 0.65_f64;
                assert!(
                    (acceleration_result - acceleration_target).abs()
                        < acceleration_target.abs() * precision,
                    "\nacceleration result:{acceleration_result} target:{acceleration_target}"
                );
                //ПЛOЩAДИ ПOД ДИAГPAMMOЙ,M*PAД: ДO 30 ГPAД
                let area_lc0_30_result = criterion_res
                    .get(&(CriterionID::AreaLC0_30 as usize))
                    .unwrap();
                let area_lc0_30_target = 0.585_f64;
                assert!(
                    (area_lc0_30_result - area_lc0_30_target).abs() < area_lc0_30_target.abs() * precision,
                    "\narea_lc0_30 result:{area_lc0_30_result} target:{area_lc0_30_target}"
                );
                //ПЛOЩAДИ ПOД ДИAГPAMMOЙ,M*PAД: ДO 40 ГPAД
                let area_lc0_40_result = criterion_res
                    .get(&(CriterionID::AreaLC0_40 as usize))
                    .unwrap();
                let area_lc0_40_target = 0.841_f64;
                assert!(
                    (area_lc0_40_result - area_lc0_40_target).abs() < area_lc0_40_target.abs() * precision,
                    "\narea_lc0_40 result:{area_lc0_40_result} target:{area_lc0_40_target}"
                );
                //ПЛOЩAДИ ПOД ДИAГPAMMOЙ,M*PAД: OT 30 ДO 40 ГPAД
                let area_lc30_40_result = criterion_res
                    .get(&(CriterionID::AreaLC30_40 as usize))
                    .unwrap();
                let area_lc30_40_target = 0.256_f64;
                assert!(
                    (area_lc30_40_result - area_lc30_40_target).abs()
                        < area_lc30_40_target.abs() * precision,
                    "\narea_lc30_40 result:{area_lc30_40_result} target:{area_lc30_40_target}"
                );
                //KPEH OT CTATИЧECKOГO ДEЙCTBИЯ BETPA,ГPAД
                let static_wind_angle_result = parameters
                    .get(ParameterID::StaticWindageHeelingAngle)
                    .unwrap();
                let static_wind_angle_target = 0.40;
                assert!(
                    (static_wind_angle_result - static_wind_angle_target).abs() < static_wind_angle_target.abs()*precision,
                    "\nstatic_wind_angle result:{static_wind_angle_result} target:{static_wind_angle_target}"
                );
                //ПЛOЩAДЬ A, M*PAД
                let area_a_result = parameters.get(ParameterID::AreaA).unwrap();
                let area_a_target = 0.2992;
                assert!(
                    (area_a_result - area_a_target).abs() < area_a_target.abs() * precision,
                    "\narea_a result:{area_a_result} target:{area_a_target}"
                );
                //ПЛOЩAДЬ B, M*PAД
                let area_b_result = parameters.get(ParameterID::AreaB).unwrap();
                let area_b_target = 0.9568;
                assert!(
                    (area_b_result - area_b_target).abs() < area_b_target.abs() * precision,
                    "\narea_b result:{area_b_result} target:{area_b_target}"
                );
                //Диаграмма плечей остойчивости
                let diagram_result: HashMap<i32, (f64, f64)> = lever_diagram
                    .diagram()
                    .unwrap()
                    .into_iter()
                    .filter(|(a, _, _)| (a / 5.).fract() <= 0.001)
                    .map(|(a, dso, ddo)| (a.round() as i32, (dso, ddo)))
                    .collect();
                let diagram_target = vec![
                    (0, 0.030, 0.000),
                    (5, 0.458, 0.021),
                    (10, 0.896, 0.080),
                    (15, 1.319, 0.177),
                    (20, 1.563, 0.305),
                    (25, 1.626, 0.445),
                    (30, 1.581, 0.585),
                    (35, 1.473, 0.719),
                    (40, 1.324, 0.841),
                    (45, 1.151, 0.949),
                    (50, 0.956, 1.042),
                    (55, 0.716, 1.115),
                    (60, 0.445, 1.166),
                    (70, -0.116, 1.194),
                    (80, -0.627, 1.129),
                    (90, -1.144, 0.975),
                ];
                diagram_target.iter().for_each(|(a, dso_trg, ddo_trg)| {
                    let (dso_res, ddo_res) = diagram_result.get(a).unwrap();
                    assert!(
                        (dso_res - dso_trg).abs() < dso_trg.abs() * precision,
                        "\ndso a:{a} result:{dso_res} target:{dso_trg}"
                    );
                    assert!(
                        (ddo_res - ddo_trg).abs() < ddo_trg.abs() * precision,
                        "\nddo a:{a} result:{ddo_res} target:{ddo_trg}"
                    );
                });
        */

        /*            log::info!("Main criterion zg:");
                for (id, zg, result, target) in criterion_computer_results.iter() {
                    log::info!("id:{id} zg:{zg} result:{result} delta:{}", result - target);
                }
                log::info!("Main criterion:");
                for v in criterion_res.iter() {
                    log::info!(
                        "id:{} result:{} target:{}",
                        v.criterion_id,
                        v.result,
                        v.target
                    );
                }
                //   send_stability_data(&mut api_server, ship_id, criterion.create()).unwrap(); //
        //        send_parameters_data(&mut api_server, ship_id, parameters.take_data()).unwrap(); //
        */

        test_duration.exit();
    }
}
