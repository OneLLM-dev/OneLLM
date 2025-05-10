// #![allow(unused)]

use auth::{generate_api, signup};
// use axum::extract::Query;
use std::error::Error;
// use database::User;
// use oneAI::Input;
mod auth;
mod database;
mod requests;
mod server;
mod utils;

use server::server;

use crate::utils::*;

// fn read_line() -> String {
//     let mut user_input = String::new();
//     std::io::stdin()
//         .read_line(&mut user_input)
//         .expect("Error reading line");
//     user_input.trim().to_string()
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // db_testing().await?;
    // requests_testing().await?;
    let _ok = server().await;

    Ok(())
}

#[allow(unused)]
async fn db_testing() -> Result<(), Box<dyn std::error::Error>> {
    match database::init_db().await {
        Ok(_) => {
            println!("Successfully created a new database")
        }
        Err(e) => {
            if !e.to_string().contains("VersionMismatch") {
                panic!("ERROR WITH DataBase: {e}");
            } else {
                println!("Could this be a error? Unsure: {e}");
            }
        }
    };

    let user = signup(
        String::from("something@gmail.com"),
        String::from("Something__132;;"),
    )
    .unwrap();

    // let get = User::get_row("oa-94763257652657558374".to_string()).await?;

    // println!("{get:#?}");
    //$argon2id$v=19$m=19456,t=2,p=1$vP65R+anMrvYO2XyFUjFzA$04uceat0q5Pk0m1vicUV2RYARuHYRMZ5HKPbAun8AQc
    //$argon2id$v=19$m=19456,t=2,p=1$vP65R+anMrvYO2XyFUjFzA$04uceat0q5Pk0m1vicUV2RYARuHYRMZ5HKPbAun8AQc
    //$argon2id$v=19$m=19456,t=2,p=1$5AWHMtxmUp0TLPMLCj+RGA$jJK59R1LkEd1pDqrTIvtCPsq/KBYjbWupCkaz897PlU

    let something = user
        .update_db(TableFields::Apikey, generate_api().as_str())
        .await?;

    Ok(())
}

#[allow(unused)]
async fn requests_testing() -> Result<(), Box<dyn Error>> {
    let input_str = r#"
    {
        "endpoint": "https://api.openai.com/v1/chat/completions",
        "data": {
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": "Hello, who are you?"}]
        },
        "ai_provider": "OpenAI"
    }
    "#;

    let input = requests::Input::parse_input(input_str)?;
    println!("{:#?}", input);
    Ok(())
}

async fn _server_testing() -> Result<(), Box<dyn Error>> {
    // use server::*;

    Ok(())
}
