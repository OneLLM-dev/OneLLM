// use sqlx::{Pool, Postgres, Row};
use dotenv::dotenv;
use std::env;
use std::error::Error;

use crate::auth::{self, generate_api};
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
    #[allow(unused)]
    pub async fn delete_apikey(email: &str, apikey: &str) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES")?;
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let deleted = sqlx::query!(
        "DELETE FROM api_keys WHERE key = $1 AND user_id = (SELECT id FROM users WHERE email = $2)",
        apikey,
        email
    )
    .execute(&pool)
    .await?;

        if deleted.rows_affected() == 0 {
            return Err("API key not found or does not belong to the user.".into());
        }

        Ok(())
    }

    pub async fn generate_apikey(&self) -> Result<String, Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES")?;
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        // Count how many keys this user already has
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM api_keys WHERE user_id = (SELECT id FROM users WHERE email = $1)",
            self.email
        )
        .fetch_one(&pool)
        .await?
        .count
        .unwrap_or(0);

        if count >= 10 {
            return Err("You have reached the maximum of 10 API keys.".into());
        }

        // Generate a new random API key

        let new_key = generate_api();
        sqlx::query!(
            "INSERT INTO api_keys (user_id, key) VALUES ((SELECT id FROM users WHERE email = $1), $2)",
            self.email,
            new_key
        )
        .execute(&pool)
        .await?;

        Ok(new_key)
    }
    pub async fn get_row_api(apikey: String) -> Result<User, Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let row = sqlx::query!(
            "SELECT u.email, u.password, u.balance FROM users u \
             JOIN api_keys a ON u.id = a.user_id WHERE a.key = $1",
            apikey
        )
        .fetch_optional(&pool)
        .await?;

        if let Some(record) = row {
            Ok(Self {
                email: record.email,
                password: record.password,
                balance: record.balance,
            })
        } else {
            Err(Box::new(MissingUser("No such user was found".to_string())))
        }
    }

    pub async fn get_row(email: String) -> Result<User, Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let row = sqlx::query!(
            "SELECT u.password, u.balance, a.key FROM users u \
             JOIN api_keys a ON u.id = a.user_id WHERE u.email = $1",
            email
        )
        .fetch_optional(&pool)
        .await?;

        if let Some(record) = row {
            Ok(Self {
                email,
                password: record.password,
                balance: record.balance,
            })
        } else {
            Err(Box::new(MissingUser("No such user was found".to_string())))
        }
    }

    pub async fn new_user(&self) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let _user_row = sqlx::query!(
            "INSERT INTO users (email, password, balance) VALUES ($1, $2, $3) RETURNING id",
            self.email,
            self.password,
            self.balance
        )
        .fetch_one(&pool)
        .await?;

        Ok(())
    }

    pub async fn update_db(
        &self,
        field: TableFields,
        new_value: &str,
    ) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let mut temp_user = self.clone();

        match field {
            TableFields::Email => temp_user.email = new_value.to_string(),
            TableFields::Password => temp_user.password = new_value.to_string(),
            TableFields::Balance => {
                temp_user.balance = new_value
                    .parse()
                    .expect("Error while trying to parse balance")
            }
        }

        auth::hash_user(&mut temp_user);

        match field {
            TableFields::Email | TableFields::Password | TableFields::Balance => {
                let field_str = field.match_field();
                let query = format!("UPDATE users SET {} = $1 WHERE email = $2", field_str);

                let balance = temp_user.balance.to_string();
                sqlx::query(&query)
                    .bind(match field {
                        TableFields::Email => &temp_user.email,
                        TableFields::Password => &temp_user.password,
                        TableFields::Balance => &balance,
                    })
                    .bind(&self.email)
                    .execute(&pool)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn delete_user(email: &str) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv::from_filename(".env.ci").ok();
        } else {
            dotenv().ok();
        }

        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let result = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::new(MissingUser("User not found".into())));
        }

        Ok(())
    }
}
pub async fn init_db() -> Result<(), Box<dyn Error>> {
    if std::env::var("CI").is_err() {
        dotenv::from_filename(".env.ci").ok();
    } else {
        dotenv().ok();
    }
    let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

impl TableFields {
    pub fn match_field(&self) -> &str {
        match self {
            TableFields::Email => "email",
            TableFields::Password => "password",
            TableFields::Balance => "balance",
        }
    }
}
