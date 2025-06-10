use std::error::Error;

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

use crate::auth::{self, login, update_bal};
use crate::requests::Input;
use crate::utils::*;

#[allow(unused)]
pub async fn server() {
    let app = Router::new()
        .fallback_service(ServeDir::new("../OneLLM-Website/"))
        .route("/api", get(handle_api))
        .route("/post-backend", get(handle_post_website));
    let ipaddr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(ipaddr).await.unwrap();

    println!("Listening at: {}", ipaddr);
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
    let user = match User::get_row_api(apikey.clone()).await {
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


    let _: Data = match serde_json::from_value::<Value>(payload.data.clone()) {
        Ok(_) => Data {
            max_tokens: user.balance as u128,
        },
        Err(_) => return unauthorised_field_provided(),
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

pub async fn signup_and_update_db(
    email: String,
    password: String,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let user = match auth::signup(email, password).await {
        Some(a) => a,
        None => return Ok(None),
    };

    match user.new_user().await {
        Ok(()) => {}
        Err(_) => return Ok(None),
    };

    Ok(None)
}

#[derive(Debug)]
struct AuthError(String);
impl Error for AuthError {}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Update error: {}", self.0)
    }
}

pub async fn handle_post_website(Json(query): Json<WebInput>) -> Json<FailOrSucc> {
    match query.function {
        WebQuery::Signup => {
            let user: Option<User> = signup_and_update_db(query.email, query.password)
                .await
                .unwrap();

            match user {
                Some(_) => return Json(FailOrSucc::Success),
                None => {
                    return Json(FailOrSucc::Failure(String::from(
                        "Error while trying to create your account",
                    )));
                }
            }

            //     auth::signup(
            //     query.data["email"].to_string(),
            //     query.data["password"].to_string(),
            // )
            // .await
            // .unwrap_or_else(|| return None)
            // .new_user()
            // .await
            // .expect("Error adding user")
        }
        WebQuery::Login => {
            return Json(FailOrSucc::Failure(String::from(
                "Error logging in. Login is not POST request",
            )));
        }
        WebQuery::UpdateBal(a) => match update_bal(query.email, query.password, a).await {
            Some(_) => return Json(FailOrSucc::Success),
            None => {
                return Json(FailOrSucc::Failure(String::from(
                    "Error while trying to update your account balance",
                )));
            }
        },
    }

    // Ok(())
}

pub async fn handle_get_website(Json(query): Json<WebInput>) -> Json<WebOutput> {
    match query.function {
        WebQuery::Signup => {}
        _ => {}
    }

    Json(WebOutput {
        user: Some(User {
            email: String::new(),
            password: String::new(),
            apikey: String::new(),
            balance: 0,
        }),
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
