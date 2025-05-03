#[allow(unused)]
use axum::{
    Router,
    extract::{Json, Query},
    routing::{get, post},
};

#[allow(unused)]
use serde::{Deserialize, Serialize};

use crate::database::User;
use crate::requests::Input;

#[allow(unused)]
async fn server() {
    let app = Router::new().route("/api", get(handle));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[allow(unused)]

async fn handle(Query(apikey): Query<String>, Json(payload): Json<Input>) -> String {
    let user = match User::get_row(apikey).await {
        Ok(user_struct) => user_struct,
        Err(e) => return e.to_string(),
    };

    String::new()
}
