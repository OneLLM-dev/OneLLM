mod auth;
mod database;
mod payment;
mod pricing;
mod requests;
mod server;
mod testing;
mod utils;


use server::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    database::init_db().await?;
    let _server = server().await;

    Ok(())
}

//fn main() -> Result<(), Box<dyn Error>>{
//    dotenv().ok();
//    let email_contents = include_str!("../../one-off.html");
//        let email = Message::builder()
//        .from(Mailbox::new(
//            Some("OneLLM".to_owned()),
//            "OneLLM.dev@gmail.com".parse().unwrap(),
//        ))
//        .to(Mailbox::new(None, "hassan@gadzhi.com".parse().unwrap()))
//        .subject("Helping you cut AI costs + boost features for Whop and Consulting.com")
//        .header(ContentType::TEXT_HTML)
//        .body(email_contents.to_string())
//        .unwrap();
//
//    let creds = Credentials::new("OneLLM.dev@gmail.com".to_owned(), std::env::var("GMAIL")?);
//
//    // Open a remote connection to gmail
//    let mailer = SmtpTransport::relay("smtp.gmail.com")
//        .unwrap()
//        .credentials(creds)
//        .build();
//
//    // Send the email
//    match mailer.send(&email) {
//        Ok(_) => Ok(()),
//        Err(e) => panic!("Could not send email: {e:?}"),
//    }
//}
