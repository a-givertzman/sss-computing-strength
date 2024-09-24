use crate::error::Error;

use super::*;
///
impl ParsedShipData {
    /// Проверка данных на корректность
    pub fn check(self) -> Result<Self, Error> {
        log::info!("result check begin");
        if self.navigation_area.m <= 0. {
            return Err(Error::Parameter(
                "Error check navigation_area: m <= 0".to_string(),
            ));
        }
        if self.navigation_area.p_v <= 0. {
            return Err(Error::Parameter(
                "Error check navigation_area: p_v <= 0".to_string(),
            ));
        }
        if self.icing_m_timber <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_timber:{} <= 0",
                self.icing_m_timber
            )));
        }
        if self.icing_m_v_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_v_full:{} <= 0",
                self.icing_m_v_full
            )));
        }
        if self.icing_m_v_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_v_half:{} <= 0",
                self.icing_m_v_half
            )));
        }
        if self.icing_m_h_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_h_full:{} <= 0",
                self.icing_m_h_full
            )));
        }
        if self.icing_m_h_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_h_half:{} <= 0",
                self.icing_m_h_half
            )));
        }
        if self.wetting_timber < 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: wetting_timber:{} < 0",
                self.wetting_timber
            )));
        }
        if self.multipler_x1.data.is_empty() {
            return Err(Error::Parameter(
                "Error check MultiplerX1Array: no data".to_string(),
            ));
        }
        if self.multipler_x2.data.is_empty() {
            return Err(Error::Parameter(
                "Error check MultiplerX2Array: no data".to_string(),
            ));
        }
        if self.multipler_s.data.is_empty() {
            return Err(Error::Parameter(
                "Error check MultiplerSArray: no data".to_string(),
            ));
        }
        if self.coefficient_k.data.is_empty() {
            return Err(Error::Parameter(
                "Error check CoefficientKArray: no data".to_string(),
            ));
        }
        if self.coefficient_k_theta.data.is_empty() {
            return Err(Error::Parameter(
                "Error check CoefficientKThetaArray: no data".to_string(),
            ));
        }
        if self.length_lbp <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: length_lbp:{} <= 0",
                self.length_lbp
            )));
        }
        if self.length_loa <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: length_loa:{} <= 0.",
                self.length_loa
            )));
        }
        if self.width <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: width:{} <= 0",
                self.width
            )));
        }
        if self.midship <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: midship:{} <= 0",
                self.midship
            )));
        }
        if self.overall_height <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: overall_height:{} <= 0",
                self.overall_height
            )));
        }
        if self.bow_h_min <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: overall_height:{} <= 0",
                self.bow_h_min
            )));
        }
        if self.velocity <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: velocity:{} <= 0",
                self.velocity
            )));
        }
        if self.deadweight <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: deadweight:{} <= 0",
                self.deadweight
            )));
        }
        if self.freeboard_type.len() == 0 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: freeboard_type.len() == 0"
            )));
        }
        if self.bow_area_min <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: bow_area_min:{} <= 0",
                self.bow_area_min
            )));
        }
        if let Some(keel_area) = self.keel_area {
            if keel_area < 0. {
                return Err(Error::Parameter(format!(
                    "Error check ParsedShipData: keel_area:{keel_area} < 0",
                )));
            }
        }
        if self.bounds.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: len() <= 1".to_string(),
            ));
        }
        if self.water_density <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: water_density:{} <= 0",
                self.water_density
            )));
        }
        if self.const_mass_shift_x.abs() >= self.midship {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: const_mass_shift_x{} >= midship{}",
                self.const_mass_shift_x, self.midship,
            )));
        }
        if self.const_mass_shift_z < 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: const_mass_shift_z{} > 0",
                self.const_mass_shift_z,
            )));
        }
        if self.draught_min <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught_min:{} <= 0",
                self.draught_min
            )));
        }
        if self.moulded_depth <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: moulded_depth:{} <= 0",
                self.moulded_depth
            )));
        }
        if self.icing_coef_v_area_full <= 0. || self.icing_coef_v_area_full > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_area_full < 1 {}",
                self.icing_coef_v_area_full
            )));
        }
        if self.icing_coef_v_area_half <= 0. || self.icing_coef_v_area_half > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_area_half < 1 {}",
                self.icing_coef_v_area_half
            )));
        }
        if self.icing_coef_v_area_zero <= 0. || self.icing_coef_v_area_zero > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_area_zero < 1 {}",
                self.icing_coef_v_area_zero
            )));
        }
        if self.icing_coef_v_moment_full <= 0. || self.icing_coef_v_moment_full > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_moment_full < 1 {}",
                self.icing_coef_v_moment_full
            )));
        }
        if self.icing_coef_v_moment_half <= 0. || self.icing_coef_v_moment_half > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_moment_half < 1 {}",
                self.icing_coef_v_moment_half
            )));
        }
        if self.icing_coef_v_moment_zero <= 0. || self.icing_coef_v_moment_zero > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_moment_zero < 1 {}",
                self.icing_coef_v_moment_zero
            )));
        }
        if self.delta_windage_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_area: delta_windage_area.len() <= 1 {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_x.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_x: delta_windage_moment_x.len() <= 1 {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_z.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_z: delta_windage_moment_z.len() <= 1 {}",
                self.flooding_angle.len()
            )));
        }
        if self.center_waterline.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: len() <= 1".to_string(),
            ));
        }
        if self.waterline_length.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: waterline_length.len() <= 1".to_string(),
            ));
        }
        if self.waterline_breadth.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: waterline_breadth.len() <= 1".to_string(),
            ));
        }
        if self.waterline_area.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: waterline_area.len() <= 1".to_string(),
            ));
        }
        if self.volume_shift.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: volume_shift.len() <= 1".to_string(),
            ));
        }
        if self.rad_long.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: rad_long.len() <= 1".to_string(),
            ));
        }
        if self.rad_trans.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: rad_trans.len() <= 1".to_string(),
            ));
        }
        if self.h_subdivision.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: h_subdivision.len() <= 1".to_string(),
            ));
        }
        if self.mean_draught.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: mean_draught.len() <= 1".to_string(),
            ));
        }
        if self.center_draught_shift_x.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: center_draught_shift_x.len() <= 1".to_string(),
            ));
        }
        if self.center_draught_shift_y.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: center_draught_shift_y.len() <= 1".to_string(),
            ));
        }
        if self.center_draught_shift_z.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: center_draught_shift_z.len() <= 1".to_string(),
            ));
        }
        if self.pantocaren.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: pantocaren.len() <= 1".to_string(),
            ));
        }
        if let Some((draught, _)) = self.pantocaren.iter().find(|(draught, _)| *draught < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: pantocaren draught draught < 0 {}",
                draught
            )));
        }
        if self.flooding_angle.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: flooding_angle.len() <= 1".to_string(),
            ));
        }
        if let Some((key, value)) = self
            .flooding_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: flooding_angle key{key} < 0 || value{value} < 0"
            )));
        }
        if self.entry_angle.len() <= 1 {
            return Err(Error::Parameter(
                "Error check ParsedShipData: entry_angle.len() <= 1".to_string(),
            ));
        }
        if let Some((key, value)) = self
            .entry_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: entry_angle key{key} < 0. || value:{value} < 0"
            )));
        }
        if self.frame_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: frame_area.len() <= 1 {}",
                self.frame_area.len()
            )));
        }
        if let Some(frame) = self
            .frame_area
            .iter()
            .find(|f| f.immersion_area.iter().any(|v| v.0 < 0. || v.1 < 0.))
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: immersion_area frame{frame}: v.0 < 0 || v.1 < 0"
            )));
        }
        if self.draft_mark.is_empty() {
            return Err(Error::Parameter(
                "Error check draft_mark: draft_mark.is_empty()".to_string(),
            ));
        }
        if self.load_line.is_empty() {
            return Err(Error::Parameter(
                "Error check load_line: load_line.is_empty()".to_string(),
            ));
        }
        if self.screw.is_empty() {
            return Err(Error::Parameter(
                "Error check screw: screw.data().is_empty()".to_string(),
            ));
        }
        if let Some(v) = self.screw.iter().find(|v| v.d <= 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: screw{v} diametr error!"
            )));
        }
        if self.bow_board.is_empty() {
            return Err(Error::Parameter(
                "Error check bow_board: bow_board.is_empty()".to_string(),
            ));
        }
        if self.compartments.is_empty() {
            return Err(Error::Parameter(
                "Error check compartments: compartments.is_empty()".to_string(),
            ));
        }
        if let Some(s) = self.compartments.iter().find(|s| s.mass.unwrap_or(0.) < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment{s} mass < 0"
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| s.bound_x1 >= s.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment{s} bound error!"
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| {
            s.mass.unwrap_or(0.) > 0.
                && s.mass_shift_x.is_some()
                && (s.bound_x1 >= s.mass_shift_x.unwrap() || s.mass_shift_x.unwrap() >= s.bound_x2)
        }) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment{s} mass_shift_x error!"
            )));
        }
        if self.load_constants.is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: load_constants.is_empty()".to_string(),
            ));
        }
        if self.area_h_stab.is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: area_h_stab.is_empty()".to_string(),
            ));
        }
        if let Some(area) = self.area_h_stab.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: area_h_stab value < 0 {area}"
            )));
        }
        if self.area_h_str.is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: area_h_str.is_empty()".to_string(),
            ));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: area_h_str value < 0 {area}"
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.bound_x1 >= f.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: area_h_str bound_x1 >= f.bound_x2 {area}"
            )));
        }
        if self.area_v_str.is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: area_v_str.is_empty()".to_string(),
            ));
        }
        if let Some(area) = self.area_v_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: area_v_str value < 0 {area}"
            )));
        }
        if self.area_v_stab.area().is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: area_v_stab.area().is_empty()".to_string(),
            ));
        }
        if let Some(v) = self
            .area_v_stab
            .area()
            .iter()
            .find(|(d, a)| *a < 0. || *d < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught:{},  area:{}",
                v.0, v.1
            )));
        }
        if self.bow_area.is_empty() {
            return Err(Error::Parameter(
                "Error check ParsedShipData: bow_area.is_empty()".to_string(),
            ));
        }

        log::info!("result check ok");
        Ok(self)
    }
}
