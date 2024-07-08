use std::{cell::RefCell, rc::Rc};

use super::{calc_context::CalcContext, calc_eval::CalcEval};
///
/// 
pub struct SetContext<I, O> {
    set: Box<dyn Fn(Rc<RefCell<CalcContext>>, O)>,
    exp: Box<dyn CalcEval<I, O>>,
}
//
//
impl<I, O> SetContext<I, O> {
    pub fn new(
        set: impl Fn(Rc<RefCell<CalcContext>>, O) + 'static,
        exp: impl CalcEval<I, O> + 'static,
    ) -> Self {
        Self {
            set: Box::new(set),
            exp: Box::new(exp),
        }
    }
}
//
//
impl<I, O: Clone> CalcEval<I, O> for SetContext<I, O> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O {
        let result = self.exp.eval(context.clone());
        (self.set)(context, result.clone());
        result
    }
}
