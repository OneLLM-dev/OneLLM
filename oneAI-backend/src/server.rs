use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    Json, Router,
    extract::Query,
    http::header::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};

use tower_http::services::ServeDir;

use serde_json::json;

use crate::{
    auth::basicauth::{self},
    requests::parseapi::APIInput,
};
use crate::{payment, utils::*};

#[allow(unused)]
pub async fn server() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (good for development)
        .allow_methods(Any) // Allow all HTTP methods (GET, POST, etc.)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(ServeDir::new("../OneLLM-Website/"))
        .route("/api", post(handle_api))
        .route("/post-backend", post(handle_post_website))
        .route("/get-backend", get(handle_get_website))
        .route("/apikey-commands", get(handle_api_auth))
        .route("/webhook", post(payment::handle_webhook))
        .layer(cors);
    let ipaddr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(ipaddr).await.unwrap();

    println!("Listening at: {}", ipaddr);
    axum::serve(listener, app).await.unwrap();
}

const MAX_TOKENS: f64 = 40.0;
const REFILL_RATE: f64 = MAX_TOKENS / 60.0; // tokens per second

async fn allow_request(
    conn: &mut redis::aio::MultiplexedConnection,
    api_key: &str,
) -> redis::RedisResult<bool> {
    let tokens_key = format!("tokens:{}", api_key);
    let timestamp_key = format!("timestamp:{}", api_key);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let tokens: Option<f64> = conn.get(&tokens_key).await.ok();
    let last_refill: Option<f64> = conn.get(&timestamp_key).await.ok();

    let tokens = tokens.unwrap_or(MAX_TOKENS);
    let last_refill = last_refill.unwrap_or(now);

    let elapsed = now - last_refill;
    let new_tokens = f64::min(MAX_TOKENS, tokens + elapsed * REFILL_RATE);

    let allowed = if new_tokens >= 1.0 {
        let updated_tokens = new_tokens - 1.0;
        let _: () = conn.set_ex(&tokens_key, updated_tokens, 120).await?;
        let _: () = conn.set_ex(&timestamp_key, now, 120).await?;
        true
    } else {
        false
    };

    Ok(allowed)
}
pub async fn handle_api(headers: HeaderMap, Json(payload): Json<APIInput>) -> Json<Output> {
    dotenv::dotenv().ok();
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

    let redis = redis::Client::open(
        std::env::var("REDIS")
            .expect("Redis ENV VAR not found")
            .as_str(),
    )
    .expect("Unable to connect to redis");

    if !allow_request(
        &mut redis
            .get_multiplexed_async_connection()
            .await
            .expect("Error while trying to get MultiplexedConnection"),
        &apikey,
    )
    .await
    .expect("Error while checking if request should be allowed")
    {
        return Json(Output {
            code: 429,
            output: json!({
                "error": "Rate limit exceeded."
            }),
        });
    }

    match User::get_row_api(apikey.clone()).await {
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

async fn signup_and_update_db(
    email: String,
    password: String,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let user = match basicauth::signup(email, password).await {
        Some(a) => a,
        None => return Ok(None),
    };

    match user.new_user().await {
        Ok(()) => {}
        Err(_) => return Ok(None),
    };

    Ok(Some(user))
}

// #[derive(Debug)]
// struct AuthError(String);
// impl Error for AuthError {}

// impl std::fmt::Display for AuthError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Update error: {}", self.0)
//     }
// }

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
        _ => {
            return Json(FailOrSucc::Failure(
                "Tried to do Handle API at POST section".to_owned(),
            ));
        }
    }

    // Ok(())
}

pub async fn handle_api_auth(Query(query): Query<WebInput>) -> impl IntoResponse {
    let user = match basicauth::login(query.email.clone(), query.password.clone()).await {
        Some(u) => u,
        None => {
            return Json(FailOrSucc::Failure(
                "Error while trying to sign in".to_owned(),
            ));
        }
    };

    match query.function {
        WebQuery::NewAPI => match user
            .generate_apikey(&query.name.unwrap_or("".to_owned()))
            .await
        {
            Ok(api) => return Json(FailOrSucc::SuccessData(api)),
            Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        },

        WebQuery::DelAPI => {
            println!("Query:\n{query:#?}");
            match User::delete_apikey(&query.email, &query.name.unwrap_or("".to_string()), false)
                .await
            {
                Ok(()) => return Json(FailOrSucc::Success),
                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
            }
        }

        WebQuery::APICount => match User::get_keynames(&query.email).await {
            Ok(keynamevec) => return Json(FailOrSucc::SuccessVecData(keynamevec)),
            Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        },

        WebQuery::DelAllAPI => match User::delete_apikey(&user.email, "", true).await {
            Ok(()) => return Json(FailOrSucc::Success),
            Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        },

        _ => return Json(FailOrSucc::Failure(String::from("Incorrect endpoint"))),
    }
}

pub async fn handle_get_website(Query(query): Query<WebInput>) -> Json<WebOutput> {
    match query.function {
        WebQuery::Login => {
            let mut user = match basicauth::login(query.email, query.password).await {
                Some(u) => u,
                None => {
                    return Json(WebOutput { user: None });
                }
            };
            return Json(WebOutput {
                user: Some(HiddenUser::from_user(&mut user).await),
            });
        }
        _ => Json(WebOutput { user: None }),
    }
}

/*------Unauthorised String returning functions------*/

// fn unauthorised_apikey() -> Json<Output> {
//     let response = Output {
//         code: 401,
//         output: json!("Unauthorized"),
//     };
//     return Json(response);
// }

//fn unauthorised_field_provided() -> Json<Output> {
//    let response = Output {
//        code: 403,
//        output: json!("error: Unauthorised field provided."),
//    };
//    return Json(response);
//}
