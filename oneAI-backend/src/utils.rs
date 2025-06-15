use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub enum WebQuery {
    Login,
    Signup,
    UpdateBal(i64),
}

#[derive(Deserialize, Serialize)]
pub struct WebInput {
    pub function: WebQuery,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub enum ApiCommand {
    NewAPI,
    DelAPI,
    DelAllAPI,
}

#[derive(Deserialize, Serialize)]
pub struct WebOutput {
    pub user: Option<HiddenUser>,
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
    Balance,
}

#[derive(PartialEq,Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub balance: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HiddenUser {
    pub email: String,
    pub balance: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FailOrSucc {
    Failure(String),
    Success,
}
