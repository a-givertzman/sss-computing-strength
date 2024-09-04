//! Структура с входными данными для тестирования.  
//! Инициализируется данными для 'Belogorodskaya ARK-20231'
mod data;
mod navigation_area;
mod multipler_x1;
mod multipler_x2;
mod multipler_s;
mod coefficient_k;
mod coefficient_k_theta;
mod icing;
mod ship_parameters;
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
mod bonjean_frame;
mod frame_area;
mod draft_mark;
mod load_line;
mod screw;
mod cargo;
mod compartment;
mod load_constants;
mod area_h_stab;
mod area_h_str;
mod area_v_stab;
mod area_v_str;

use crate::data::structs::ParsedShipData;

#[allow(dead_code)]
pub(crate) fn input_data() -> ParsedShipData {
    ParsedShipData::parse(
        navigation_area::navigation_area(),
        multipler_x1::multipler_x1(),
        multipler_x2::multipler_x2(),    
        multipler_s::multipler_s(),
        coefficient_k::coefficient_k(),
        coefficient_k_theta::coefficient_k_theta(),
        icing::icing(),
        1,
        ship_parameters::ship_parameters(),
        bounds::bounds(119.95, 59.194, 20),
        center_waterline::center_waterline(),
        waterline_length::waterline_length(),
        waterline_breadth::waterline_breadth(),
        waterline_area::waterline_area(),
        volume_shift::volume_shift(),
        rad_long::rad_long(),
        rad_trans::rad_trans(),
        h_subdivision::h_subdivision(),
        mean_draught::mean_draught(),
        center_draught_shift::center_draught_shift(),
        pantocaren::pantocaren(),
        flooding_angle::flooding_angle(),
        entry_angle::entry_angle(),
        delta_windage_area::delta_windage_area(),
        delta_windage_moment::delta_windage_moment(),
        bonjean_frame::bonjean_frame(),
        frame_area::frame_area(),
        draft_mark::draft_mark(),
        load_line::load_line(),
        screw::screw(),
        cargo::cargo(),
        bulkhead_src::bulkhead_src(),
        compartment::compartment(),
        hold_compartments_src::hold_compartments_src(),
        load_constant_src::load_constant_src(),
        area_h_stab::area_h_stab(),
        area_h_str::area_h_str(),
        area_v_stab::area_v_stab(),
        area_v_str::area_v_str(),
    ).unwrap()
}
