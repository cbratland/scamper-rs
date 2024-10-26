use super::Block;
use crate::interpreter::RuntimeError;
use std::{any::Any, fmt::Debug, rc::Rc};

type NativeFnSignature = fn(&[Value]) -> Result<Value, RuntimeError>;

#[derive(Clone, PartialEq)]
pub struct NativeFn(pub NativeFnSignature);

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeFn>")
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Pair(Box<Value>, Box<Value>),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Symbol(String),
    Closure { params: Vec<String>, body: Block },
    // rust function
    Function(NativeFn),
    Foreign(Rc<dyn Any>),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => {
                if fl.fract() == 0.0 {
                    write!(f, "{}", *fl as i64)
                } else {
                    write!(f, "{}", fl)
                }
            }
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Char(c) => write!(f, "\\#{}", c),
            Value::Pair(a, b) => write!(f, "(pair {} {})", a, b),
            Value::List(l) => {
                write!(f, "(list ")?;
                for (i, item) in l.iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Vector(v) => {
                write!(f, "(vector ")?;
                for (i, item) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Closure { params, .. } => {
                write!(f, "(lambda ({}) <body>)", params.join(", "))
            }
            Value::Function(_) => write!(f, "<function>"),
            Value::Foreign(_) => write!(f, "<foreign>"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => *a == *b,
            (Value::Integer(a), Value::Integer(b)) => *a == *b,
            (Value::Integer(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Integer(b)) => *a == (*b as f64),
            (Value::Float(a), Value::Float(b)) => *a == *b,
            (Value::String(a), Value::String(b)) => *a == *b,
            (Value::Char(a), Value::Char(b)) => *a == *b,
            (Value::List(a), Value::List(b)) => *a == *b,
            (Value::Vector(a), Value::Vector(b)) => *a == *b,
            (Value::Symbol(a), Value::Symbol(b)) => *a == *b,
            (Value::Closure { params: a, body: x }, Value::Closure { params: b, body: y }) => {
                a == b && x == y
            }
            (Value::Function(a), Value::Function(b)) => *a == *b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::Null) => a.is_empty(),
            (Value::Null, Value::List(b)) => b.is_empty(),
            _ => false,
        }
    }
}

// for turning a scamper value into a rust value
pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>
    where
        Self: Sized;
}

// for turning a rust value into a scamper value
pub trait IntoValue: Sized {
    fn into_value(self) -> Option<Value>;
}

impl FromValue for Value {
    fn from_value(value: &Value) -> Option<Self> {
        Some(value.clone())
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Option<Value> {
        Some(self)
    }
}

impl FromValue for bool {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Option<Value> {
        Some(Value::Boolean(self))
    }
}

impl FromValue for i64 {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(i) => Some(*i),
            Value::Float(f) => {
                if f.fract() == 0.0 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Option<Value> {
        Some(Value::Integer(self))
    }
}

impl FromValue for f64 {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Option<Value> {
        Some(Value::Float(self))
    }
}

impl FromValue for String {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl IntoValue for String {
    fn into_value(self) -> Option<Value> {
        Some(Value::String(self))
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Option<Value> {
        Some(Value::String(String::from(self)))
    }
}
