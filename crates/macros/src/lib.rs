use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FnArg, ItemFn, Pat, ReturnType, Type, TypeSlice};

use syn::{parse::Parse, parse::ParseStream, punctuated::Punctuated, Lit, Meta, Token};

struct TypeCheckAttr {
    index: usize,
    checker_type: Type,
}

struct TypeCheckSpec {
    attrs: Vec<TypeCheckAttr>,
}

impl Parse for TypeCheckSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

        let attrs = parsed
            .into_iter()
            .filter_map(|meta| {
                if let Meta::List(list) = meta {
                    if list.path.is_ident("contract") {
                        let content = list.parse_args_with(|content: ParseStream| {
                            let index: Lit = content.parse()?;
                            content.parse::<Token![,]>()?;
                            let checker_type: Type = content.parse()?;
                            Ok((index, checker_type))
                        });

                        if let Ok((Lit::Int(index), checker_type)) = content {
                            if let Ok(index) = index.base10_parse::<usize>() {
                                return Some(TypeCheckAttr {
                                    index,
                                    checker_type,
                                });
                            }
                        }
                    }
                }
                None
            })
            .collect();

        Ok(TypeCheckSpec { attrs })
    }
}

#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_spec = parse_macro_input!(attr as TypeCheckSpec);
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

                if slice_type.is_some() {
                    panic!("Slice argument must appear last");
                }

                if let Type::Reference(type_ref) = &*pat_type.ty {
                    if let Type::Slice(TypeSlice { elem, .. }) = &*type_ref.elem {
                        slice_type = Some(elem.as_ref());
                    }
                }
            }
        }
    }

    let type_checks = type_spec.attrs.iter().map(|attr| {
        let index = attr.index;
        let checker_type = &attr.checker_type;

        quote! {
            if !#checker_type.check(&args[#index]) {
                return Err(RuntimeError::new(
                    format!("argument {} must be a {}", #index + 1, #checker_type.name()),
                    None,
                ));
            }
        }
    });

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

    let conversion = if let Some(slice_type) = slice_type {
        let non_slice_count = arg_count - 1;
        let non_slice_indices = 0..non_slice_count;
        let non_slice_names = &arg_names[..non_slice_count];
        let non_slice_types = &arg_types[..non_slice_count];
        let slice_name = &arg_names[non_slice_count];

        quote! {
            #(#type_checks)*

            if args.len() < #non_slice_count {
                return Err(RuntimeError {
                    message: format!("Expected at least {} arguments, got {}", #non_slice_count, args.len()),
                    span: None,
                });
            }

            #(
                let #non_slice_names = if let Some(value) = <#non_slice_types as crate::ast::FromValue>::from_value(&args[#non_slice_indices]) {
                    value
                } else {
                    return Err(RuntimeError {
                        message: format!("Failed to convert argument {} to {}", #non_slice_indices, stringify!(#non_slice_types)),
                        span: None,
                    });
                };
            )*

            let __slice: Vec<_> = args[#non_slice_count..]
                .iter()
                .map(|value| {
                    if let Some(converted) = <#slice_type as crate::ast::FromValue>::from_value(value) {
                        Ok(converted)
                    } else {
                        Err(RuntimeError {
                            message: format!("Failed to convert slice item to {}", stringify!(#slice_type)),
                            span: None,
                        })
                    }
                })
                .collect::<Result<Vec<_>, RuntimeError>>()?;
            let #slice_name = &__slice;
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

    let inner_fn_name = format_ident!("{}_inner", fn_name);
    let result = quote! {
        let result = #inner_fn_name(#(#arg_names),*);
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

#[proc_macro_derive(ScamperStruct, attributes(default, contract))]
pub fn derive_struct_conversion(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();

    let fields = if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("ScamperStruct can only be derived for structs with named fields");
        }
    } else {
        panic!("ScamperStruct can only be derived for structs");
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_name_strings: Vec<_> = field_names.iter().map(|f| f.to_string()).collect();

    let mut default_values: Vec<Option<syn::Expr>> = Vec::new();
    let mut type_annotations: Vec<Option<syn::Expr>> = Vec::new();

    for field in fields {
        let default = field.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("default") {
                attr.parse_args::<syn::Expr>().ok()
            } else {
                None
            }
        });
        default_values.push(default);

        let ty = field.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("contract") {
                attr.parse_args::<syn::Expr>().ok()
            } else {
                None
            }
        });
        type_annotations.push(ty);
    }

    // validate default value positions
    let mut saw_non_default = false;
    for (i, default) in default_values.iter().rev().enumerate() {
        if default.is_none() {
            saw_non_default = true;
        } else if saw_non_default {
            let field_name = &field_names[field_names.len() - i - 1];
            panic!(
                "Field `{}` has a default value, but is followed by fields without default values",
                field_name
            );
        }
    }

    let default_values_expr = if default_values.iter().any(|v| v.is_some()) {
        let values = default_values
            .iter()
            .filter_map(|v| v.as_ref())
            .collect::<Vec<_>>();
        quote! {
            Some(vec![
                #(crate::ast::IntoValue::into_value(#values).unwrap()),*
            ])
        }
    } else {
        quote! { None }
    };

    let type_checker_tokens: Vec<proc_macro2::TokenStream> = type_annotations
        .iter()
        .map(|opt_expr| match opt_expr {
            Some(expr) => quote! { Some(Box::new(#expr) as Box<dyn Contract>) },
            None => quote! { None },
        })
        .collect();

    let type_annotations_expr = {
        quote! {
            Some(vec![
                #(#type_checker_tokens),*
            ])
        }
    };

    let indices = 0..field_names.len();

    let expanded = quote! {
        impl #name {
            pub fn add_to(env: &mut crate::interpreter::Env) {
                let s = crate::ast::Struct {
                       kind: #name_str.to_string(),
                       fields: vec![
                        #(#field_name_strings.to_string()),*
                    ],
                    values: vec![],
                };
                s.add_to(env, #type_annotations_expr, #default_values_expr);
            }
        }

        impl crate::ast::IntoValue for #name {
            fn into_value(self) -> Option<Value> {
                Some(Value::Struct(Struct {
                    kind: #name_str.to_string(),
                    fields: vec![
                        #(#field_name_strings.to_string()),*
                    ],
                    values: vec![
                        #(crate::ast::IntoValue::into_value(self.#field_names)?),*
                    ],
                }))
            }
        }

        impl crate::ast::FromValue for #name {
            fn from_value(value: &Value) -> Option<Self> {
                match value {
                    Value::Struct(s) => {
                        if s.kind == #name_str {
                            Some(#name {
                                #(#field_names: crate::ast::FromValue::from_value(&s.values[#indices])?),*
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
