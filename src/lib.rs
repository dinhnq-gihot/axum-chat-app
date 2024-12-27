use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse_macro_input,
        ItemFn,
    },
};

#[proc_macro_attribute]
pub fn only_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the required role from the macro's attribute
    let roles: Vec<String> = attr
        .to_string()
        .replace("\"", "") // Remove quotes
        .split(',') // Split roles by comma
        .map(|role| role.trim().to_string())
        .collect();

    // Parse the input handler function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function's signature and block
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input_fn;

    // Generate the expanded code
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use axum::{
                http::StatusCode,
                response::IntoResponse,
                Extension,
                Json,
            };
            use crate::features::users::models::User;
            use crate::enums::errors::Error;

            // Check if the user's role is allowed
            let allowed_roles = vec![#(#roles),*]; // Allowed roles from the macro input

            // Check the user's role
            if !allowed_roles.contains(&sender.role.as_str()) {
                return Err(Error::AccessDenied(sender.role));
            }

            // Continue executing the original handler
            async move { #block }.await
        }
    };

    TokenStream::from(expanded)
}
