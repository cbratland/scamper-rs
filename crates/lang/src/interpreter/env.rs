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
            .insert(name.to_string(), Value::Function(NativeFn(func)));
    }

    pub fn register_value<T: IntoValue>(&mut self, name: &str, value: T) -> bool {
        let Some(value) = value.into_value() else {
            return false;
        };
        self.bindings.insert(name.to_string(), value);
        return true;
    }
}

// pub struct Wrapper<T>(PhantomData<T>);
// pub struct Wrapper2<T>(PhantomData<T>);

// pub trait RegisterFn<FN, RET, EXTRA> {
//     fn register(&mut self, name: &'static str, func: FN);
// }

// impl<RET: IntoValue, FN: Fn() -> Result<RET, RuntimeError> + 'static>
//     RegisterFn<FN, RET, Wrapper<()>> for Env
// {
//     fn register(&mut self, name: &'static str, func: FN) {
//         self.register_fn(name, move |_| func().map(|v| v.into_value().unwrap()));
//     }
// }

// impl<RET: IntoValue, A: FromValue, FN: Fn(A) -> Result<RET, RuntimeError> + 'static>
//     RegisterFn<FN, RET, Wrapper<A>> for Env
// {
//     fn register(&mut self, name: &'static str, func: FN) {
//         self.register_fn(name, move |args| {
//             func(A::from_value(args[0].clone()).unwrap()).map(|v| v.into_value().unwrap())
//         });
//     }
// }

// impl<
//         RET: IntoValue,
//         A: FromValue,
//         B: FromValue,
//         FN: Fn(A, B) -> Result<RET, RuntimeError> + 'static,
//     > RegisterFn<FN, RET, Wrapper<(A, B)>> for Env
// {
//     fn register(&mut self, name: &'static str, func: FN) {
//         self.register_fn(name, move |args| {
//             func(
//                 A::from_value(args[0].clone()).unwrap(),
//                 B::from_value(args[1].clone()).unwrap(),
//             )
//             .map(|v| v.into_value().unwrap())
//         });
//     }
// }

// impl<RET: IntoValue, FN: Fn(&[Value]) -> Result<RET, RuntimeError> + 'static>
//     RegisterFn<FN, RET, Wrapper2<()>> for Env
// {
//     fn register(&mut self, name: &'static str, func: FN) {
//         self.register_fn(name, move |args| {
//             func(args).map(|v| v.into_value().unwrap())
//         });
//     }
// }

// impl<T: IntoValue, F: Fn() -> Result<T, RuntimeError> + 'static> From<F> for Env {
//     fn from(i: F) -> Self {
//         Self::new(None)
//     }
// }

// impl<T: IntoValue, F: Fn() -> Result<T, RuntimeError> + 'static> From<F> for Env {
//     fn from(i: F) -> Self {
//         Self::new(None)
//     }
// }

// impl<F: Fn(i64, i64) -> Result<i64, RuntimeError> + 'static> From<F> for Env {
//     fn from(i: F) -> Self {
//         Self::new(None)
//     }
// }

// impl<F: Fn(&[Value]) -> Result<Value, RuntimeError> + 'static> From<F> for Env {
//     fn from(i: F) -> Self {
//         Self::new(None)
//     }
// }

// use std::marker::PhantomData;

// // Trait for callable functions
// pub trait Callable {
//     fn call(&self, args: &[Value]) -> Result<Value, RuntimeError>;
// }

// // Type-erased wrapper for native functions
// pub struct NativeFunction {
//     func: Box<dyn Callable>,
// }

// // Implementation for variadic functions that take &[Value]
// impl<F> Callable for F
// where
//     F: Fn(&[Value]) -> Result<Value, RuntimeError>,
// {
//     fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
//         self(args)
//     }
// }

// // Helper struct for functions with fixed arguments
// struct FixedFunction<F, Args, Ret> {
//     f: F,
//     _phantom: PhantomData<(Args, Ret)>,
// }

// // Macro to implement Callable for functions with different arities
// macro_rules! impl_callable {
//     ($($arg:ident),*) => {
//         impl<F, Ret, $($arg,)*> Callable for FixedFunction<F, ($($arg,)*), Ret>
//         where
//             F: Fn($($arg),*) -> Result<Ret, RuntimeError>,
//             $($arg: FromValue,)*
//             Ret: IntoValue,
//         {
//             fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
//                 if args.len() != count!($($arg)*) {
//                     return Err(RuntimeError {
//                         message: format!("Expected {} arguments, got {}", count!($($arg)*), args.len()),
//                         span: None,
//                     });
//                 }

//                 let mut arg_iter = args.iter();
//                 Ok((self.f)(
//                     $($arg::from_value(arg_iter.next().unwrap())?,)*
//                 )?.to_value())
//             }
//         }
//     };
// }

// // Helper macro to count arguments
// macro_rules! count {
//     () => (0);
//     ($x:tt $($xs:tt)*) => (1 + count!($($xs)*));
// }

// // Implement for different arities
// // impl_callable!();
// // impl_callable!(A);
// // impl_callable!(A, B);
// // impl_callable!(A, B, C);
// // Add more as needed...

// impl NativeFunction {
//     // Constructor for variadic functions
//     pub fn new_variadic<F>(f: F) -> Self
//     where
//         F: Fn(&[Value]) -> Result<Value, RuntimeError> + 'static,
//     {
//         NativeFunction { func: Box::new(f) }
//     }

//     // Constructor for fixed-arity functions
//     pub fn new<F, Args, Ret>(f: F) -> Self
//     where
//         F: Fn(Args) -> Result<Ret, RuntimeError> + 'static,
//         FixedFunction<F, Args, Ret>: Callable,
//     {
//         NativeFunction {
//             func: Box::new(FixedFunction {
//                 f,
//                 _phantom: PhantomData,
//             }),
//         }
//     }

//     pub fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
//         self.func.call(args)
//     }
// }

// pub struct NativeFun<F>(pub F);

// pub trait NativeFunction {
//     fn call(&self, args: &[Value]) -> Result<Value, RuntimeError>;
// }

// macro_rules! impl_native_function {
//     // For functions that take &[Value] directly
//     () => {
//         impl<F> NativeFunction for NativeFun<F>
//         where
//             F: Fn(&[Value]) -> Result<Value, RuntimeError>,
//         {
//             fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
//                 (self.0)(args)
//             }
//         }
//     };

//     // For functions with specific arguments
//     ($($arg:ident),*) => {
//         paste::paste! {
//             impl<F, R, $([<T_ $arg>]),*> NativeFunction for NativeFun<F>
//             where
//                 F: Fn($([<T_ $arg>]),*) -> Result<R, RuntimeError>,
//                 R: IntoValue,
//                 $([<T_ $arg>]: FromValue),*
//             {
//                 fn call(&self, args: &[Value]) -> Result<Value, RuntimeError> {
//                     let expected_args = count_tts!($($arg)*);
//                     if args.len() != expected_args {
//                         return Err(RuntimeError {
//                             message: format!("Expected {} arguments, got {}", expected_args, args.len()),
//                             span: None,
//                         });
//                     }

//                     let mut arg_iter = args.iter();
//                     $(
//                         let $arg = [<T_ $arg>]::from_value(arg_iter.next().unwrap().clone())
//                             .ok_or_else(|| RuntimeError {
//                                 message: format!("Failed to convert argument"),
//                                 span: None,
//                             })?;
//                     )*

//                     let result = (self.0)($($arg),*)?;
//                     result.into_value().ok_or_else(|| RuntimeError {
//                         message: "Failed to convert return value".to_string(),
//                         span: None,
//                     })
//                 }
//             }
//         }
//     };
// }

// // Helper macro to count the number of tokens
// macro_rules! count_tts {
//     () => (0usize);
//     ($head:tt $($tail:tt)*) => (1usize + count_tts!($($tail)*));
// }

// impl_native_function!();
// impl_native_function!(a);
// // impl_native_function!(a: i64, b: i64);
