use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use rand::Rng;

use zxcvbn::zxcvbn;

use crate::database::User;

pub fn hasher(input: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(input.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn signup(email: String, password: String) -> Option<User> {
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

#[allow(unused)]
pub fn verify_user(to_verify: (String, String), actual: (String, String)) -> bool {
    verify(to_verify.0, actual.0) && verify(to_verify.1, actual.1)
}
#[allow(unused)]
fn verify(to_check: String, answer: String) -> bool {
    let parsed_hash = PasswordHash::new(&answer).expect("failed");
    Argon2::default()
        .verify_password(to_check.as_bytes(), &parsed_hash)
        .is_ok()
}
