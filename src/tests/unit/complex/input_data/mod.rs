use std::collections::HashMap;

mod data;
mod multipler_x1;
mod multipler_x2;
mod multipler_s;
mod coefficient_k;
mod coefficient_k_theta;
mod bounds;
mod delta_windage_area;
mod delta_windage_moment;
mod center_waterline;
mod waterline_length;
mod waterline_breadth;
mod waterline_area;
mod volume_shift;
mod rad_long;
mod rad_trans;
mod h_subdivision;
mod mean_draught;
mod center_draught_shift;
mod pantocaren;
mod flooding_angle;
mod entry_angle;
mod frame_area;
mod draft_mark;
mod load_line;
mod screw;
mod cargoes;
mod compartments;
mod load_constants;
mod area_h_stab;
mod area_h_str;
mod area_v_stab;
mod area_v_str;

use crate::{data::structs::{
    navigation_area_data::NavigationAreaData, NavigationArea, NavigationAreaArray, ParsedShipData,
    ShipType,
}, icing_stab::IcingStabType, icing_timber::IcingTimberType};

#[allow(dead_code)]
pub(crate) fn input_data() -> ParsedShipData {
    ParsedShipData {
        ship_type: ShipType::GeneralDryCargoShip,
        navigation_area: NavigationArea::R2,
        navigation_area_param: NavigationAreaArray {
            data: vec![NavigationAreaData {
                area: NavigationArea::R2,
                p_v: 252.,
                m: 0.52,
            }],
            error: HashMap::new(),
        },
        icing_stab: IcingStabType::None,
        icing_timber_stab: IcingTimberType::Full,
        icing_m_timber: 0.032,
        icing_m_v_full: 0.015,
        icing_m_v_half: 0.0075,
        icing_m_h_full: 0.03,
        icing_m_h_half: 0.015,
        icing_coef_v_area_full: 0.1,
        icing_coef_v_area_half: 0.075,
        icing_coef_v_area_zero: 0.05,
        icing_coef_v_moment_full: 0.2,
        icing_coef_v_moment_half: 0.15,
        icing_coef_v_moment_zero: 0.1,
        wetting_timber: 10.,
        multipler_x1: multipler_x1::multipler_x1(),
        multipler_x2: multipler_x2::multipler_x2(),
        multipler_s: multipler_s::multipler_s(),
        coefficient_k: coefficient_k::coefficient_k(),
        coefficient_k_theta: coefficient_k_theta::coefficient_k_theta(),
        length_lbp: 118.388,
        length_loa: 119.95,
        width: 13.4,
        midship: 59.194,
        overall_height: 16.8,
        velocity: 16.,
        keel_area: Some(24.69),
        bounds: bounds::bounds(119.95, 59.194, 20),
        water_density: 1.025,
        const_mass_shift_x: 1.05,
        const_mass_shift_y: 0.,
        const_mass_shift_z: 5.32,
        draught_min: 1.40,
        moulded_depth: 6.8,
        delta_windage_area: delta_windage_area::delta_windage_area(),
        delta_windage_moment_x: delta_windage_moment::delta_windage_moment_x(),
        delta_windage_moment_z: delta_windage_moment::delta_windage_moment_z(),
        center_waterline: center_waterline::center_waterline(),
        waterline_length: todo!(),
        waterline_breadth: todo!(),
        waterline_area: todo!(),
        volume_shift: todo!(),
        rad_long: todo!(),
        rad_trans: todo!(),
        h_subdivision: todo!(),
        mean_draught: todo!(),
        center_draught_shift_x: todo!(),
        center_draught_shift_y: todo!(),
        center_draught_shift_z: todo!(),
        pantocaren: todo!(),
        flooding_angle: todo!(),
        entry_angle: todo!(),
        frame_area: todo!(),
        draft_mark: todo!(),
        load_line: todo!(),
        screw: todo!(),
        cargoes: todo!(),
        compartments: todo!(),
        load_constants: todo!(),
        area_h_stab: todo!(),
        area_h_str: todo!(),
        area_v_stab: todo!(),
        area_v_str: todo!(),
    }
}
