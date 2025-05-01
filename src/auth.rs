use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use rand::Rng;
use zxcvbn::zxcvbn;

use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::database::{self, User};

pub fn hasher(input: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(input.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn std_hash(input: String) -> String {
    let mut output = DefaultHasher::new();
    input.hash(&mut output);
    output.finish().to_string()
}

pub fn signup(email: String, password: String) -> Option<User> {
    match zxcvbn(&password, &[email.as_str()]) {
        Ok(entropy) => {
            if entropy.score() >= 3 {
                let user = User {
                    email,
                    password,
                    apikey: generate_api(),
                    balance: 0.0,
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
        email: std_hash(user.email),
        password: hasher(user.password.as_str()),
        apikey: std_hash(generate_api()),
        balance: 0.0,
    }
}

pub fn generate_api() -> String {
    let mut res = String::new();
    for _ in 0..20 {
        res.push_str((rand::rng().random_range(1..10) as u8).to_string().as_str());
    }

    format!("oa-{}", res)
}

pub fn verify_user(to_verify: (String, String), actual: (String, String)) -> bool {
    verify(to_verify.0, actual.0) && verify(to_verify.1, actual.1)
}

fn verify(to_check: String, answer: String) -> bool {
    let parsed_hash = PasswordHash::new(&answer).expect("failed");
    Argon2::default()
        .verify_password(to_check.as_bytes(), &parsed_hash)
        .is_ok()
}
