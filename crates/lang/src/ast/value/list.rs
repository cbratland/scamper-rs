use super::{FromValue, IntoValue, Value};

macro_rules! implement_value_collection {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name(Vec<Value>);

        impl $name {
            pub fn empty() -> Self {
                Self(Vec::new())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn values(&self) -> &[Value] {
                &self.0
            }

            pub fn values_vec(self) -> Vec<Value> {
                self.0
            }
        }

        impl From<Vec<Value>> for $name {
            fn from(vec: Vec<Value>) -> Self {
                Self(vec)
            }
        }

        impl Into<Vec<Value>> for $name {
            fn into(self) -> Vec<Value> {
                self.0
            }
        }
    };
}

implement_value_collection!(List);
implement_value_collection!(Vector);

impl FromValue for List {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::List(list) => Some(List(list.clone())),
            Value::Null => Some(List(Vec::new())),
            _ => None,
        }
    }

    fn name() -> &'static str {
        "list"
    }
}

impl IntoValue for List {
    fn into_value(self) -> Option<Value> {
        if self.0.is_empty() {
            Some(Value::Null)
        } else {
            Some(Value::List(self.0))
        }
    }
}

impl FromValue for Vector {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Vector(list) => Some(Vector(list.clone())),
            _ => None,
        }
    }

    fn name() -> &'static str {
        "vector"
    }
}

impl IntoValue for Vector {
    fn into_value(self) -> Option<Value> {
        Some(Value::Vector(self.0))
    }
}
