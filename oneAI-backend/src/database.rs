// use sqlx::{Pool, Postgres, Row};
use std::error::Error;

use password_auth::verify_password;

use crate::auth::{self};
use crate::utils::*;

#[derive(Debug)]
struct MissingUser(String);
impl Error for MissingUser {}

impl std::fmt::Display for MissingUser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl User {
    pub async fn get_row_api(apikey: String) -> Result<User, Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE apikey = $1)")
                .bind(apikey.clone())
                .fetch_one(&pool)
                .await?;

        if !exists {
            return Err(Box::new(MissingUser("No such user was found".to_string())));
        }

        let email: String = sqlx::query_scalar("SELECT email FROM users WHERE apikey = $1")
            .bind(apikey.clone())
            .fetch_one(&pool)
            .await?;

        let password: String = sqlx::query_scalar("SELECT password FROM users WHERE apikey = $1")
            .bind(apikey.clone())
            .fetch_one(&pool)
            .await?;

        let balance_str: String = sqlx::query_scalar("SELECT balance FROM users WHERE apikey = $1")
            .bind(apikey.clone())
            .fetch_one(&pool)
            .await?;

        let balance: i64 = balance_str.parse()?;

        Ok(Self {
            email,
            password,
            apikey,
            balance,
        })
    }
    pub async fn get_row(email: String) -> Result<User, Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email.clone())
                .fetch_one(&pool)
                .await?;

        if !exists {
            return Err(Box::new(MissingUser("No such user was found".to_string())));
        }

        let apikey: String = sqlx::query_scalar("SELECT apikey FROM users WHERE email = $1")
            .bind(email.clone())
            .fetch_one(&pool)
            .await?;

        let password: String = sqlx::query_scalar("SELECT password FROM users WHERE email = $1")
            .bind(email.clone())
            .fetch_one(&pool)
            .await?;

        let balance_str: String = sqlx::query_scalar("SELECT balance FROM users WHERE email = $1")
            .bind(email.clone())
            .fetch_one(&pool)
            .await?;

        let balance: i64 = balance_str.parse()?;

        Ok(Self {
            email,
            password,
            apikey,
            balance,
        })
    }

    #[allow(unused)]
    pub async fn new_user(&self) -> Result<(), Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let query = "INSERT INTO users (email, password, apikey, balance) VALUES ($1, $2, $3, $4);";

        sqlx::query(query)
            .bind(&self.email)
            .bind(&self.password)
            .bind(&self.apikey)
            .bind(self.balance)
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn update_db(
        &self,
        field: TableFields,
        new_value: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let field_str = field.match_field();

        let query = format!("UPDATE users SET {} = $1 WHERE email = $2", field_str);

        let mut temp_user = User {
            email: self.email.clone(),
            password: self.password.clone(),
            apikey: self.apikey.clone(),
            balance: self.balance,
        };

        match field {
            TableFields::Email => temp_user.email = new_value.to_string(),
            TableFields::Password => temp_user.password = new_value.to_string(),
            TableFields::Apikey => temp_user.apikey = new_value.to_string(),
            TableFields::Balance => {
                temp_user.balance = new_value
                    .parse()
                    .expect("Error while trying to parse balance")
            }
        }

        auth::hash_user(&mut temp_user);

        let value_to_bind = match field {
            TableFields::Email => &temp_user.email,
            TableFields::Password => &temp_user.password,
            TableFields::Apikey => &temp_user.apikey,
            TableFields::Balance => &temp_user.balance.to_string(),
        };

        sqlx::query(&query)
            .bind(value_to_bind)
            .bind(temp_user.email.clone())
            .execute(&pool)
            .await?;

        Ok(())
    }
    #[allow(unused)]
    pub async fn login_user(username: String, password: String) -> Result<(), Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(username.clone())
                .fetch_one(&pool)
                .await?;

        if !exists {
            return Err(Box::new(MissingUser("No such user was found".to_string())));
        }

        let password_hash: String =
            sqlx::query_scalar("SELECT password FROM users WHERE username = $1")
                .bind(username.clone())
                .fetch_one(&pool)
                .await?;

        let result = match verify_password(password, password_hash.as_str()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        };

        result
    }
}

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    let url = "postgres://REDACTED";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

impl TableFields {
    pub fn match_field(&self) -> &str {
        match self {
            TableFields::Email => "email",
            TableFields::Password => "password",
            TableFields::Apikey => "apikey",
            TableFields::Balance => "balance",
        }
    }
}
