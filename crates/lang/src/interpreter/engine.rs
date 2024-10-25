use std::cell::RefCell;
use std::rc::Rc;

use super::{eval::Runner, Env, RuntimeError};
use crate::ast::Value;
use crate::diagnostics::ParseError;
use crate::parser::parse;

pub enum EngineError {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

impl EngineError {
    pub fn emit(&self, file_name: &str, file_content: &str) {
        match self {
            Self::ParseError(err) => err.emit(file_name, file_content),
            Self::RuntimeError(err) => err.emit(file_content),
        }
    }
}

impl From<ParseError> for EngineError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<RuntimeError> for EngineError {
    fn from(err: RuntimeError) -> Self {
        Self::RuntimeError(err)
    }
}

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

    pub fn run(&self, code: &str) -> Result<Vec<Value>, EngineError> {
        let ast = parse(&code)?;
        let mut interpreter = Runner::new(ast, Some(self.env.clone()));
        interpreter.execute()?;
        Ok(interpreter.get_output())
    }
}
