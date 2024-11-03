use crate::{
    ast::{Closure, FromValue, Function, List, Number, Operation, Span, Value, Vector},
    interpreter::{Env, RuntimeError},
};
use core::f64;
use scamper_macros::function;

pub fn add_to(env: &mut Env) {
    // numbers (6.2)
    env.register("equal?", equal_q);
    env.register("number?", number_q);
    env.register("real?", real_q);
    env.register("integer?", integer_q);
    env.register("nan?", nan_q);

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
    env.register("floor", floor);
    env.register("ceiling", ceiling);
    env.register("truncate", truncate);
    env.register("round", round);
    env.register("square", square);
    env.register("sqrt", sqrt);
    env.register("expt", expt);
    env.register("number->string", number_to_string);
    env.register("string->number", string_to_number);
    env.register("exp", exp);
    env.register("log", log);
    env.register("sin", sin);
    env.register("cos", cos);
    env.register("tan", tan);
    env.register("asin", asin);
    env.register("acos", acos);
    env.register("atan", atan);

    // booleans (6.3)
    env.register("boolean?", boolean_q);
    // env.register("=-eps", eq_eps);
    env.register("not", not);
    env.register("nand", nand);
    env.register("nor", nor);
    env.register("implies", implies);
    env.register("xor", xor);

    // pairs and lists (6.4)
    env.register("pair?", pair_q);
    env.register("pair", cons);
    env.register("cons", cons);
    env.register("car", car);
    env.register("cdr", cdr);
    env.register("null?", null_q);
    env.register("list?", list_q);
    env.register("list", list);
    env.register("make-list", make_list);
    env.register("length", length);
    env.register("append", append);
    env.register("reverse", reverse);
    env.register("list-tail", list_tail);
    env.register("list-drop", list_tail);
    env.register("list-take", list_take);
    env.register("list-ref", list_ref);
    env.register("index-of", index_of);
    env.register("assoc-key?", assoc_key);
    env.register("assoc-ref", assoc_ref);
    env.register("assoc-set", assoc_set);

    // characters (6.6)
    env.register("char?", char_q);
    env.register("char=?", char_eq);
    env.register("char<?", char_lt);
    env.register("char>?", char_gt);
    env.register("char<=?", char_le);
    env.register("char>=?", char_ge);
    env.register("char-ci=?", char_ci_eq);
    env.register("char-ci<?", char_ci_lt);
    env.register("char-ci>?", char_ci_gt);
    env.register("char-ci<=?", char_ci_le);
    env.register("char-ci>=?", char_ci_ge);
    env.register("char-alphabetic?", char_alphabetic_q);
    env.register("char-numeric?", char_numeric_q);
    env.register("char-whitespace?", char_whitespace_q);
    env.register("char-upper-case?", char_upper_case_q);
    env.register("char-lower-case?", char_lower_case_q);
    env.register("digit-value", digit_value);
    env.register("char->integer", char_to_integer);
    env.register("integer->char", integer_to_char);
    env.register("char-upcase", char_upcase);
    env.register("char-downcase", char_downcase);
    env.register("char-foldcase", char_downcase);

    // strings (6.7)
    env.register("string?", string_q);
    env.register("make-string", make_string);
    env.register("string", string);
    env.register("string-length", string_length);
    env.register("string-ref", string_ref);
    env.register("string=?", string_eq);
    env.register("string<?", string_lt);
    env.register("string>?", string_gt);
    env.register("string<=?", string_le);
    env.register("string>=?", string_ge);
    env.register("string-ci=?", string_ci_eq);
    env.register("string-ci<?", string_ci_lt);
    env.register("string-ci>?", string_ci_gt);
    env.register("string-ci<=?", string_ci_le);
    env.register("string-ci>=?", string_ci_ge);
    env.register("string-upcase", string_upcase);
    env.register("string-downcase", string_downcase);
    env.register("string-foldcase", string_downcase);
    env.register("substring", substring);
    env.register("string-append", string_append);
    env.register("string->list", string_to_list);
    env.register("list->string", list_to_string);
    env.register("string->vector", string_to_vector);
    env.register("vector->string", vector_to_string);
    env.register("string-contains", string_contains);
    env.register("string-split", string_split);
    env.register("string-split-vector", string_split_vector);

    // env.register("with-file", with_file);

    // vectors (6.8)
    env.register("vector?", vector_q);
    env.register("vector", vector);
    env.register("make-vector", make_vector);
    env.register("vector-length", vector_length);
    env.register("vector-ref", vector_ref);
    // env.register("vector-set!", vector_set);
    // env.register("vector-fill!", vector_fill);
    env.register("vector->list", vector_to_list);
    env.register("list->vector", list_to_vector);
    env.register("vector-range", vector_range);
    env.register("vector-append", vector_append);

    // control features (6.10)
    env.register("procedure?", procedure_q);
    env.register("apply", apply);
    env.register("string-map", string_map);
    env.register("map", map);
    env.register("filter", filter);
    env.register("fold", fold_left);
    env.register("fold-left", fold_left);
    env.register("fold-right", fold_right);
    env.register("reduce", reduce);
    env.register("reduce-right", reduce_right);
    env.register("vector-map", vector_map);
    // env.register("vector-map!", vector_map_bang);
    env.register("vector-for-each", vector_for_each);
    env.register("for-range", for_range);
    env.register("vector-filter", vector_filter);

    env.register("void?", void_q);
    env.register("error", error);
    env.register("??", qq);
    env.register("compose", compose);
    env.register("o", compose);
    env.register("|>", pipe);
    env.register("range", range);
    env.register("random", random);
    // env.register("with-handler", with_handler);
    // env.register("ignore", ignore); // needs custom type/renderer

    // additional constants
    env.register_value("else", true);
    env.register_value("null", Value::Null);
    env.register_value("pi", f64::consts::PI);
    env.register_value("π", f64::consts::PI);
    env.register_value("void", Value::Void);
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
fn real_q(x: Value) -> bool {
    match x {
        Value::Integer(_) => false,
        Value::Float(f) => f.fract() != 0.0,
        _ => false,
    }
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
fn nan_q(x: Value) -> bool {
    match x {
        Value::Float(f) => f.is_nan(),
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
        return Err(RuntimeError::new(
            "Expected at least one argument".to_string(),
            None,
        ));
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
        return Err(RuntimeError::new(
            "Expected at least one argument".to_string(),
            None,
        ));
    }
    let mut quotient = args[0];
    for i in 1..args.len() {
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
fn floor(x: Number) -> Number {
    f64::floor(x.into()).into()
}

#[function]
fn ceiling(x: Number) -> Number {
    f64::ceil(x.into()).into()
}

#[function]
fn truncate(x: Number) -> Number {
    f64::trunc(x.into()).into()
}

#[function]
fn round(x: Number) -> Number {
    f64::round(x.into()).into()
}

#[function]
fn square(x: Number) -> Number {
    x * x
}

#[function]
fn sqrt(x: Number) -> Number {
    f64::sqrt(x.into()).into()
}

#[function]
fn expt(x: Number, y: Number) -> Number {
    let x_float: f64 = x.into();
    x_float.powf(y.into()).into()
}

#[function]
fn number_to_string(x: Number) -> String {
    x.to_string()
}

#[function]
fn string_to_number(x: String) -> Result<Value, RuntimeError> {
    // check if number is integer or float
    if x.contains('.') {
        if let Ok(n) = x.parse::<f64>() {
            return Ok(Value::Float(n));
        }
    } else if let Ok(n) = x.parse::<i64>() {
        return Ok(Value::Integer(n));
    }
    Err(RuntimeError::new(format!("invalid string: {}", x), None))
}

#[function]
fn exp(x: Number) -> Number {
    f64::exp(x.into()).into()
}

#[function]
fn log(x: Number) -> Number {
    f64::log10(x.into()).into()
}

#[function]
fn sin(x: Number) -> Number {
    f64::sin(x.into()).into()
}

#[function]
fn cos(x: Number) -> Number {
    f64::cos(x.into()).into()
}

#[function]
fn tan(x: Number) -> Number {
    f64::tan(x.into()).into()
}

#[function]
fn asin(x: Number) -> Number {
    f64::asin(x.into()).into()
}

#[function]
fn acos(x: Number) -> Number {
    f64::acos(x.into()).into()
}

#[function]
fn atan(x: Number) -> Number {
    f64::atan(x.into()).into()
}

#[function]
fn boolean_q(x: Value) -> bool {
    match x {
        Value::Boolean(_) => true,
        _ => false,
    }
}

#[function]
fn not(x: bool) -> bool {
    !x
}

#[function]
fn nand(args: &[bool]) -> bool {
    args.iter().any(|&x| !x)
}

#[function]
fn nor(args: &[bool]) -> bool {
    !args.iter().any(|&x| x)
}

#[function]
fn implies(x: bool, y: bool) -> bool {
    !x || y
}

#[function]
fn xor(x: bool, y: bool) -> bool {
    (x && !y) || (!x && y)
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
    Err(RuntimeError::new("Expected a pair".to_string(), None))
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
    Err(RuntimeError::new("Expected a pair".to_string(), None))
}

#[function]
fn null_q(x: Value) -> bool {
    matches!(x, Value::Null)
}

#[function]
fn list_q(x: Value) -> bool {
    matches!(x, Value::List(_) | Value::Null)
}

#[function]
fn list(values: &[Value]) -> Value {
    if values.is_empty() {
        Value::Null
    } else {
        Value::List(values.to_vec())
    }
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
        _ => Err(RuntimeError::new("Expected a list".to_string(), None)),
    }
}

#[function]
fn append(args: &[Value]) -> Result<Value, RuntimeError> {
    let Some(first) = args.first() else {
        return Err(RuntimeError::new(
            "Expected at least one argument".to_string(),
            None,
        ));
    };

    let mut total = match first {
        Value::List(values) => values.clone(),
        Value::Null => Vec::new(),
        _ => {
            return Err(RuntimeError::new(
                "Expected first argument to be a list".to_string(),
                None,
            ));
        }
    };

    for arg in args.to_vec().into_iter().skip(1) {
        match arg {
            Value::List(values) => total.extend(values),
            Value::Null => {}
            _ => {
                return Err(RuntimeError::new(
                    "Expected all arguments to be lists".to_string(),
                    None,
                ));
            }
        }
    }

    if total.is_empty() {
        Ok(Value::Null)
    } else {
        Ok(Value::List(total))
    }
}

#[function]
fn reverse(l: List) -> List {
    let vec: Vec<Value> = l.into();
    List::from(vec.into_iter().rev().collect::<Vec<_>>())
}

// Returns l but with the first k elements of l omitted.
#[function]
fn list_tail(l: List, k: i64) -> Result<List, RuntimeError> {
    if k < 0 {
        return Err(RuntimeError::new(
            "Expected argument 2 to be a non-negative integer".to_string(),
            None,
        ));
    }
    if l.is_empty() {
        return Ok(l);
    }
    if k >= l.len() as i64 {
        Ok(List::empty())
    } else {
        let vec: Vec<Value> = l.into();
        Ok(List::from(
            vec.into_iter().skip(k as usize).collect::<Vec<_>>(),
        ))
    }
}

#[function]
fn list_take(l: List, k: i64) -> Result<List, RuntimeError> {
    if k < 0 {
        return Err(RuntimeError::new(
            "Expected argument 2 to be a non-negative integer".to_string(),
            None,
        ));
    }
    if l.is_empty() {
        return Ok(l);
    }
    if k >= l.len() as i64 {
        Ok(l)
    } else {
        let vec: Vec<Value> = l.into();
        Ok(List::from(
            vec.into_iter().take(k as usize).collect::<Vec<_>>(),
        ))
    }
}

#[function]
fn list_ref(l: List, n: i64) -> Result<Value, RuntimeError> {
    if n < 0 {
        return Err(RuntimeError::new(
            "Expected argument 2 to be a non-negative integer".to_string(),
            None,
        ));
    }
    let vec: Vec<Value> = l.into();
    if n >= vec.len() as i64 {
        return Err(RuntimeError::new("Index out of bounds".to_string(), None));
    }
    Ok(vec[n as usize].clone())
}

#[function]
fn index_of(l: List, value: Value) -> Result<i64, RuntimeError> {
    let vec: Vec<Value> = l.into();
    for (i, v) in vec.iter().enumerate() {
        if v == &value {
            return Ok(i as i64);
        }
    }
    Ok(-1)
}

#[function]
fn assoc_key(v: Value, l: List) -> Result<bool, RuntimeError> {
    let vec: Vec<Value> = l.into();
    for pair in vec {
        if let Value::Pair(x, _) = pair {
            if *x == v {
                return Ok(true);
            }
        } else {
            return Err(RuntimeError::new(
                "Expected a list of pairs".to_string(),
                None,
            ));
        }
    }
    Ok(false)
}

#[function]
fn assoc_ref(v: Value, l: List) -> Result<Value, RuntimeError> {
    let vec: Vec<Value> = l.into();
    for pair in vec {
        if let Value::Pair(x, y) = pair {
            if *x == v {
                return Ok(*y);
            }
        } else {
            return Err(RuntimeError::new(
                "Expected a list of pairs".to_string(),
                None,
            ));
        }
    }
    Err(RuntimeError::new(
        format!("assoc-ref: key {v} not found in association list"),
        None,
    ))
}

// Returns a new association list containing the same key-value pairs as l except that k is associated with v.
#[function]
fn assoc_set(k: Value, v: Value, l: List) -> Result<List, RuntimeError> {
    let vec: Vec<Value> = l.into();
    let mut new_vec = Vec::new();
    let mut found = false;
    for pair in vec {
        if let Value::Pair(x, y) = pair {
            if *x == k {
                new_vec.push(Value::Pair(Box::new(k.clone()), Box::new(v.clone())));
                found = true;
            } else {
                new_vec.push(Value::Pair(x, y));
            }
        } else {
            return Err(RuntimeError::new(
                "Expected a list of pairs".to_string(),
                None,
            ));
        }
    }
    if !found {
        new_vec.push(Value::Pair(Box::new(k), Box::new(v)));
    }
    Ok(List::from(new_vec))
}

#[function]
fn char_q(v: Value) -> bool {
    matches!(v, Value::Char(_))
}

#[function]
fn char_eq(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0] == w[1])
}

#[function]
fn char_lt(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0] < w[1])
}

#[function]
fn char_gt(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0] > w[1])
}

#[function]
fn char_le(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0] <= w[1])
}

#[function]
fn char_ge(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0] >= w[1])
}

#[function]
fn char_ci_eq(chars: &[char]) -> bool {
    chars.windows(2).all(|w| w[0].eq_ignore_ascii_case(&w[1]))
}

#[function]
fn char_ci_lt(chars: &[char]) -> bool {
    chars
        .windows(2)
        .all(|w| w[0].to_ascii_lowercase() < w[1].to_ascii_lowercase())
}

#[function]
fn char_ci_gt(chars: &[char]) -> bool {
    chars
        .windows(2)
        .all(|w| w[0].to_ascii_lowercase() > w[1].to_ascii_lowercase())
}

#[function]
fn char_ci_le(chars: &[char]) -> bool {
    chars
        .windows(2)
        .all(|w| w[0].to_ascii_lowercase() <= w[1].to_ascii_lowercase())
}

#[function]
fn char_ci_ge(chars: &[char]) -> bool {
    chars
        .windows(2)
        .all(|w| w[0].to_ascii_lowercase() >= w[1].to_ascii_lowercase())
}

#[function]
fn char_alphabetic_q(c: char) -> bool {
    c.is_alphabetic()
}

#[function]
fn char_numeric_q(c: char) -> bool {
    c.is_numeric()
}

#[function]
fn char_whitespace_q(c: char) -> bool {
    c.is_whitespace()
}

#[function]
fn char_upper_case_q(c: char) -> bool {
    c.is_uppercase()
}

#[function]
fn char_lower_case_q(c: char) -> bool {
    c.is_lowercase()
}

#[function]
fn digit_value(c: char) -> Result<i64, RuntimeError> {
    if c.is_digit(10) {
        Ok(c.to_digit(10).unwrap() as i64)
    } else {
        Err(RuntimeError::new(
            format!("digit-value: {c} is not a decimal digit"),
            None,
        ))
    }
}

#[function]
fn char_to_integer(c: char) -> i64 {
    c as i64
}

#[function]
fn integer_to_char(i: i64) -> Result<char, RuntimeError> {
    if i >= 0 && i <= std::char::MAX as i64 {
        Ok(i as u8 as char)
    } else {
        Err(RuntimeError::new(
            format!("integer->char: {i} is not in the range of a character"),
            None,
        ))
    }
}

#[function]
fn char_upcase(c: char) -> char {
    c.to_ascii_uppercase()
}

#[function]
fn char_downcase(c: char) -> char {
    c.to_ascii_lowercase()
}

#[function]
fn string_q(v: Value) -> bool {
    matches!(v, Value::String(_))
}

#[function]
fn make_string(n: i64, c: char) -> Result<String, RuntimeError> {
    if n >= 0 {
        Ok(c.to_string().repeat(n as usize))
    } else {
        Err(RuntimeError::new(
            format!("make-string: {n} should be positive"),
            None,
        ))
    }
}

#[function]
fn string(v: &[char]) -> String {
    v.iter().collect()
}

#[function]
fn string_length(s: String) -> i64 {
    s.len() as i64
}

#[function]
fn string_ref(s: String, i: i64) -> Result<char, RuntimeError> {
    if i >= 0 && i < s.len() as i64 {
        Ok(s.chars().nth(i as usize).unwrap())
    } else {
        // scamper (upstream) returns #\undefined (?)
        Err(RuntimeError::new(
            format!("string-ref: index {i} out of bounds"),
            None,
        ))
    }
}

#[function]
fn string_eq(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0] == w[1])
}

#[function]
fn string_lt(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0] < w[1])
}

#[function]
fn string_gt(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0] > w[1])
}

#[function]
fn string_le(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0] <= w[1])
}

#[function]
fn string_ge(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0] >= w[1])
}

#[function]
fn string_ci_eq(strs: &[String]) -> bool {
    strs.windows(2).all(|w| w[0].eq_ignore_ascii_case(&w[1]))
}

#[function]
fn string_ci_lt(strs: &[String]) -> bool {
    strs.windows(2)
        .all(|w| w[0].to_ascii_lowercase() < w[1].to_ascii_lowercase())
}

#[function]
fn string_ci_gt(strs: &[String]) -> bool {
    strs.windows(2)
        .all(|w| w[0].to_ascii_lowercase() > w[1].to_ascii_lowercase())
}

#[function]
fn string_ci_le(strs: &[String]) -> bool {
    strs.windows(2)
        .all(|w| w[0].to_ascii_lowercase() <= w[1].to_ascii_lowercase())
}

#[function]
fn string_ci_ge(strs: &[String]) -> bool {
    strs.windows(2)
        .all(|w| w[0].to_ascii_lowercase() >= w[1].to_ascii_lowercase())
}

#[function]
fn string_upcase(s: String) -> String {
    s.to_ascii_uppercase()
}

#[function]
fn string_downcase(s: String) -> String {
    s.to_ascii_lowercase()
}

#[function]
fn substring(s: String, start: i64, end: i64) -> Result<String, RuntimeError> {
    if start >= 0 && end >= 0 && start <= end && end <= s.len() as i64 {
        Ok(s.chars()
            .skip(start as usize)
            .take((end - start) as usize)
            .collect())
    } else {
        Err(RuntimeError::new(
            format!("substring: invalid start {start} or end {end}"),
            None,
        ))
    }
}

#[function]
fn string_append(strings: &[String]) -> String {
    strings.join("")
}

#[function]
fn string_to_list(s: String) -> List {
    List::from(s.chars().map(|c| Value::Char(c)).collect::<Vec<_>>())
}

#[function]
fn list_to_string(l: List) -> Result<String, RuntimeError> {
    let mut s = String::new();
    for v in l.values().iter() {
        match v {
            Value::Char(c) => s.push(*c),
            _ => {
                return Err(RuntimeError::new(
                    format!("list->string: list contains non-character element: {v}"),
                    None,
                ));
            }
        }
    }
    Ok(s)
}

#[function]
fn string_to_vector(s: String) -> Vector {
    Vector::from(s.chars().map(|c| Value::Char(c)).collect::<Vec<_>>())
}

#[function]
fn vector_to_string(v: Vector) -> Result<String, RuntimeError> {
    let mut s = String::new();
    for c in v.values().iter() {
        match c {
            Value::Char(c) => s.push(*c),
            _ => {
                return Err(RuntimeError::new(
                    format!("vector->string: vector contains non-character element: {c}"),
                    None,
                ));
            }
        }
    }
    Ok(s)
}

#[function]
fn string_contains(s: String, sub: String) -> bool {
    s.contains(&sub)
}

#[function]
fn string_split(s: String, sep: String) -> List {
    List::from(
        s.split(&sep)
            .map(|s| Value::String(s.to_string()))
            .collect::<Vec<_>>(),
    )
}

#[function]
fn string_split_vector(s: String, sep: String) -> Vector {
    Vector::from(
        s.split(&sep)
            .map(|s| Value::String(s.to_string()))
            .collect::<Vec<_>>(),
    )
}

#[function]
fn vector_q(v: Value) -> bool {
    matches!(v, Value::Vector(_))
}

#[function]
fn vector(v: &[Value]) -> Vector {
    v.to_vec().into()
}

#[function]
fn make_vector(n: i64, fill: Value) -> Vector {
    vec![fill; n as usize].into()
}

#[function]
fn vector_length(v: Vector) -> i64 {
    v.len() as i64
}

#[function]
fn vector_ref(v: Vector, i: i64) -> Result<Value, RuntimeError> {
    if i >= 0 && i < v.len() as i64 {
        Ok(v.values()[i as usize].clone())
    } else {
        Err(RuntimeError::new(
            format!("vector-ref: index {i} out of bounds"),
            None,
        ))
    }
}

// #[function]
// fn vector_set(v: Vector, i: i64, x: Value) -> Result<Value, RuntimeError> {
// 	Ok(Value::Void)
// }

// #[function]
// fn vector_fill(v: Vector, x: Value) -> Value {
//     Ok(Value::Void)
// }

#[function]
fn vector_to_list(v: Vector) -> List {
    List::from(v.values_vec())
}

#[function]
fn list_to_vector(l: List) -> Vector {
    Vector::from(l.values_vec())
}

#[function]
fn vector_range(args: &[Number]) -> Result<Vector, RuntimeError> {
    if args.is_empty() || args.len() > 3 {
        return Err(RuntimeError::new(
            format!("1, 2, or 3 numbers must be passed to function"),
            None,
        ));
    }
    let m = if args.len() == 1 {
        0.0
    } else {
        args[0].as_f64()
    };
    let n = if args.len() == 1 {
        args[0].as_f64()
    } else {
        args[1].as_f64()
    };
    let step = if args.len() == 3 {
        args[2].as_f64()
    } else {
        1.0
    };
    if step == 0.0 {
        return Err(RuntimeError::new(
            format!("\"step\" argument must be non-zero"),
            None,
        ));
    }
    let mut v = Vec::new();
    let mut i = m;
    while (step > 0.0 && i < n) || (step < 0.0 && i > n) {
        v.push(Value::Float(i));
        i += step;
    }
    Ok(v.into())
}

#[function]
fn vector_append(vectors: &[Vector]) -> Vector {
    let mut v = Vec::new();
    for vec in vectors.to_vec() {
        v.extend(vec.values_vec());
    }
    v.into()
}

#[function]
fn procedure_q(v: Value) -> bool {
    matches!(v, Value::Function(_, _) | Value::Closure { .. })
}

#[function]
fn apply(f: Function, args: List) -> Result<Value, RuntimeError> {
    f.call(&args.values_vec())
}

#[function]
fn string_map(f: Function, s: String) -> Result<String, RuntimeError> {
    s.chars()
        .map(|c| match f.call(&[Value::Char(c)])? {
            Value::Char(c) => Ok(c),
            _ => Err(RuntimeError::new(
                format!("function must return a character"),
                None,
            )),
        })
        .collect()
}

fn map_prim(f: Function, vectors: Vec<Vec<Value>>) -> Result<Vec<Value>, RuntimeError> {
    let mut v = Vec::new();
    for i in 0..vectors[0].len() {
        let mut args = Vec::new();
        for vec in vectors.iter() {
            args.push(vec[i].clone());
        }
        v.push(f.call(&args)?);
    }
    Ok(v)
}

#[function]
fn map(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::new(
            format!("at least one argument must be passed to function"),
            None,
        ));
    }

    if args.len() == 1 {
        return Ok(Value::Null);
    }

    let f: Function = Function::from_value(&args[0]).ok_or(RuntimeError::new(
        format!("first argument must be a function"),
        None,
    ))?;

    let mut lists = Vec::new();
    for arg in args.iter().skip(1) {
        match arg {
            Value::List(l) => lists.push(l.clone()),
            Value::Null => {
                if !lists.is_empty() {
                    return Err(RuntimeError::new(
                        format!(
                            "the lists passed to the function call do not have the same length"
                        ),
                        None,
                    ));
                }
            }
            _ => {
                return Err(RuntimeError::new(
                    format!("all arguments after the first must be lists"),
                    None,
                ));
            }
        }
    }

    if lists.is_empty() {
        return Ok(Value::Null);
    }

    let list_len = lists[0].len();
    if lists.iter().any(|l| l.len() != list_len) {
        return Err(RuntimeError::new(
            format!("the lists passed to the function call do not have the same length"),
            None,
        ));
    }

    let result = map_prim(f, lists)?;

    if result.is_empty() {
        Ok(Value::Null)
    } else {
        Ok(Value::List(result))
    }
}

#[function]
fn filter(f: Function, lst: List) -> Result<List, RuntimeError> {
    let mut result = Vec::new();
    for value in lst.values() {
        // scamper just uses a js truthy check here, but I think it would be better to ensure the function returns a boolean
        if f.call(&[value.clone()])?.truthy() {
            result.push(value.clone());
        }
    }
    Ok(result.into())
}

#[function]
fn fold_left(f: Function, init: Value, lst: List) -> Result<Value, RuntimeError> {
    let mut acc = init;
    for value in lst.values() {
        acc = f.call(&[acc, value.clone()])?;
    }
    Ok(acc)
}

#[function]
fn fold_right(f: Function, init: Value, lst: List) -> Result<Value, RuntimeError> {
    let mut acc = init;
    for value in lst.values().iter().rev() {
        acc = f.call(&[value.clone(), acc])?;
    }
    Ok(acc)
}

#[function]
fn reduce(f: Function, lst: List) -> Result<Value, RuntimeError> {
    if lst.is_empty() {
        return Err(RuntimeError::new(format!("list must not be empty"), None));
    }
    let mut iter = lst.values_vec().into_iter();
    let mut acc = iter.next().unwrap();
    for value in iter {
        acc = f.call(&[acc, value])?;
    }
    Ok(acc)
}

#[function]
fn reduce_right(f: Function, lst: List) -> Result<Value, RuntimeError> {
    if lst.is_empty() {
        return Err(RuntimeError::new(format!("list must not be empty"), None));
    }
    let mut iter = lst.values_vec().into_iter().rev();
    let mut acc = iter.next().unwrap();
    for value in iter {
        acc = f.call(&[value, acc])?;
    }
    Ok(acc)
}

#[function]
fn vector_map(args: &[Value]) -> Result<Vector, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::new(
            format!("at least one argument must be passed to function"),
            None,
        ));
    }

    if args.len() == 1 {
        return Ok(Vector::empty());
    }

    let f: Function = Function::from_value(&args[0]).ok_or(RuntimeError::new(
        format!("first argument must be a function"),
        None,
    ))?;

    let mut lists = Vec::new();
    for arg in args.iter().skip(1) {
        match arg {
            Value::List(l) => lists.push(l.clone()),
            Value::Null => {
                if !lists.is_empty() {
                    return Err(RuntimeError::new(
							format!("vector-map: the vectors passed to the function call do not have the same length"),
							 None,
								));
                }
            }
            _ => {
                return Err(RuntimeError::new(
                    format!("vector-map: all arguments after the first must be vectors"),
                    None,
                ));
            }
        }
    }

    if lists.is_empty() {
        return Ok(Vector::empty());
    }

    let list_len = lists[0].len();
    if lists.iter().any(|l| l.len() != list_len) {
        return Err(RuntimeError::new(
            format!("the vectors passed to the function call do not have the same length"),
            None,
        ));
    }

    let result = map_prim(f, lists)?;

    Ok(result.into())
}

#[function]
fn vector_for_each(f: Function, vec: Vector) -> Result<Value, RuntimeError> {
    for value in vec.values_vec() {
        f.call(&[value])?;
    }
    Ok(Value::Void)
}

#[function]
fn for_range(start: i64, end: i64, f: Function) -> Result<Value, RuntimeError> {
    if start < end {
        for i in start..end {
            f.call(&[Value::Integer(i)])?;
        }
    } else {
        for i in (end..start).rev() {
            f.call(&[Value::Integer(i)])?;
        }
    }
    Ok(Value::Void)
}

#[function]
fn vector_filter(f: Function, vec: Vector) -> Result<Vector, RuntimeError> {
    let mut result = Vec::new();
    for value in vec.values_vec() {
        if f.call(&[value.clone()])?.truthy() {
            result.push(value);
        }
    }
    Ok(result.into())
}

#[function]
fn void_q(value: Value) -> bool {
    matches!(value, Value::Void)
}

#[function]
fn error(message: String) -> Result<Value, RuntimeError> {
    Err(RuntimeError::new(message, None))
}

#[function]
fn qq() -> Result<Value, RuntimeError> {
    Err(RuntimeError::new(
        "Hole encountered in program!".to_string(),
        None,
    ))
}

// todo: this might have issues idk
#[function]
fn compose(funcs: &[Function]) -> Result<Value, RuntimeError> {
    if funcs.is_empty() {
        return Err(RuntimeError::new(
            "at least one function must be passed".to_string(),
            None,
        ));
    }
    let mut operations: Vec<Operation> = Vec::new();

    for func in funcs.iter() {
        operations.push(Operation::value(func.value(), Span { loc: 0, len: 0 }));
    }
    operations.push(Operation::var(String::from("x"), Span { loc: 0, len: 0 }));
    for _ in funcs.iter().rev() {
        operations.push(Operation::ap(1, Span { loc: 0, len: 0 }));
    }

    Ok(Value::Closure(
        Closure {
            params: vec![String::from("x")],
            body: operations,
            env: None,
        },
        None,
    ))
}

#[function]
fn pipe(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::new(
            "at least two arguments must be passed".to_string(),
            None,
        ));
    }
    let functions = args
        .iter()
        .skip(1)
        .map(|v| {
            Function::from_value(v)
                .ok_or(RuntimeError::new("expected a function".to_string(), None))
        })
        .collect::<Result<Vec<_>, RuntimeError>>()?;
    let mut acc = args[0].clone();
    for f in functions {
        acc = f.call(&[acc])?;
    }
    Ok(acc)
}

#[function]
fn range(args: &[Number]) -> Result<List, RuntimeError> {
    if args.is_empty() || args.len() > 3 {
        return Err(RuntimeError::new(
            "1, 2, or 3 numbers must be passed to function".to_string(),
            None,
        ));
    }
    let m = if args.len() == 1 {
        0.0
    } else {
        args[0].as_f64()
    };
    let n = if args.len() == 1 {
        args[0].as_f64()
    } else {
        args[1].as_f64()
    };
    let step = if args.len() == 3 {
        args[2].as_f64()
    } else {
        1.0
    };
    if step == 0.0 {
        return Err(RuntimeError::new(
            "step argument must be non-zero".to_string(),
            None,
        ));
    }
    let mut result = Vec::new();
    let mut i = m;
    while (step > 0.0 && i < n) || (step < 0.0 && i > n) {
        result.push(Value::Float(i));
        i += step;
    }
    Ok(result.into())
}

// random number between 0 and n
#[function]
fn random(n: i64) -> i64 {
    use rand::Rng;
    rand::thread_rng().gen_range(0..n)
}
