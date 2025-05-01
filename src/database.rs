use sqlx::{Pool, Postgres, Row};
use std::error::Error;

use crate::auth::{self, signup};

pub enum TableFields {
    Email,
    Password,
    Apikey,
    Balance,
}

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub apikey: String,
    pub balance: f64,
}

impl User {
    pub async fn new_user(&self) -> Result<(), Box<dyn Error>> {
        let url = "postgres://REDACTED";
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        let query = "INSERT INTO users (email, password, apikey, balance) VALUES ($1, $2, $3, $4);";

        let hashed_user = signup(self.email.clone(), self.password.clone()).unwrap();

        sqlx::query(query)
            .bind(&hashed_user.email)
            .bind(&hashed_user.password)
            .bind(&hashed_user.apikey)
            .bind(hashed_user.balance)
            .execute(&pool)
            .await?;

        Ok(())
    }

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

        let temp_struct = auth::hash_user(temp_user);

        sqlx::query(&query)
            .bind(new_value)
            .bind(temp_struct.email)
            .execute(&pool)
            .await?;

        Ok(())
    }
}

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    let url = "postgres://REDACTED";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

impl TableFields {
    fn match_field(&self) -> &str {
        match self {
            TableFields::Email => "email",
            TableFields::Password => "password",
            TableFields::Apikey => "apikey",
            TableFields::Balance => "balance",
        }
    }
}
