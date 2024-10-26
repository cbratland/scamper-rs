use core::f64;

use scamper_macros::function;

use crate::{
    ast::{Number, Value},
    interpreter::{Env, RuntimeError},
};

pub fn add_to(env: &mut Env) {
    env.register("equal?", equal_q);
    env.register("number?", number_q);
    env.register("integer?", integer_q);

    env.register("zero?", zero_q);
    env.register("positive?", positive_q);
    env.register("negative?", negative_q);
    env.register("odd?", odd_q);
    env.register("even?", even_q);

    env.register("<", less_than);
    env.register("<=", less_than_eq);
    env.register(">", greater_than);
    env.register(">=", greater_than_eq);
    env.register("=", equal);
    env.register("+", plus);
    env.register("-", minus);
    env.register("*", times);
    env.register("/", divide);

    env.register("max", max);
    env.register("min", min);

    env.register("abs", abs);
    env.register("quotient", quotient);
    env.register("remainder", remainder);
    env.register("modulo", modulo);

    env.register("pair?", pair_q);
    env.register("pair", cons);
    env.register("cons", cons);
    env.register("car", car);
    env.register("cdr", cdr);
    env.register("list?", list_q);
    env.register("list", list);
    env.register("length", length);

    // additional constants
    env.register_value("else", true);
    env.register_value("null", Value::Null);
    env.register_value("pi", f64::consts::PI);
    env.register_value("π", f64::consts::PI);
    // todo: void
}

#[function]
fn equal_q(x: Value, y: Value) -> bool {
    x == y
}

#[function]
fn number_q(x: Value) -> bool {
    matches!(x, Value::Integer(_) | Value::Float(_))
}

#[function]
fn integer_q(x: Value) -> bool {
    match x {
        Value::Integer(_) => true,
        Value::Float(f) => f.fract() == 0.0,
        _ => false,
    }
}

#[function]
fn zero_q(x: Number) -> bool {
    x == 0.0
}

#[function]
fn positive_q(x: Number) -> bool {
    x > 0.0
}

#[function]
fn negative_q(x: Number) -> bool {
    x < 0.0
}

#[function]
fn odd_q(x: i64) -> bool {
    x % 2 != 0
}

#[function]
fn even_q(x: i64) -> bool {
    x % 2 == 0
}

#[function]
fn less_than(x: Number, y: Number) -> bool {
    x < y
}

#[function]
fn less_than_eq(x: Number, y: Number) -> bool {
    x <= y
}

#[function]
fn greater_than(x: Number, y: Number) -> bool {
    x > y
}

#[function]
fn greater_than_eq(x: Number, y: Number) -> bool {
    x >= y
}

#[function]
fn equal(x: Number, y: Number) -> bool {
    x == y
}

#[function]
fn plus(args: &[Number]) -> Number {
    let mut sum: Number = 0.0.into();
    for arg in args {
        sum = sum + *arg;
    }
    sum
}

#[function]
fn minus(args: &[Number]) -> Result<Number, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError {
            message: "Expected at least one argument".to_string(),
            span: None,
        });
    }
    if args.len() == 1 {
        return Ok(0.0 - args[0]);
    }
    let mut difference = args[0];
    for i in 1..args.len() {
        difference = difference - args[i];
    }
    Ok(difference)
}

#[function]
fn times(args: &[Number]) -> Number {
    let mut product: Number = 1.0.into();
    for arg in args {
        product = product * *arg;
    }
    product
}

#[function]
fn divide(args: &[Number]) -> Result<Number, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError {
            message: "Expected at least one argument".to_string(),
            span: None,
        });
    }
    let mut quotient = args[0];
    for i in 1..args.len() {
        if args[i] == 0.0 {
            return Err(RuntimeError {
                message: "Division by zero".to_string(),
                span: None,
            });
        }
        quotient = quotient / args[i];
    }
    Ok(quotient)
}

#[function]
fn max(args: &[Number]) -> Number {
    let mut max = f64::NEG_INFINITY.into();
    for arg in args {
        if *arg > max {
            max = *arg;
        }
    }
    max
}

#[function]
fn min(args: &[Number]) -> Number {
    let mut min = f64::INFINITY.into();
    for arg in args {
        if *arg < min {
            min = *arg;
        }
    }
    min
}

#[function]
fn abs(x: Number) -> Number {
    x.abs()
}

#[function]
fn quotient(x: Number, y: Number) -> Value {
    let result: f64 = (x / y).into();
    Value::Integer(result as i64)
}

#[function]
fn remainder(x: Number, y: Number) -> Value {
    let result: f64 = (x % y).into();
    Value::Integer(result as i64)
}

#[function]
fn modulo(x: Number, y: Number) -> Number {
    x % y
}

#[function]
fn pair_q(x: Value) -> bool {
    matches!(x, Value::Pair(_, _))
}

#[function]
fn cons(x: Value, y: Value) -> Value {
    match y {
        Value::Null => Value::List(vec![x]),
        Value::List(mut values) => {
            values.insert(0, x);
            Value::List(values)
        }
        _ => Value::Pair(Box::new(x), Box::new(y)),
    }
}

#[function]
fn car(pair: Value) -> Result<Value, RuntimeError> {
    match pair {
        Value::Pair(x, _) => return Ok(*x),
        Value::List(values) => {
            if let Some(value) = values.first() {
                return Ok(value.clone());
            }
        }
        _ => {}
    }
    Err(RuntimeError {
        message: "Expected a pair".to_string(),
        span: None,
    })
}

#[function]
fn cdr(pair: Value) -> Result<Value, RuntimeError> {
    match pair {
        Value::Pair(_, y) => return Ok(*y),
        Value::List(values) => {
            if values.len() > 1 {
                return Ok(Value::List(values[1..].to_vec()));
            }
        }
        _ => {}
    }
    Err(RuntimeError {
        message: "Expected a pair".to_string(),
        span: None,
    })
}

#[function]
fn list_q(x: Value) -> bool {
    matches!(x, Value::List(_))
}

#[function]
fn list(values: &[Value]) -> Value {
    Value::List(values.to_vec())
}

#[function]
fn make_list(n: i64, value: Value) -> Value {
    Value::List(vec![value; n as usize])
}

#[function]
fn length(list: Value) -> Result<i64, RuntimeError> {
    match list {
        Value::List(values) => Ok(values.len() as i64),
        Value::Null => Ok(0),
        _ => Err(RuntimeError {
            message: "Expected a list".to_string(),
            span: None,
        }),
    }
}
