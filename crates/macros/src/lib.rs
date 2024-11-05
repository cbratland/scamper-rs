use proc_macro::TokenStream;
use quote::{format_ident, quote};
use scamper_doc::ScamperDoc;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, DeriveInput, Expr,
    FnArg, Ident, ItemFn, Lit, Meta, Pat, ReturnType, Token, Type, TypeSlice,
};

#[derive(Debug)]
struct ContractAttr {
    index: usize,
    checker_type: Expr,
}

struct ContractSpec {
    attrs: Vec<ContractAttr>,
}

impl Parse for ContractSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

        let attrs: Vec<_> = parsed
            .into_iter()
            .filter_map(|meta| {
                if let Meta::List(list) = meta {
                    if list.path.is_ident("contract") {
                        let content = list.parse_args_with(|content: ParseStream| {
                            let index: Lit = content.parse()?;
                            content.parse::<Token![,]>()?;
                            let checker_type: Expr = content.parse()?;
                            Ok((index, checker_type))
                        });

                        if let Ok((Lit::Int(index), checker_type)) = content {
                            if let Ok(index) = index.base10_parse::<usize>() {
                                return Some(ContractAttr {
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

        Ok(ContractSpec { attrs })
    }
}

#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_spec = parse_macro_input!(attr as ContractSpec);
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
                return Err(crate::interpreter::RuntimeError::new(
                    format!("expected a {} in argument {}, received {}", #checker_type.name(), #index + 1, args[#index].name()),
                    None,
                ));
            }
        }
    });

    let arg_count = arg_names.len();

    // check if the return type is a Result
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
                return Err(crate::interpreter::RuntimeError::new(
                     format!("wrong number of arguments provided: expected at least {}, received {}", #non_slice_count, args.len()),
                     None,
                ));
            }

            #(
                let #non_slice_names = if let Some(value) = <#non_slice_types as crate::ast::FromValue>::from_value(
                    &args[#non_slice_indices]
                ) {
                    value
                } else {
                    return Err(crate::interpreter::RuntimeError::new(
                        format!(
                            "expected a {}, received {}",
                            <#slice_type as crate::ast::FromValue>::name(),
                            args[#non_slice_indices].name()
                        ),
                        None,
                    ));
                };
            )*

            let __slice: Vec<_> = args[#non_slice_count..]
                .iter()
                .map(|value| {
                    if let Some(converted) = <#slice_type as crate::ast::FromValue>::from_value(value) {
                        Ok(converted)
                    } else {
                        Err(crate::interpreter::RuntimeError::new(
                            format!(
                                "expected a {}, received {}",
                                <#slice_type as crate::ast::FromValue>::name(),
                                value.name()
                            ),
                            None,
                        ))
                    }
                })
                .collect::<Result<Vec<_>, crate::interpreter::RuntimeError>>()?;
            let #slice_name = &__slice;
        }
    } else {
        let arg_conversions = arg_types.iter().enumerate().map(|(i, ty)| {
	        let name = &arg_names[i];
	        if let Some(inner_ty) = extract_option_inner_type(ty) {
	            quote! {
	                let #name = if #i < args.len() {
	                    if let Some(value) = <#inner_ty as crate::ast::FromValue>::from_value(&args[#i]) {
	                        Some(value)
	                    } else {
	                        return Err(crate::interpreter::RuntimeError::new(
	                            format!(
	                                "expected a {}, received {}",
	                                <#inner_ty as crate::ast::FromValue>::name(),
	                                args[#i].name()
	                            ),
	                            None,
	                        ));
	                    }
	                } else {
	                    None
	                };
	            }
	        } else {
	            quote! {
	                let #name = if let Some(value) = <#ty as crate::ast::FromValue>::from_value(&args[#i]) {
	                    value
	                } else {
	                    return Err(crate::interpreter::RuntimeError::new(
	                        format!(
	                            "expected a {}, received {}",
	                            <#ty as crate::ast::FromValue>::name(),
	                            args[#i].name()
	                        ),
	                        None,
	                    ));
	                };
	            }
	        }
	    });

        quote! {
            #(#type_checks)*

            let required_args = {
                let mut count = 0;
                #(
                    if !matches!(stringify!(#arg_types), s if s.starts_with("Option <")) {
                        count += 1;
                    }
                )*
                count
            };

            if args.len() < required_args || args.len() > #arg_count {
                if required_args != #arg_count {
                    return Err(crate::interpreter::RuntimeError::new(
                        format!("wrong number of arguments provided: expected {} to {}, received {}", required_args, #arg_count, args.len()),
                        None,
                    ));
                } else {
                    return Err(crate::interpreter::RuntimeError::new(
                        format!("wrong number of arguments provided: expected {}, received {}", #arg_count, args.len()),
                        None,
                    ));
                }
            }

            #(#arg_conversions)*
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
                Err(crate::interpreter::RuntimeError::new(
                    "failed to parse return value".to_string(),
                    None,
                ))
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
        fn #fn_name(args: &[crate::ast::Value]) -> Result<crate::ast::Value, crate::interpreter::RuntimeError> {
            #conversion

            fn #inner_fn_name(#(#arg_names: #arg_types),*) #return_type #fn_block
            #result

            #handle_result
        }
    };

    TokenStream::from(expanded)
}

fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

#[proc_macro_derive(ForeignValue)]
pub fn derive_value_conversion(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();

    let expanded = quote! {
        impl crate::ast::IntoValue for #name {
            fn into_value(self) -> Option<crate::ast::Value> {
                Some(crate::ast::Value::Foreign(std::rc::Rc::new(self)))
            }
        }

        impl crate::ast::FromValue for #name {
            fn from_value(value: &crate::ast::Value) -> Option<Self> {
                match value {
                    crate::ast::Value::Foreign(f) => f.downcast_ref::<#name>().cloned(),
                    _ => None,
                }
            }

            fn name() -> &'static str {
                #name_str
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

            fn name() -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn scamper_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let doc = parse_macro_input!(attr as ScamperDoc);
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;

    // Generate documentation string
    let doc_string = generate_doc_string(&doc, fn_name, fn_args);

    // Add documentation as a doc comment
    let expanded = quote! {
        #[doc = #doc_string]
        #input_fn
    };

    TokenStream::from(expanded)
}

fn generate_doc_string(
    doc: &ScamperDoc,
    fn_name: &Ident,
    fn_args: &Punctuated<FnArg, Token![,]>,
) -> String {
    let mut result = String::new();

    let fn_name_str = fn_name.to_string();

    let name = doc
        .name
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or_else(|| fn_name_str.as_str());
    result.push_str(&format!("({}", name));

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                result.push_str(&format!(" {}", pat_ident.ident));
            }
        }
    }

    if let Some(ret_type) = &doc.return_type {
        result.push_str(&format!(") -> {}\n", ret_type));
    } else {
        result.push_str(")\n");
    }

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let param_name = pat_ident.ident.to_string();
                if let Some(param_doc) = doc.params.iter().find(|p| p.name == param_name) {
                    result.push_str(&format!(
                        "  {}: {}",
                        param_name,
                        param_doc.type_name.as_ref().unwrap_or(&"any".to_string())
                    ));
                    if let Some(desc) = &param_doc.description {
                        result.push_str(&format!(", {}", desc));
                    }
                    result.push('\n');
                }
            }
        }
    }

    // Description
    result.push_str(&doc.description);
    result.push('\n');

    result
}
