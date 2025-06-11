use password_auth::{generate_hash, verify_password};
use rand::Rng;

use zxcvbn::zxcvbn;

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
    match zxcvbn(&password, &[email.as_str()]) {
        Ok(entropy) => {
            if entropy.score() >= 3 {
                let apikey = generate_api();
                let mut user = User {
                    email,
                    password,
                    apikey,
                    balance: 0,
                };
                hash_user(&mut user);

                return Some(user);
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
