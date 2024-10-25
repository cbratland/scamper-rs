use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FnArg, ItemFn, Pat, ReturnType, Type, TypeSlice};

#[proc_macro_attribute]
pub fn function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let return_type = &input_fn.sig.output;

    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    let mut slice_type = None;

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                arg_names.push(pat_ident.ident.clone());
                arg_types.push(&*pat_type.ty);

                if let Type::Reference(type_ref) = &*pat_type.ty {
                    if let Type::Slice(TypeSlice { elem, .. }) = &*type_ref.elem {
                        slice_type = Some(elem.as_ref());
                    }
                }
            }
        }
    }

    let arg_count = arg_names.len();
    let indices = (0..arg_count).collect::<Vec<_>>();

    // Determine if the return type is a Result
    let is_result = if let ReturnType::Type(_, ty) = return_type {
        if let Type::Path(type_path) = &**ty {
            type_path
                .path
                .segments
                .first()
                .map(|seg| seg.ident == "Result")
                .unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    };

    let is_single_slice = slice_type.is_some() && arg_count == 1;

    let conversion = if is_single_slice {
        let arg_type = slice_type.unwrap();
        quote! {
            let converted: Vec<_> = args
                .iter()
                .map(|value| {
                    if let Some(converted) = <#arg_type as crate::ast::FromValue>::from_value(value) {
                        Ok(converted)
                    } else {
                        Err(RuntimeError {
                            message: format!("Failed to convert slice item to {}", stringify!(#arg_type)),
                            span: None,
                        })
                    }
                })
                .collect::<Result<Vec<_>, RuntimeError>>()?;
        }
    } else {
        quote! {
           if args.len() != #arg_count {
               return Err(RuntimeError {
                   message: format!("Expected {} arguments, got {}", #arg_count, args.len()),
                   span: None,
               });
           }

           #(
               let #arg_names = if let Some(value) = <#arg_types as crate::ast::FromValue>::from_value(&args[#indices]) {
                   value
               } else {
                   return Err(RuntimeError {
                       message: format!("Failed to convert argument {} to {}", #indices, stringify!(#arg_types)),
                       span: None,
                   });
               };
           )*
        }
    };

    let handle_result = {
        let handle_value = quote! {
            use crate::ast::IntoValue;
            if let Some(converted) = result.into_value() {
                Ok(converted)
            } else {
                Err(RuntimeError {
                    message: "Failed to convert return value".to_string(),
                    span: None,
                })
            }
        };
        if is_result {
            quote! {
                match result {
                    Ok(result) => {
                        #handle_value
                    }
                    Err(e) => Err(e),
                }
            }
        } else {
            handle_value
        }
    };

    let inner_fn_name = format_ident!("{}_inner", fn_name);
    let result = if is_single_slice {
        quote! {
            let result = #inner_fn_name(&converted);
        }
    } else {
        quote! {
            let result = #inner_fn_name(#(#arg_names),*);
        }
    };

    let expanded = quote! {
        fn #fn_name(args: &[Value]) -> Result<Value, RuntimeError> {
            #conversion

            fn #inner_fn_name(#(#arg_names: #arg_types),*) #return_type #fn_block
            #result

            #handle_result
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ForeignValue)]
pub fn derive_value_conversion(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::ast::IntoValue for #name {
            fn into_value(self) -> Option<Value> {
                Some(Value::Foreign(std::rc::Rc::new(self)))
            }
        }

        impl crate::ast::FromValue for #name {
            fn from_value(value: &Value) -> Option<Self> {
                match value {
                    Value::Foreign(f) => f.downcast_ref::<#name>().cloned(),
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
