use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub enum WebQuery {
    Login,
    Signup,
    UpdateBal,
}

#[derive(Deserialize, Serialize)]
pub struct WebInput {
    pub function: WebQuery,
    pub email: String,
    pub apikey: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize)]
pub struct WebOutput {
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct Output {
    pub code: u32,
    pub output: Value,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub max_tokens: u128,
}

// Users:

#[allow(unused)]
pub enum TableFields {
    Email,
    Password,
    Apikey,
    Balance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub apikey: String,
    pub balance: i64,
}
