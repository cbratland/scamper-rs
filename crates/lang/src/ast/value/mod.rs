use super::Block;
use crate::interpreter::{Env, RuntimeError};
use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

mod function;
mod list;
mod number;
pub use function::Function;
pub use list::{List, Vector};
pub use number::{NonNegative, Number};

pub type NativeFnSignature = dyn Fn(&[Value]) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct NativeFn(pub Rc<NativeFnSignature>);

impl NativeFn {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&[Value]) -> Result<Value, RuntimeError> + 'static,
    {
        Self(Rc::new(f))
    }
}

impl Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function>")
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Block,
    pub env: Option<Rc<RefCell<Env>>>,
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub kind: String,
    pub fields: Vec<String>,
    pub values: Vec<Value>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.values == other.values
    }
}

pub trait Contract {
    fn check(&self, value: &Value) -> bool;
    fn name(&self) -> &'static str;
}

impl Struct {
    // defaults is an array of things of length n covering the last n field values
    pub fn add_to(
        &self,
        env: &mut Env,
        types: Option<Vec<Option<Box<dyn Contract>>>>,
        defaults: Option<Vec<Value>>,
    ) {
        let pred_id = self.kind.clone();
        let pred_fn = move |args: &[Value]| {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    format!("expected 1 argument, found {}", args.len()),
                    None,
                ));
            }
            Ok(Value::Boolean(match &args[0] {
                Value::Struct(s) => s.kind == pred_id,
                _ => false,
            }))
        };
        env.set(
            format!("{}?", self.kind),
            Value::Function(NativeFn::new(pred_fn)),
        );

        let ctor_id = self.kind.clone();
        let ctor_fields = self.fields.clone();
        let ctor_defaults = defaults.clone();
        let defaults_len = ctor_defaults.as_ref().map_or(0, |f| f.len());
        let ctor_types = types;
        let ctor_fn = move |args: &[Value]| {
            let max_args = ctor_fields.len();
            let min_args = ctor_fields.len() - defaults_len;
            if args.len() < min_args || args.len() > max_args {
                if min_args == max_args {
                    return Err(RuntimeError::new(
                        format!("expected {max_args} arguments, found {}", args.len()),
                        None,
                    ));
                } else {
                    return Err(RuntimeError::new(
                        format!(
                            "expected {min_args} to {max_args} arguments, found {}",
                            args.len()
                        ),
                        None,
                    ));
                }
            }

            if let Some(types) = &ctor_types {
                for (idx, arg) in args.iter().enumerate() {
                    if idx >= types.len() {
                        break;
                    }
                    if let Some(typ) = &types[idx] {
                        if !typ.check(arg) {
                            return Err(RuntimeError::new(
                                format!("argument {} must be a {}", idx + 1, typ.name()),
                                None,
                            ));
                        }
                    }
                }
            }

            // map in the defaults with args so that values is length max_args
            let values = args
                .iter()
                .cloned()
                .chain(ctor_defaults.clone().unwrap_or_default())
                .take(max_args)
                .collect();
            Ok(Value::Struct(Struct {
                kind: ctor_id.clone(),
                fields: ctor_fields.clone(),
                values,
            }))
        };
        env.set(self.kind.clone(), Value::Function(NativeFn::new(ctor_fn)));

        for (field_idx, field) in self.fields.iter().enumerate() {
            let field_name = format!("{}-{field}", self.kind);
            let field_id = self.kind.clone();
            let field_fn = move |args: &[Value]| {
                if args.len() != 1 {
                    return Err(RuntimeError::new(
                        format!("expected 1 argument, found {}", args.len()),
                        None,
                    ));
                }
                match &args[0] {
                    Value::Struct(s) => {
                        if s.kind == field_id {
                            return Ok(s.values[field_idx].clone());
                        }
                    }
                    _ => {}
                };
                Err(RuntimeError::new(
                    format!("expected {} struct", field_id),
                    None,
                ))
            };
            env.set(field_name, Value::Function(NativeFn::new(field_fn)));
        }
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
    Closure(Closure),
    Struct(Struct),
    Function(NativeFn), // rust function
    Foreign(Rc<dyn Any>),
    Null,
    Void,
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            Value::Void => false,
            _ => true,
        }
    }

    pub fn numeric(&self) -> Option<f64> {
        match self {
            Value::Integer(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
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
                write!(f, "(list")?;
                for item in l {
                    write!(f, " {}", item)?;
                }
                write!(f, ")")
            }
            Value::Vector(v) => {
                write!(f, "(vector")?;
                for item in v {
                    write!(f, " {}", item)?;
                }
                write!(f, ")")
            }
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Closure(c) => {
                write!(f, "(lambda ({}) <body>)", c.params.join(" "))
            }
            Value::Struct(s) => {
                write!(f, "({}", s.kind)?;
                for value in &s.values {
                    write!(f, " {}", value)?;
                }
                write!(f, ")")
            }
            Value::Function(_) => write!(f, "<function>"),
            Value::Foreign(_) => write!(f, "<foreign>"),
            Value::Null => write!(f, "null"),
            Value::Void => write!(f, "void"),
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
            (Value::Closure(a), Value::Closure(b)) => *a == *b,
            (Value::Struct(a), Value::Struct(b)) => *a == *b,
            (Value::Function(a), Value::Function(b)) => *a == *b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::Null) => a.is_empty(),
            (Value::Foreign(a), Value::Foreign(b)) => Rc::ptr_eq(a, b),
            (Value::Null, Value::List(b)) => b.is_empty(),
            (Value::Void, Value::Void) => true,
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

impl FromValue for char {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Char(c) => Some(*c),
            _ => None,
        }
    }
}

impl IntoValue for char {
    fn into_value(self) -> Option<Value> {
        Some(Value::Char(self))
    }
}
