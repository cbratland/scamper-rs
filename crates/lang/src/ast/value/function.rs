use std::{cell::RefCell, rc::Rc};

use super::{Closure, FromValue, NativeFn, Value};
use crate::interpreter::{Env, ExecutionStack, RuntimeError};

#[derive(Debug, Clone)]
pub struct Function {
    _function: Option<NativeFn>,
    _closure: Option<Closure>,
}

impl Function {
    pub fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        match &self._function {
            Some(func) => return func.0(&args),
            None => {}
        }
        match &self._closure {
            Some(closure) => {
                let mut stack = ExecutionStack::new(
                    closure
                        .env
                        .clone()
                        .unwrap_or(Rc::new(RefCell::new(Env::new(None)))),
                    closure.body.clone(),
                );
                stack.run()?;
                return stack
                    .pop()
                    .ok_or(RuntimeError::new("missing stack value".to_string(), None));
            }
            None => {}
        }
        unreachable!()
    }

    pub fn value(&self) -> Value {
        match &self._function {
            Some(func) => return Value::Function(func.clone()),
            None => {}
        }
        match &self._closure {
            Some(closure) => return Value::Closure(closure.clone()),
            None => {}
        }
        unreachable!()
    }
}

impl FromValue for Function {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Function(func) => Some(Function {
                _function: Some(func.clone()),
                _closure: None,
            }),
            Value::Closure(closure) => Some(Function {
                _function: None,
                _closure: Some(closure.clone()),
            }),
            _ => None,
        }
    }
}
