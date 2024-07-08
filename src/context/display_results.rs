use std::{cell::RefCell, rc::Rc};
use super::{calc_context::CalcContext, calc_eval::CalcEval};
///
/// 
pub struct DisplayState<I, O> {
    id: String,
    label: String,
    disp: Box<dyn Fn(Rc<RefCell<CalcContext>>) -> Box<dyn std::fmt::Debug>>,
    exp: Box<dyn CalcEval<I, O>>,
}
//
//
impl<I, O> DisplayState<I, O> {
    pub fn new(
        label: impl Into<String>,
        disp: impl (Fn(Rc<RefCell<CalcContext>>) -> Box<dyn std::fmt::Debug>) + 'static,
        exp: impl CalcEval<I, O> + 'static,
    ) -> Self {
        Self {
            id: format!("DisplayCalcResults"),
            label: label.into(),
            disp: Box::new(disp),
            exp: Box::new(exp),
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O {
        let result = self.exp.eval(context.clone());
        // let context = context.borrow();
        println!("\n{}", self.label);
        let f = self.disp.as_ref();
        let disp = f(context);
        println!("{}.eval | results: {:#?}", self.id, disp);
        // println!("{}.eval results/field1: {}", self.id, context.results.mul2.field1.get());
        // println!("{}.eval results/field2: {:#?}", self.id, context.results.mul2.field2.get());
        // println!("{}.eval results/field3: {}", self.id, context.results.mul2.field3.get());
        result
    }
}
//
//
impl<I, O> CalcEval<I, O> for DisplayState<I, O> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O {
        DisplayState::eval(self, context)
    }
}
