use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;
use tower_http::cors::{Any, CorsLayer};

use axum::{Json, Router, http::header::HeaderMap, response::IntoResponse, routing::post};

use tower_http::services::ServeDir;

use serde_json::json;

use crate::{
    auth::{
        basicauth::{self},
        twofa::{self, send_verify},
    },
    requests::parseapi::APIInput,
};
use crate::{payment, utils::*};

#[axum::debug_handler]

pub async fn server() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (good for development)
        .allow_methods(Any) // Allow all HTTP methods (GET, POST, etc.)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(ServeDir::new("OneLLM-Website/"))
        .route("/api", post(handle_api))
        .route("/post-backend", post(handle_post_website))
        .route("/verify-email", post(verify_email))
        .route("/check-verify", post(verify_code))
        .route("/apikey-commands", post(handle_token_auth))
        .route("/token-login", post(login_with_token))
        .route("/webhook", post(payment::handle_webhook))
        .layer(cors);
    let ipaddr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(ipaddr).await.unwrap();

    println!("Listening at: {}", ipaddr);
    axum::serve(listener, app).await.unwrap();
}

pub async fn verify_email(Json(payload): Json<VerifyInput>) -> Json<FailOrSucc> {
    let mut redis = redis::Client::open(
        std::env::var("REDIS")
            .expect("Redis ENV VAR not found")
            .as_str(),
    )
    .expect("Unable to connect to redis")
    .get_multiplexed_async_connection()
    .await
    .expect("Unable to get MultiplexedAsyncConnection");

    match send_verify(&mut redis, &payload.email).await {
        Ok(()) => {
            return Json(FailOrSucc::Successful("Successful".to_string()));
        }
        Err(e) => {
            return Json(FailOrSucc::Failure(e.to_string()));
        }
    };
}

pub async fn verify_code(Json(payload): Json<VerifyInput>) -> Json<FailOrSucc> {
    let mut redis = redis::Client::open(
        std::env::var("REDIS")
            .expect("Redis ENV VAR not found")
            .as_str(),
    )
    .expect("Unable to connect to redis")
    .get_multiplexed_async_connection()
    .await
    .expect("Unable to get MultiplexedAsyncConnection");

    let did_verify = match twofa::verify_code(
        &mut redis,
        &payload.email,
        &payload.code.unwrap_or("".to_string()),
    )
    .await
    {
        Ok(a) => a,
        Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
    };

    if did_verify {
        match User::verify_user(&payload.email).await {
            Ok(()) => return Json(FailOrSucc::Successful("Successful".to_string())),

            Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        }
    } else {
        Json(FailOrSucc::Failure("Verification unsuccessful".to_string()))
    }
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

    let output = match payload.get(apikey).await {
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

pub async fn handle_post_website(Json(query): Json<WebInput>) -> impl IntoResponse {
    match query.function {
        WebQuery::Signup => {
            let user: Option<User> = signup_and_update_db(query.email, query.password)
                .await
                .unwrap();

            match user {
                Some(_) => {
                    return Json(FailOrSucc::Successful(String::from("Successful operation")));
                }
                None => {
                    return Json(FailOrSucc::Failure(String::from(
                        "Error while trying to create your account",
                    )));
                }
            }
        }

        //        WebQuery::ChangePwd => {
        //            match User::change_password(&query.token.unwrap_or("".to_string()), query.password)
        //                .await
        //            {
        //                Ok(_) => {
        //                    return Json(FailOrSucc::Successful(
        //                        "Successfully changed password".to_string(),
        //                    ));
        //                }
        //                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        //            }
        //        }
        WebQuery::Login => {
            let mut user = match basicauth::login(query.email, query.password).await {
                Some(u) => u,
                None => {
                    return Json(FailOrSucc::Failure("Could not log user in".to_string()));
                }
            };

            user.balance /= 1_000_000;

            let token = match User::new_token(user.id).await {
                Ok(tok) => tok,
                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
            };

            let hidden_user = WebOutput {
                user: HiddenUser::from_user(&mut user).await,
                token,
            };

            return Json(FailOrSucc::User(hidden_user));
        }
        _ => {
            return Json(FailOrSucc::Failure(
                "Tried to do Handle API at POST section".to_owned(),
            ));
        }
    }
}

async fn login_with_token(Json(query): Json<TokenInput>) -> Json<FailOrSucc> {
    let mut hidden_user = match User::from_token(query.token.clone()).await {
        Ok(huser) => huser,
        Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
    };
    hidden_user.balance /= 1_000_000;
    return Json(FailOrSucc::User(WebOutput {
        user: hidden_user,
        token: query.token,
    }));
}

pub async fn handle_token_auth(Json(payload): Json<TokenInput>) -> impl IntoResponse {
    let res = match User::from_token(payload.token.clone()).await {
        Ok(u) => u,
        Err(e) => {
            return Json(FailOrSucc::Failure(e.to_string()));
        }
    };

    let user = match User::get_row(res.email).await {
        Ok(a) => a,
        Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
    };

    if !user
        .is_verified()
        .await
        .expect("Error trying to see if user was verified")
    {
        return Json(FailOrSucc::Failure("User isn't verified".to_string()));
    }

    match payload.function {
        WebQuery::NewAPI => match user
            .generate_apikey(&payload.name.unwrap_or("".to_owned()))
            .await
        {
            Ok(api) => return Json(FailOrSucc::SuccessData(api)),
            Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        },

        WebQuery::DelAPI => {
            match User::delete_apikey(
                &payload.token,
                Some(&payload.name.unwrap_or("".to_string())),
                false,
            )
            .await
            {
                Ok(()) => return Json(FailOrSucc::Successful("Successful operation".to_string())),
                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
            }
        }

        WebQuery::APICount => {
            match User::get_keynames(&payload.email.unwrap_or("".to_string())).await {
                Ok(keynamevec) => return Json(FailOrSucc::SuccessVecData(keynamevec)),
                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
            }
        }

        //        WebQuery::DelAllAPI => {
        //            match User::delete_apikey(&user.email, &payload.password, "", true).await {
        //                Ok(()) => return Json(FailOrSucc::Successful("Successful operation".to_string())),
        //                Err(e) => return Json(FailOrSucc::Failure(e.to_string())),
        //            }
        //        }
        _ => return Json(FailOrSucc::Failure(String::from("Incorrect endpoint"))),
    }
}
