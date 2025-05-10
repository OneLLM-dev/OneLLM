#[allow(unused)]
use axum::{
    Json, Router,
    extract::Query,
    http::header::HeaderMap,
    response::Html,
    routing::{get, post},
};
// use std::net::SocketAddr;

use tower_http::services::ServeDir;

use serde_json::{Value, json};

use crate::auth;
use crate::requests::Input;
use crate::utils::*;

#[allow(unused)]
pub async fn server() {
    let app = Router::new()
        .fallback_service(ServeDir::new("frontend"))
        .route("/api", get(handle_api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn handle_api(headers: HeaderMap, Json(payload): Json<Input>) -> Json<Output> {
    let apikey = if let Some(auth_header_value) = headers.get("Authorization") {
        match auth_header_value.to_str() {
            Ok(header_str) => {
                if let Some(token) = header_str.strip_prefix("Bearer ") {
                    token.to_string()
                } else {
                    return Json(Output {
                        code: 401,
                        output: json!({
                            "error": "Invalid authorization scheme. Expected Bearer token.",
                        }),
                    });
                }
            }
            Err(e) => {
                return Json(Output {
                    code: 401,
                    output: json!({
                        "error": format!("Invalid header value: {}", e),
                    }),
                });
            }
        }
    } else {
        return Json(Output {
            code: 401,
            output: json!({
                "error": "No Authorization header provided.",
            }),
        });
    };
    let user = match User::get_row(apikey.clone()).await {
        Ok(user_struct) => user_struct,
        Err(e) => {
            return Json(Output {
                code: 401,
                output: json!({
                    "error": e.to_string()
                }),
            });
        }
    };

    if user.apikey != apikey {
        return unauthorised_apikey();
    }

    let _: Data = match serde_json::from_value::<Value>(payload.data.clone()) {
        Ok(_) => return unauthorised_field_provided(),
        Err(_) => Data {
            max_tokens: user.balance as u128,
        },
    };

    let output = match payload.get().await {
        Ok(result) => result,
        Err(_) => {
            return Json(Output {
                code: 500,
                output: json!({
                "output": "Error during payload processing"
                }),
            });
        }
    };

    // Return the successful response
    Json(Output {
        code: 200,
        output: json!(output),
    })
}

pub async fn _handle_website(Json(query): Json<WebInput>) -> Json<WebOutput> {
    match query.function {
        WebQuery::Login => {}
        WebQuery::Signup => {
            let user = auth::signup(
                query.data["email"].to_string(),
                query.data["password"].to_string(),
            )
            .unwrap();
            return Json(WebOutput { user });
        }
        _ => {}
    }

    Json(WebOutput {
        user: User {
            email: String::new(),
            password: String::new(),
            apikey: String::new(),
            balance: 0,
        },
    })
}

/*------Unauthorised String returning functions------*/

fn unauthorised_apikey() -> Json<Output> {
    let response = Output {
        code: 401,
        output: json!("Unauthorized"),
    };
    return Json(response);
}

fn unauthorised_field_provided() -> Json<Output> {
    let response = Output {
        code: 403,
        output: json!("error: Unauthorised field provided."),
    };
    return Json(response);
}
