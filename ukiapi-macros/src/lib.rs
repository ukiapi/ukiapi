use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, GenericArgument, ItemFn, PathArguments, Type};

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

struct RouteArgs {
    path: syn::LitStr,
    registry: Option<syn::Path>,
}

impl syn::parse::Parse for RouteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse()?;
        let mut registry = None;
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            while !input.is_empty() {
                let ident: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                if ident == "registry" {
                    registry = Some(input.parse()?);
                } else {
                    let _: syn::Expr = input.parse()?;
                }
                if input.peek(syn::Token![,]) {
                    input.parse::<syn::Token![,]>()?;
                } else {
                    break;
                }
            }
        }
        Ok(RouteArgs { path, registry })
    }
}

fn route_macro(args: TokenStream, input: TokenStream, method: &str) -> TokenStream {
    let route_args = parse_macro_input!(args as RouteArgs);
    let path_lit = &route_args.path;
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
                    return Some(quote! { .with_request_schema(::ukiapi::schema_for::<#inner>()) });
                }
            }
        }
        None
    });

    let query_schema = func.sig.inputs.iter().find_map(|input| {
        if let syn::FnArg::Typed(pat_type) = input {
            if let Some((name, inner)) = extractor_inner_type(&pat_type.ty) {
                if name == "Query" {
                    return Some(quote! { .with_query_schema(::ukiapi::schema_for::<#inner>()) });
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
                    Some(quote! { .with_response_schema(::ukiapi::schema_for::<#inner>()) })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    };

    let registry_submit = if let Some(reg) = &route_args.registry {
        if state_ty.to_string() == "()" {
            quote! { ::ukiapi::submit_route!(#route_fn_name, #reg, stateless); }
        } else {
            quote! { ::ukiapi::submit_route!(#route_fn_name, #reg, stateful); }
        }
    } else if state_ty.to_string() == "()" {
        quote! { ::ukiapi::submit_route!(#route_fn_name); }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #sig #block


        #[doc(hidden)]
        pub fn #route_fn_name() -> ::ukiapi::Route<#state_ty> {
            ::ukiapi::Route::#method_ident(#path_lit, #fn_name)
                #req_schema
                #res_schema
                #query_schema
        }

        #registry_submit
    };

    expanded.into()
}

/// Macro to define a data model. Derives Serialize, Deserialize, JsonSchema, Validate, Clone, and TS.
#[proc_macro_attribute]
pub fn model(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = input.into();
    let expanded = quote! {
        #[derive(::ukiapi::Serialize, ::ukiapi::Deserialize, ::ukiapi::JsonSchema, ::validator::Validate, Clone, ::ukiapi::ts_rs::TS)]
        #[ts(export, crate = "::ukiapi::ts_rs")]
        #item
    };
    expanded.into()
}

/// Define a GET endpoint.
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "get")
}

/// Define a POST endpoint.
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "post")
}

/// Define a PUT endpoint.
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "put")
}

/// Define a DELETE endpoint.
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "delete")
}

/// Define a PATCH endpoint.
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "patch")
}

/// Define a WebSocket endpoint.
#[proc_macro_attribute]
pub fn websocket(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "websocket")
}

/// Macro to define the main entry point for a UkiApi application.
///
/// Sets up the tokio runtime, environment variables, and logger.
///
/// # Example
/// ```rust,ignore
/// use ukiapi::{get, routes};
///
/// #[get("/hello")]
/// async fn hello() -> &'static str {
///     "Hello from UkiApi!"
/// }
///
/// #[ukiapi::main]
/// async fn main() {
///     routes![(),
///         hello_route().with_state::<()>()
///     ]
///     .serve(())
///     .await;
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let block = &func.block;
    let sig = &func.sig;
    let attrs = &func.attrs;
    let vis = &func.vis;

    let expanded = quote! {
        #[tokio::main]
        #(#attrs)*
        #vis #sig {
            if std::env::var("UKIAPI_HOST").is_err() {
                std::env::set_var("UKIAPI_HOST", "127.0.0.1");
            }
            if std::env::var("UKIAPI_PORT").is_err() {
                std::env::set_var("UKIAPI_PORT", "3000");
            }
            env_logger::init();

            #block
        }
    };
    expanded.into()
}
