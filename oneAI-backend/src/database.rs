use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::{PgPool, Row};
use std::error::Error;

use crate::auth::basicauth::generate_api;
use crate::{auth, utils::*};

#[derive(Debug)]
struct MissingUser(String);
impl Error for MissingUser {}

impl std::fmt::Display for MissingUser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

pub async fn init_pool() -> Result<PgPool, Box<dyn Error>> {
    if std::env::var("CI").is_err() {
        dotenv().ok();
    }

    let url = std::env::var("POSTGRES")?;
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    Ok(pool)
}

impl User {
    pub async fn from_token(
        pool: Option<PgPool>,
        token: String,
    ) -> Result<HiddenUser, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let row = sqlx::query(
            r#"
            SELECT u.id, u.email, u.password, u.balance, u.verified
            FROM users u
            INNER JOIN sessions s ON s.user_id = u.id
            WHERE s.token = $1 AND s.expires_at > NOW()
            "#,
        )
        .bind(&token)
        .fetch_one(&pool)
        .await?;

        let bal: i32 = row.get("balance");
        let mut user = User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            balance: bal as u32,
            verified: row.get("verified"),
        };

        Ok(HiddenUser::from_user(&mut user).await)
    }

    pub async fn new_token(
        pool: Option<PgPool>,
        user_id: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let now = Utc::now();
        let exp = now + Duration::hours(24);

        let claims = Claims {
            sub: user_id,
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        };

        let secret = std::env::var("JWT_SECRET")?;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        // Store token in `sessions` table
        sqlx::query(
            r#"
        INSERT INTO sessions (user_id, token, created_at, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(user_id)
        .bind(&token)
        .bind(now)
        .bind(exp)
        .execute(&pool)
        .await?;

        Ok(token)
    }

    //    pub async fn verify_token(payload: &TokenInput) -> Result<(), Box<dyn Error>> {
    //        let pool = init_pool().await?;
    //
    //        // Check if token exists and is not expired
    //        let session_row =
    //            sqlx::query("SELECT user_id FROM sessions WHERE token = $1 AND expires_at > NOW()")
    //                .bind(&payload.token)
    //                .fetch_optional(&pool)
    //                .await?;
    //
    //        let user_id = match session_row {
    //            Some(row) => row.get::<i32, _>("user_id"),
    //            None => return Err("Invalid or expired token".into()),
    //        };
    //
    //        // Get user email if not in payload
    //        let email = if let Some(email) = &payload.email {
    //            email.clone()
    //        } else {
    //            let row = sqlx::query("SELECT email FROM users WHERE id = $1")
    //                .bind(user_id)
    //                .fetch_one(&pool)
    //                .await?;
    //            row.get::<String, _>("email")
    //        };
    //
    //        // Mark user as verified
    //        sqlx::query("UPDATE users SET verified = TRUE WHERE email = $1")
    //            .bind(email)
    //            .execute(&pool)
    //            .await?;
    //
    //        // Optionally mark session as verified too
    //        sqlx::query("UPDATE sessions SET verified = TRUE WHERE token = $1")
    //            .bind(&payload.token)
    //            .execute(&pool)
    //            .await?;
    //
    //        Ok(())
    //    }

    pub async fn is_verified(&self, pool: Option<PgPool>) -> Result<bool, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let row = sqlx::query("SELECT verified FROM users WHERE email = $1")
            .bind(&self.email)
            .fetch_optional(&pool)
            .await?;

        match row {
            Some(row) => Ok(row.get::<bool, _>("verified")),
            None => Ok(false), // Or Err("User not found") if you prefer
        }
    }
    pub async fn verify_user(pool: Option<PgPool>, email: &str) -> Result<(), Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        sqlx::query("UPDATE users SET verified = TRUE WHERE email = $1")
            .bind(email)
            .execute(&pool)
            .await?;

        Ok(())
    }
    pub async fn count_apikey(pool: Option<PgPool>, email: &str) -> Result<i64, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let row = sqlx::query(
            "SELECT COUNT(*) as count \
             FROM api_keys \
             WHERE user_id = (SELECT id FROM users WHERE email = $1)",
        )
        .bind(email)
        .fetch_one(&pool)
        .await?;
        let count: i64 = row.try_get("count")?;

        Ok(count)
    }

    pub async fn delete_apikey(
        pool: Option<PgPool>,
        token: &str,
        name: Option<&str>,
        all: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        // 1. Decode JWT and get user_id
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;
        let user_id = token_data.claims.sub; // assuming `sub` is user_id

        // 2. Delete all or one API key
        let query = if all {
            sqlx::query("DELETE FROM api_keys WHERE user_id = $1").bind(user_id)
        } else {
            let name = name.ok_or("Missing API key name when 'all' is false")?;
            sqlx::query("DELETE FROM api_keys WHERE user_id = $1 AND name = $2")
                .bind(user_id)
                .bind(name)
        };

        let result = query.execute(&pool).await?;

        if result.rows_affected() == 0 {
            return Err("No API key(s) found to delete.".into());
        }

        Ok(())
    }
    pub async fn generate_apikey(
        &self,
        pool: Option<PgPool>,
        name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        // Count how many keys this user already has
        let count = Self::count_apikey(Some(pool.clone()), &self.email).await?;

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
    pub async fn get_row_api(pool: Option<PgPool>, apikey: String) -> Result<User, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let row = sqlx::query(
            "SELECT users.id, users.email, users.password, users.balance, users.verified \
     FROM users \
     JOIN api_keys a ON users.id = a.user_id \
     WHERE a.key = $1",
        )
        .bind(apikey)
        .fetch_optional(&pool)
        .await?;

        if let Some(record) = row {
            let balance: i32 = record.try_get("balance")?;

            Ok(User {
                id: record.get("id"),
                email: record.get("email"),
                password: record.get("password"),
                balance: balance as u32,
                verified: record.get("verified"),
            })
        } else {
            Err(Box::new(MissingUser("No such user was found".to_string())))
        }
    }

    pub async fn get_row(pool: Option<PgPool>, email: String) -> Result<User, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        let row = sqlx::query(
            "SELECT id, email, password, balance, verified FROM users WHERE email = $1",
        )
        .bind(&email)
        .fetch_optional(&pool)
        .await?;

        if let Some(record) = row {
            let balance: i32 = record.try_get("balance")?;

            Ok(User {
                id: record.get("id"),
                email: record.get("email"),
                password: record.get("password"),
                balance: balance as u32,
                verified: record.get("verified"),
            })
        } else {
            Err(Box::new(MissingUser("Invalid apikey".to_string())))
        }
    }

    #[allow(unused)]
    pub async fn change_password(
        pool: Option<PgPool>,
        token: &str,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

        // Check if token is verified and not expired
        let verified =
            sqlx::query("SELECT verified FROM sessions WHERE token = $1 AND expires_at > NOW()")
                .bind(token)
                .fetch_optional(&pool)
                .await?
                .map(|r| r.get::<bool, _>("verified"))
                .unwrap_or(false);

        if !verified {
            return Err("Attempt at using unverified or expired token".into());
        }

        // Get user email from session
        let row = sqlx::query("SELECT user_id FROM sessions WHERE token = $1")
            .bind(token)
            .fetch_one(&pool)
            .await?;

        let user_id: i32 = row.get("user_id");

        // Get full user row from users table
        let user_row =
            sqlx::query("SELECT id, email, password, balance, verified FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await?;

        // Construct the User object
        let user = User {
            id: user_row.get("id"),
            email: user_row.get("email"),
            password: user_row.get("password"),
            balance: user_row.get::<i32, _>("balance") as u32,
            verified: user_row.get("verified"),
        };

        // Call the update method on the user to update password
        user.update_db(Some(pool), TableFields::Password, &new_password)
            .await?;

        Ok(())
    }
    pub async fn get_keynames(
        pool: Option<PgPool>,
        email: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

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
    pub async fn new_user(&self, pool: Option<PgPool>) -> Result<(), Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

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
        pool: Option<PgPool>,
        field: TableFields,
        new_value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

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
    pub async fn delete_user(pool: Option<PgPool>, email: &str) -> Result<(), Box<dyn Error>> {
        let pool = match pool {
            Some(a) => a,
            None => init_pool().await?,
        };

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
    let pool = init_pool().await?;

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
