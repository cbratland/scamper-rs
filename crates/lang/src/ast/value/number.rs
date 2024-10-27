use super::{FromValue, IntoValue, Value};
use std::ops::{Add, Div, Mul, Rem, Sub};

/// A generic number type that can be converted to and from `Value`.
#[derive(Copy, Clone)]
pub struct Number(f64, /* is float */ bool);

impl Number {
    pub fn new(value: f64) -> Self {
        Number(value, value.fract() != 0.0)
    }

    pub fn as_f64(self) -> f64 {
        self.into()
    }

    pub fn abs(self) -> Self {
        Number(self.0.abs(), self.1)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =
impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Number> for f64 {
    fn eq(&self, other: &Number) -> bool {
        *self == other.0
    }
}

// +
impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Number(self.0 + other.0, self.1 || other.1)
    }
}

impl Add<f64> for Number {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Number::new(self.0 + other)
    }
}

impl Add<Number> for f64 {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        Number::new(self + other.0)
    }
}

// -
impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Number(self.0 - other.0, self.1 || other.1)
    }
}

impl Sub<f64> for Number {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Number::new(self.0 - other)
    }
}

impl Sub<Number> for f64 {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        Number::new(self - other.0)
    }
}

// *
impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Number::new(self.0 * other.0)
    }
}

impl Mul<f64> for Number {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Number::new(self.0 * other)
    }
}

impl Mul<Number> for f64 {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        Number::new(self * other.0)
    }
}

// /
impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Number::new(self.0 / other.0)
    }
}

impl Div<f64> for Number {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Number::new(self.0 / other)
    }
}

impl Div<Number> for f64 {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        Number::new(self / other.0)
    }
}

// %
impl Rem for Number {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Number::new(self.0 % other.0)
    }
}

impl Rem<f64> for Number {
    type Output = Self;

    fn rem(self, other: f64) -> Self {
        Number::new(self.0 % other)
    }
}

impl Rem<Number> for f64 {
    type Output = Number;

    fn rem(self, other: Number) -> Number {
        Number::new(self % other.0)
    }
}

// <, <=, >, >=
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<f64> for Number {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Number> for f64 {
    fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

// conversions
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::new(value)
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl FromValue for Number {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(i) => Some(Number(*i as f64, false)),
            Value::Float(f) => Some(Number(*f, true)),
            _ => None,
        }
    }
}

impl IntoValue for Number {
    fn into_value(self) -> Option<Value> {
        if self.1 {
            Some(Value::Float(self.0))
        } else {
            Some(Value::Integer(self.0 as i64))
        }
    }
}
