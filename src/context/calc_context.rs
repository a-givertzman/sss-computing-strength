use std::rc::Rc;

use crate::{data::structs::ParsedShipData, Bounds, CriterionData, IBulk, IDesk, IParameters, IResults, ITank, LoadMass};
///
/// 
pub struct CalcContext {
    pub data: ParsedShipData,
    pub gravity_g: Option<f64>,
    pub bounds: Option<Rc<Bounds>>,
    pub tanks: Option<Rc<Vec<Rc<dyn ITank>>>>,
    pub desks: Option<Rc<Vec<Rc<dyn IDesk>>>>,
    pub bulk: Option<Rc<Vec<Rc<dyn IBulk>>>>,
    pub load_variable: Option<Rc<Vec<Rc<LoadMass>>>>,
    pub strength_results: Option<Rc<dyn IResults>>,  
    pub parameters: Option<Rc<dyn IParameters>>,      
    pub criterion: Option<Vec<CriterionData>>,
}
//
//
impl CalcContext {
    pub fn new(data: ParsedShipData) -> Self {
        Self { 
            data, 
            gravity_g: None,
            bounds: None,
            tanks: None,
            desks: None,
            bulk: None,
            load_variable: None,
            strength_results: None,
            parameters: None,            
            criterion: None,
        }
    }
}
