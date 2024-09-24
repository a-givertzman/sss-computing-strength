use std::collections::HashMap;

use crate::data::structs::{NavigationArea, NavigationAreaData, Ship};

#[allow(dead_code)]
pub(crate) fn ship() -> Ship {
    Ship{
        name: "Belogorodskaya ARK-20231".to_owned(),
        ship_type: "General dry cargo ship".to_owned(),
        icing_type: "none".to_owned(),
        icing_timber_type: "full".to_owned(),
        navigation_area: NavigationAreaData{area: NavigationArea::R2, p_v: 252., m: 0.52},
        freeboard_type: "B".to_owned(),
        error: HashMap::new(),
    }
}