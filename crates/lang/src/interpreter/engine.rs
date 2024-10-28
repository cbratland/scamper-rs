use std::cell::RefCell;
use std::rc::Rc;

use super::eval::{Output, Runner};
use super::Env;
use crate::diagnostics::ParseError;
use crate::parser::parse;

pub struct Engine {
    env: Rc<RefCell<Env>>,
}

impl Engine {
    /// Instantiates a new engine instance with prelude imported.
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Env::new(None))),
        }
    }

    pub fn run(&self, code: &str) -> Result<Vec<Output>, ParseError> {
        let ast = parse(&code)?;
        let mut interpreter = Runner::new(ast, Some(self.env.clone()));
        interpreter.execute();
        Ok(interpreter.get_output())
    }

    pub fn check_syntax(&self, code: &str) -> Result<(), ParseError> {
        parse(&code)?;
        Ok(())
    }
}
