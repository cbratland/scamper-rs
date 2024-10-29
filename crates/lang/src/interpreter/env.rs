use super::RuntimeError;
use crate::ast::{IntoValue, NativeFn, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    bindings: HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Env>) -> Self {
        let mut env = Self {
            bindings: HashMap::new(),
            parent: parent.map(Box::new),
        };

        crate::modules::prelude::add_to(&mut env);

        env
    }

    pub fn get<K>(&self, key: K) -> Option<&Value>
    where
        K: AsRef<str>,
    {
        let value = self.bindings.get(key.as_ref());
        if value.is_none() {
            if let Some(parent) = &self.parent {
                return parent.get(key);
            }
        }
        value
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.bindings.insert(key.clone(), value);
    }

    pub fn register(&mut self, name: &str, func: fn(&[Value]) -> Result<Value, RuntimeError>) {
        self.bindings
            .insert(name.to_string(), Value::Function(NativeFn::new(func)));
    }

    pub fn register_value<T: IntoValue>(&mut self, name: &str, value: T) -> bool {
        let Some(value) = value.into_value() else {
            return false;
        };
        self.bindings.insert(name.to_string(), value);
        return true;
    }
}
