use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, GenericArgument, ItemFn, LitStr, PathArguments, Type};

fn extractor_inner_type(ty: &Type) -> Option<(String, &Type)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let name = segment.ident.to_string();
            if matches!(name.as_str(), "Json" | "ValidatedJson" | "Path" | "Query") {
                if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    if let Some(GenericArgument::Type(inner)) = args.args.first() {
                        return Some((name, inner));
                    }
                }
            }
        }
    }
    None
}

fn route_macro(args: TokenStream, input: TokenStream, method: &str) -> TokenStream {
    let path = parse_macro_input!(args as LitStr);
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;
    let route_fn_name = syn::Ident::new(&format!("{}_route", fn_name), fn_name.span());
    let method_ident = syn::Ident::new(method, fn_name.span());
    let vis = &func.vis;
    let attrs = &func.attrs;
    let sig = &func.sig;
    let block = &func.block;

    // Infer state type S from handler arguments
    let state_ty = func
        .sig
        .inputs
        .iter()
        .find_map(|input| {
            if let syn::FnArg::Typed(pat_type) = input {
                if let Type::Path(type_path) = &*pat_type.ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == "State" {
                            if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                    return Some(quote! { #inner_ty });
                                }
                            }
                        }
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| quote! { () });

    // Detect request schema from first Json parameter
    let req_schema = func.sig.inputs.iter().find_map(|input| {
        if let syn::FnArg::Typed(pat_type) = input {
            if let Some((name, inner)) = extractor_inner_type(&pat_type.ty) {
                if name == "Json" || name == "ValidatedJson" {
                    return Some(
                        quote! { .with_request_schema(::rustapi::schema_for::<#inner>()) },
                    );
                }
            }
        }
        None
    });

    let query_schema = func.sig.inputs.iter().find_map(|input| {
        if let syn::FnArg::Typed(pat_type) = input {
            if let Some((name, inner)) = extractor_inner_type(&pat_type.ty) {
                if name == "Query" {
                    return Some(quote! { .with_query_schema(::rustapi::schema_for::<#inner>()) });
                }
            }
        }
        None
    });

    // Detect response schema from return type
    let res_schema = match &func.sig.output {
        syn::ReturnType::Type(_, ret_ty) => {
            if let Some((name, inner)) = extractor_inner_type(ret_ty.as_ref()) {
                if name == "Json" {
                    Some(quote! { .with_response_schema(::rustapi::schema_for::<#inner>()) })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #sig #block


        #[doc(hidden)]
        pub fn #route_fn_name() -> ::rustapi::Route<#state_ty> {
            ::rustapi::Route::#method_ident(#path, #fn_name)
                #req_schema
                #res_schema
                #query_schema
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn model(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = input.into();
    let expanded = quote! {
        #[derive(::rustapi::Serialize, ::rustapi::Deserialize, ::rustapi::JsonSchema, ::validator::Validate, Clone, ::rustapi::ts_rs::TS)]
        #[ts(export, crate = "::rustapi::ts_rs")]
        #item
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "get")
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "post")
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "put")
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "delete")
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "patch")
}
