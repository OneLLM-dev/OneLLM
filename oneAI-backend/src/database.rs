// use sqlx::{Pool, Postgres, Row};
use dotenv::dotenv;
use sqlx::Row;
use std::env;
use std::error::Error;

use crate::auth::basicauth::{generate_api, login};
use crate::{auth, utils::*};

#[derive(Debug)]
struct MissingUser(String);
impl Error for MissingUser {}

impl std::fmt::Display for MissingUser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl User {
    pub async fn count_apikey(email: &str) -> Result<i64, Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES")?;
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let row = sqlx::query(
            "SELECT COUNT(*) as count \
             FROM api_keys \
             WHERE user_id = (SELECT id FROM users WHERE email = $1)",
        )
        .bind(&email)
        .fetch_one(&pool)
        .await?;
        let count: i64 = row.try_get("count")?;

        Ok(count)
    }

    pub async fn delete_apikey(
        email: &str,
        password: &str,
        name: &str,
        all: bool,
    ) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES")?;
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        if let None = login(email.to_string(), password.to_string()).await {
            return Err("Couldnt log user in".into());
        };

        let mut deleted = sqlx::query(
            "DELETE FROM api_keys \
             WHERE name = $1 AND user_id = \
             (SELECT id FROM users WHERE email = $2)",
        );

        if all {
            deleted = sqlx::query(
                "DELETE FROM api_keys \
                WHERE user_id = (SELECT id FROM users WHERE email = $1)",
            );
        }

        let executed = deleted.bind(name).bind(email).execute(&pool).await?;

        if executed.rows_affected() == 0 {
            return Err("API key not found or does not belong to the user.".into());
        }

        Ok(())
    }

    pub async fn generate_apikey(&self, name: &str) -> Result<String, Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES")?;
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        // Count how many keys this user already has
        let count = Self::count_apikey(&self.email).await?;

        if count >= 10 {
            return Err("You have reached the maximum of 10 API keys.".into());
        }

        // Generate a new random API key

        let new_key = generate_api();
        sqlx::query(
            "INSERT INTO api_keys (user_id, key, name) VALUES ((SELECT id FROM users WHERE email = $1), $2, $3)",
        ).bind(&self.email).bind(&new_key).bind(name)
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

        let row = sqlx::query(
            "SELECT u.email, u.password, u.balance FROM users u \
             JOIN api_keys a ON u.id = a.user_id WHERE a.key = $1",
        )
        .bind(apikey)
        .fetch_optional(&pool)
        .await?;

        if let Some(record) = row {
            let email: String = record.try_get("email")?;
            let password: String = record.try_get("password")?;
            let balance: i32 = record.try_get("balance")?;

            Ok(Self {
                email,
                password,
                balance: balance as u32,
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

        let row = sqlx::query("SELECT email, password, balance FROM users WHERE email = $1")
            .bind(&email)
            .fetch_optional(&pool)
            .await?;

        if let Some(record) = row {
            let password: String = record.try_get("password")?;
            let balance: i32 = record.try_get("balance")?;

            //let _key: String = record.try_get("key")?; // if you want to use it

            Ok(Self {
                email,
                password,
                balance: balance as u32,
            })
        } else {
            Err(Box::new(MissingUser("No such user was found".to_string())))
        }
    }

    pub async fn get_keynames(email: &str) -> Result<Vec<String>, Box<dyn Error>> {
        // Load .env unless running in CI
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }

        // Connect to DB
        let url = env::var("POSTGRES").expect("Postgres DB URL not found");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        // Get API key names
        let rows = sqlx::query(
            "SELECT api_keys.name 
         FROM api_keys
         JOIN users ON api_keys.user_id = users.id
         WHERE users.email = $1",
        )
        .bind(email)
        .fetch_all(&pool)
        .await?;
        let keynames = rows
            .into_iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        Ok(keynames)
    }
    pub async fn new_user(&self) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let _user_row = sqlx::query(
            "INSERT INTO users (email, password, balance) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&self.email)
        .bind(&self.password)
        .bind(self.balance as i32)
        .fetch_one(&pool)
        .await?;

        Ok(())
    }

    pub async fn update_db(
        &self,
        field: TableFields,
        new_value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }
        let url = std::env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let mut temp_user = self.clone();

        // Update the in-memory user struct with the new value
        match field {
            TableFields::Email => temp_user.email = new_value.to_string(),
            TableFields::Password => temp_user.password = new_value.to_string(),
            TableFields::Balance => {
                temp_user.balance = new_value
                    .parse::<u32>()
                    .expect("Error while trying to parse balance");
            }
        }

        // Apply hashing logic if needed (likely hashes password if field == Password)
        auth::basicauth::hash_user(&mut temp_user);

        // Prepare the SQL query
        let field_str = field.match_field();
        let query = format!("UPDATE users SET {} = $1 WHERE email = $2", field_str);

        let mut query_builder = sqlx::query(&query);

        // Bind the correct value based on the field type
        match field {
            TableFields::Email => {
                query_builder = query_builder.bind(&temp_user.email);
            }
            TableFields::Password => {
                query_builder = query_builder.bind(&temp_user.password);
            }
            TableFields::Balance => {
                query_builder = query_builder.bind(temp_user.balance as i32);
            }
        }

        // Bind the email for the WHERE clause and execute
        query_builder.bind(&self.email).execute(&pool).await?;

        Ok(())
    }
    #[allow(unused)]
    pub async fn delete_user(email: &str) -> Result<(), Box<dyn Error>> {
        if std::env::var("CI").is_err() {
            dotenv().ok();
        }

        let url = env::var("POSTGRES").expect("POSTGRES DB URL NOT FOUND");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;

        let result = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
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
