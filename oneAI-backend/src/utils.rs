use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyInput {
    pub email: String,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32, // user_id
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VerificationOption {
    Token,
    Email,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WebQuery {
    Login,
    Signup,
    UpdateBal(i64),
    NewAPI,
    DelAPI,
    DelAllAPI,
    APICount,
    ChangePwd,
    VerifyToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenInput {
    pub function: WebQuery,
    pub token: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebInput {
    pub function: WebQuery,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WebOutput {
    pub user: HiddenUser,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Output {
    pub code: u32,
    pub output: Value,
}

#[allow(unused)]
pub enum TableFields {
    Email,
    Password,
    Balance,
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub balance: u32,
    pub verified: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HiddenUser {
    pub email: String,
    pub balance: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FailOrSucc {
    Failure(String),
    Successful(String),
    SuccessData(String),
    SuccessVecData(Vec<String>),
    User(WebOutput),
}
