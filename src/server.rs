#[allow(unused)]
use axum::{
    Json, Router,
    extract::Query,
    response::Html,
    routing::{get, post},
};

use tower_http::services::ServeDir;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::database::User;
use crate::requests::Input;

#[derive(Deserialize, Serialize)]
struct Output {
    code: u32,
    data: Value,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub max_tokens: u128,
}

#[allow(unused)]
pub async fn server() {
    let app = Router::new()
        .nest_service("/something/", ServeDir::new("frontend"))
        .route("/api", get(handle_api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn handle_api(Query(apikey): Query<String>, Json(payload): Json<Input>) -> String {
    let user = match User::get_row(apikey.clone()).await {
        Ok(user_struct) => user_struct,
        Err(e) => return e.to_string(),
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
            return json!({
                "code": 500,
                "output": "Error during payload processing"
            })
            .to_string();
        }
    };

    // Return the successful response
    json!({
        "code": 200,
        "output": output
    })
    .to_string()
}
#[allow(unused)]
async fn handle_website() -> Html<String> {
    let file_contents = std::fs::read_to_string("frontend/index.html")
        .unwrap_or_else(|_| "<h1>500 Internal Server Error</h1>".to_string());

    Html(file_contents)
}

/*------Unauthorised String returning functions------*/

fn unauthorised_apikey() -> String {
    let response = json!({
        "code": 401,
        "output": "Unauthorized"
    });
    return response.to_string();
}

fn unauthorised_field_provided() -> String {
    let response = json!({
        "code": 403,
        "output": "You are not allowed to provide the field 'is_admin'"
    });
    return response.to_string();
}
