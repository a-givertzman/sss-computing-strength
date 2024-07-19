

use crate::error::Error;

use super::*;
///
impl ParsedShipData {
    /// Проверка данных на корректность
    pub fn check(self) -> Result<Self, Error> {
        if self.navigation_area_param.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check NavigationAreaArray: no data"
            )));
        }
        if self.multipler_x1.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerX1Array: no data"
            )));
        }
        if self.multipler_x2.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerX2Array: no data"
            )));
        }
        if self.multipler_s.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerSArray: no data"
            )));
        }
        if self.coefficient_k.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check CoefficientKArray: no data"
            )));
        }
        if self.coefficient_k_theta.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check CoefficientKThetaArray: no data"
            )));
        }
        if self.length_lbp <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: length must be positive {}",
                self.length_lbp
            )));
        }
        if self.width <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: width must be positive {}",
                self.width
            )));
        }
        if self.midship <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: midship must be positive {}",
                self.midship
            )));
        }
        if self.velocity <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: velocity must be positive {}",
                self.velocity
            )));
        }
        if let Some(keel_area) = self.keel_area {
            if keel_area < 0. {
                return Err(Error::Parameter(format!(
                    "Error check ParsedShipData: keel_area must be non-negative {keel_area}",
                )));
            }
        }
        if self.water_density <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of water density must be positive {}",
                self.water_density
            )));
        }
        /*   if self.mass <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of mass must be positive {}",
                self.mass
            )));
        }
        if self.volume <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of volume must be positive {}",
                self.volume
            )));
        }*/
        if self.draught_min <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of draught_min must be positive {}",
                self.draught_min
            )));
        }
        if self.moulded_depth <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of moulded_depth must be positive {}",
                self.moulded_depth
            )));
        }
        if self.icing_m_timber <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_timber must be positive {}",
                self.icing_m_timber
            )));
        }
        if self.icing_m_v_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_v_full must be positive {}",
                self.icing_m_v_full
            )));
        }
        if self.icing_m_v_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_v_half must be positive {}",
                self.icing_m_v_half
            )));
        }
        if self.icing_m_h_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_h_full must be positive {}",
                self.icing_m_h_full
            )));
        }
        if self.icing_m_h_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_h_half must be positive {}",
                self.icing_m_h_half
            )));
        }
        if self.icing_m_v_full <= self.icing_m_v_half {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_v_full {} <= icing_m_v_half {}",
                self.icing_m_v_full, self.icing_m_v_half
            )));
        }
        if self.icing_m_h_full <= self.icing_m_h_half {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_h_full {} <= icing_m_h_half {}",
                self.icing_m_h_full, self.icing_m_h_half
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
        if self.bounds.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of bounds's points {}",
                self.bounds.len()
            )));
        }
        if self.center_waterline.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of waterline's points {}",
                self.center_waterline.len()
            )));
        }
        if self.rad_long.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_long's points {}",
                self.rad_long.len()
            )));
        }
        if self.rad_trans.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_trans's points {}",
                self.rad_trans.len()
            )));
        }
        if self.h_subdivision.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of h_subdivision's points {}",
                self.h_subdivision.len()
            )));
        }
        if self.mean_draught.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of mean_draught's points {}",
                self.mean_draught.len()
            )));
        }
        if self.center_draught_shift_x.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_x's points {}",
                self.center_draught_shift_x.len()
            )));
        }
        if self.center_draught_shift_y.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_y's points {}",
                self.center_draught_shift_y.len()
            )));
        }
        if self.center_draught_shift_z.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_z's points {}",
                self.center_draught_shift_z.len()
            )));
        }
        if self.pantocaren.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of pantocaren's points {}",
                self.pantocaren.len()
            )));
        }
        if let Some((draught, _)) = self.pantocaren.iter().find(|(draught, _)| *draught < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught in pantocaren is negative!, {}",
                draught
            )));
        }
        if self.flooding_angle.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if let Some((key, value)) = self
            .flooding_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in flooding_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.entry_angle.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_area: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_x.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_x: number of points{}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_z.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_z: number of points{}",
                self.flooding_angle.len()
            )));
        }
        if let Some((key, value)) = self
            .entry_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in entry_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.frame_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be {}",
                self.frame_area.len()
            )));
        }
        if self.draft_mark.data().is_empty() {
            return Err(Error::Parameter(format!(
                "Error check draft_mark: draft_mark.data().is_empty()"
            )));
        }

        /*      if let Some(frame) = self.theoretical_frame.iter().find(|f| f.index >= self.frame_area.len() as i32) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: index of frame bigger or equal then frames.len(), {}",
                frame
            )));
        }
        if let Some(frame) = self.theoretical_frame.iter().find(|f| f.index < 0) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: index of frame is negative, {}",
                frame
            )));
        }
        let qnt_unique_index = self
            .theoretical_frame
            .iter()
            .map(|f| f.index)
            .collect::<HashSet<_>>()
            .len();
        if self.theoretical_frame.len() != qnt_unique_index {
            return Err(Error::Parameter(format!("Error check ParsedShipData: index of frame must be unique frames:{}, unique index:{}", self.theoretical_frame.len(), qnt_unique_index )));
        }
        if self
            .theoretical_frame
            .iter()
            .find(|f| f.index == 0)
            .ok_or(format!(
                "ParsedShipData parse error: no frame with index = 0"
            ))?
            .x
            != 0.
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame with index = 0 must be equal to 0"
            )));
        }*/
        /* длинна судна не обязательно совпадает с расстоянием между крайними шпангоутами
        if (self
            .frames
            .iter()
            .find(|f| f.index == self.frames.len() - 1)
            .ok_or(format!(
                "ParsedShipData parse error: no frame with last index = len-1"
            ))?
            .x - self.ship_length).abs() > 0.01
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame with last index must be equal to ship_length"
            )));
        }*/
        /*     if let Some(frame) = self.theoretical_frame.iter().find(|f| f.x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame must be greater or equal to 0, {}",
                frame
            )));
        }*/
        if let Some(frame) = self.frame_area.iter().find(|f| {
            f.immersion_area
                .iter()
                .find(|v| v.0 < 0. || v.1 < 0.)
                .is_some()
        }) {
            return Err(Error::Parameter(format!("Error check ParsedShipData: values of immersion_area in frame must be greater or equal to 0, {}", frame)));
        }
        /*      let cargo_data = self.cargo.data();
        if let Some((index, value)) = cargo_data.iter().find(|(_, value)| **value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: mass of load_constant must be greater or equal to 0, index:{}, value:{}",
                index, value,
            )));
        } */
        /*     if let Some((index, _)) = load_constant_data.iter().find(|(index, _)| self.theoretical_frame.iter().find(|frame| frame.index == **index as i32 ).is_none()) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: index of load_constant must be contained in frames, index:{}", index)));
        }*/
        /*    if self.compartments.len() < 1 {
            return Err(Error::Parameter(format!(
                "Error check compartments: number of compartments: {}",
                self.compartments.len()
            )));
        }*/
        if let Some(s) = self.compartments.iter().find(|s| s.mass.unwrap() <= 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: mass of compartment must be greater or equal to 0, {}",
                s
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| s.bound_x1 >= s.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment bound error! {}",
                s
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| {
            s.mass.unwrap() > 0.
                && s.mass_shift_x.is_some()
                && (s.bound_x1 >= s.mass_shift_x.unwrap() || s.mass_shift_x.unwrap() >= s.bound_x2)
        }) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment center out of bound error! {}",
                s
            )));
        }
        if self.load_constants.len() < 1 {
            return Err(Error::Parameter(format!(
                "Error check load_constants: number of load_constants: {}",
                self.load_constants.len()
            )));
        }

        /*       if let Some(tank) = self.tanks.iter().find(|t| t.density <= 0.) {
                  return Err(Error::Parameter(format!(
                      "Error check ParsedShipData: density of liquid must be greater or equal to 0 {}",
                      tank
                  )));
              }
              if let Some(tank) = self.tanks.iter().find(|t| t.volume <= 0.) {
                  return Err(Error::Parameter(format!(
                      "Error check ParsedShipData: volume of liquid must be greater or equal to 0 {}",
                      tank
                  )));
              }
              if let Some(tank) = self
                  .tanks
                  .iter()
                  .find(|t| t.center_x.len() <= 1 || t.center_y.len() <= 1 || t.center_z.len() <= 1)
              {
                  return Err(Error::Parameter(format!("Error check ParsedShipData: number of center's points must be {}", tank)));
              }
              if let Some(tank) = self
                  .tanks
                  .iter()
                  .find(|t| t.free_surf_inertia_x.len() <= 1 || t.free_surf_inertia_y.len() <= 1)
              {
                  return Err(Error::Parameter(format!("Error check ParsedShipData: number of free_surf_inertia's points must be {}", tank)));
              }
        */
        if self.waterline_length.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_length")));
        }
        if self.waterline_breadth.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_breadth")));
        }
        if self.waterline_area.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_area")));
        }
        if self.area_h_stab.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_h_stab: number of area_h_stab's points {}",
                self.area_h_stab.len()
            )));
        }
        if let Some(area) = self.area_h_stab.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_h_stab: value of area_h_stab must be greater or equal to 0, {}",
                area
            )));
        }
        if self.area_h_str.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: number of points {}",
                self.area_h_str.len()
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: value of area_h_str must be greater or equal to 0, {}",
                area
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.bound_x1 >= f.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: f.bound_x1 >= f.bound_x2 {}",
                area
            )));
        }      
        if self.area_v_str.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_v_str: number of points {}",
                self.area_v_str.len()
            )));
        }
        if let Some(area) = self.area_v_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_v_str: value of area_v must be greater or equal to 0, {}",
                area
            )));
        }
        if self.area_v_stab.area().len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_v_stab.area(): number of points {}",
                self.area_v_stab.area().len() 
            )));
        }
        if let Some(v) = self.area_v_stab.area().iter().find(|(d, a)| *a < 0. || *d < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check area_v_stab: draught:{},  area:{}",
                v.0, v.1
            )));
        }  
        Ok(self)
    }
}
