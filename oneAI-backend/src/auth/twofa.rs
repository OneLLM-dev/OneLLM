use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use redis::AsyncCommands;
use std::error::Error;

use totp_rs::{Algorithm, Secret, TOTP};

pub async fn send_verify(
    redis: &mut redis::aio::MultiplexedConnection,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let secret = Secret::generate_secret().to_bytes().unwrap();
    let encoded = Secret::Raw(secret.clone()).to_encoded();

    let secret_str = match encoded {
        Secret::Encoded(s) => s,
        Secret::Raw(_) => panic!("Expected an encoded secret, but got raw"),
    };
    let _: () = redis.set(email, secret_str).await?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,  // 6 digits
        1,  // step size (usually 1)
        60, // time period in seconds
        secret,
        Some("OneLLM".to_string()), // issuer
        email.to_string(),          // account name
    )
    .unwrap();

    let code = totp.generate_current().unwrap();
    println!("Code: {code}");

    let body = include_str!("verify.html")
        .replace("{{ YEAR }}", "2025")
        .replace("{{ CODE }}", &code);
    let email = Message::builder()
        .from(Mailbox::new(
            Some("OneLLM".to_owned()),
            "OneLLM.dev@gmail.com".parse().unwrap(),
        ))
        .to(Mailbox::new(None, email.parse().unwrap()))
        .subject("Verification for OneLLM")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    let creds = Credentials::new(
        "OneLLM.dev@gmail.com".to_owned(),
        std::env::var("GMAIL")?,
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub async fn verify_code(
    redis: &mut redis::aio::MultiplexedConnection,
    email: &str,
    user_code: &str,
) -> Result<bool, Box<dyn Error>> {
    let secret_base32: String = match redis.get(email).await.ok() {
        Some(a) => a,
        None => return Err("No secret found for provided user".into()),
    };

    let secret = Secret::Encoded(secret_base32).to_raw()?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        60,
        secret.to_bytes().unwrap(),
        Some("OneLLM".to_string()),
        email.to_string(),
    )
    .unwrap();

    let is_valid = totp.check_current(user_code).unwrap_or(false);
    Ok(is_valid)
}
