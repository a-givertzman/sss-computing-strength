use std::{cell::RefCell, rc::Rc};

use super::{calc_context::CalcContext, calc_eval::CalcEval};

///
/// 
pub struct DisplaySrc<T> {
    id: String,
    label: String,
    exp: Box<dyn CalcEval<T, T>>,
}
//
//
impl<T> DisplaySrc<T> {
    pub fn new(
        label: impl Into<String>,
        exp: impl CalcEval<T, T> + 'static,
    ) -> Self {
        Self {
            id: format!("DisplaySrc"),
            label: label.into(),
            exp: Box::new(exp),
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        let context_ref = self.exp.eval(context.clone());
        // let context = context_ref.clone();
        let context = context.borrow_mut();
        println!("\n{}", self.label);
 //      println!("{}.eval src/field1: {}", self.id, context.src.field1.get());
  //      println!("{}.eval src/field2: {:#?}", self.id, context.src.field2.get());
   //     println!("{}.eval src/field3: {}", self.id, context.src.field3.get());
        context_ref
    }
}
//
//
impl<T> CalcEval<T, T> for DisplaySrc<T> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        DisplaySrc::eval(self, context)
    }
}