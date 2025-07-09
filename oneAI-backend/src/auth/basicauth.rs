use password_auth::{generate_hash, verify_password};
use rand::Rng;

use crate::utils::*;

pub async fn login(email: String, password: String) -> Option<User> {
    let user = match User::get_row(email).await {
        Ok(a) => a,
        Err(_) => return None,
    };

    match verify_password(password, user.password.as_str()) {
        Ok(_) => {
            return Some(user);
        }
        Err(_) => return None,
    }
}

pub async fn signup(email: String, password: String) -> Option<User> {
    let mut user = User {
        id: 0,
        email,
        password,
        balance: 0,
        verified: false,
    };
    hash_user(&mut user);

    Some(user)
}

pub async fn update_bal(email: String, change: i32) -> Option<User> {
    let user = match User::get_row(email).await {
        Ok(a) => a,
        Err(_) => {
            return None;
        }
    };

    match user
        .update_db(
            TableFields::Balance,
            &(user.balance as i32 + change).to_string(),
        )
        .await
    {
        Ok(_) => {
            return Some(user);
        }
        Err(_) => {
            return None;
        }
    }
}

impl HiddenUser {
    pub async fn from_user(user: &mut User) -> Self {
        let email = user.clone().email;
        let balance = user.balance;
        drop(user.to_owned());

        return Self { email, balance };
    }
}

pub fn hash_user(user: &mut User) {
    user.password = generate_hash(&user.password)
}

pub fn generate_api() -> String {
    let mut res = String::new();
    for _ in 0..20 {
        res.push_str((rand::rng().random_range(1..10) as u8).to_string().as_str());
    }

    format!("oa-{}", res)
}
