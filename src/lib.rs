use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse_macro_input,
        ItemFn,
    },
};

#[proc_macro_attribute]
pub fn only_user(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the required role from the macro's attribute
    let role = attr.to_string().replace("\"", ""); // Parse "Admin" or "User" as string

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

            // Extract the user from the request extensions
            let user = match Extension::<User>::from_request(&req).await {
                Ok(user) => user,
                Err(_) => {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(GenericResponse {
                            status: StatusCode::FORBIDDEN.to_string(),
                            result: DataResponse::<String> {
                                msg: "Access denied: No user information".into(),
                                data: None,
                            },
                        }),
                    ).into_response()
                }
            };

            // Check the user's role
            if user.role != #role {
                return (
                    StatusCode::FORBIDDEN,
                    Json(GenericResponse {
                        status: StatusCode::FORBIDDEN.to_string(),
                        result: DataResponse::<String> {
                            msg: format!("Access denied: {} role required", #role),
                            data: None,
                        },
                    }),
                ).into_response();
            }

            // Continue executing the original handler
            async move { #block }.await
        }
    };

    TokenStream::from(expanded)
}
