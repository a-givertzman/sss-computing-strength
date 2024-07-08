use std::rc::Rc;

use crate::{data::structs::ParsedShipData, Bounds, CriterionData, IBulk, IDesk, IParameters, IResults, ITank, LoadMass};
///
/// 
pub struct CalcContext {
    pub src: ParsedShipData,
    pub prep: Prepared, 
    pub results: Results,
}
//
//
impl CalcContext {
    pub fn new(src: ParsedShipData, results: Results) -> Self {
        Self { src, results }
    }
}
///
#[derive(Clone)]
pub struct Prepared {
    pub gravity_g: f64,
    pub bounds: Rc<Bounds>,
    pub tanks: Rc<Vec<Rc<dyn ITank>>>,
    pub desks: Rc<Vec<Rc<dyn IDesk>>>,
    pub bulk: Rc<Vec<Rc<dyn IBulk>>>,
    pub load_variable: Rc<Vec<Rc<LoadMass>>>,
}
/// 
#[derive(Clone)]
pub struct Results {
    pub criterion: Vec<CriterionData>,
    pub results: Rc<dyn IResults>,
    pub parameters: Rc<dyn IParameters>
}
//
//
impl Results {
    pub fn new(
        criterion: Vec<CriterionData>,
        results: Rc<dyn IResults>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            criterion,
            results,
            parameters,
        }
    }
}
