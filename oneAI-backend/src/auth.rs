use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use password_auth::verify_password;
use rand::Rng;

use zxcvbn::zxcvbn;

use crate::utils::*;

pub fn hasher(input: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(input.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub async fn login(email: String, password: String) -> Option<User> {
    let user = match User::get_row(email).await {
        Ok(a) => a,
        Err(_) => return None,
    };

    match verify_password(password, user.password.as_str()) {
        Ok(_) => return Some(user),
        Err(_) => return None,
    }
}

pub async fn signup(email: String, password: String) -> Option<User> {
    match zxcvbn(&password, &[email.as_str()]) {
        Ok(entropy) => {
            if entropy.score() >= 3 {
                let apikey = generate_api();
                let user = User {
                    email,
                    password,
                    apikey,
                    balance: 0,
                };

                return Some(hash_user(user));
            } else {
                println!("Password isn't strong enough");
            }
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    None
}

pub async fn update_bal(email: String, password: String, change: i64) -> Option<User> {
    let user = match login(email, password).await {
        Some(a) => a,
        None => return None,
    };

    match user
        .update_db(TableFields::Balance, &(user.balance + change).to_string())
        .await
    {
        Ok(_) => {}
        Err(_) => return None,
    }

    None
}

pub fn hash_user(user: User) -> User {
    User {
        email: user.email,
        password: hasher(user.password.as_str()),
        apikey: generate_api(),
        balance: 0,
    }
}

pub fn generate_api() -> String {
    let mut res = String::new();
    for _ in 0..20 {
        res.push_str((rand::rng().random_range(1..10) as u8).to_string().as_str());
    }

    format!("oa-{}", res)
}
