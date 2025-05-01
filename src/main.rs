#![allow(unused)]

use auth::{generate_api, signup};
use database::User;
// use oneAI::Input;
mod auth;
mod database;

fn read_line() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");
    user_input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    println!("{:#?}", user);

    // user.new_user().await.expect("Error");

    let something = user
        .update_db(database::TableFields::Password, "somethingElse--_:;")
        .await?;

    Ok(())
}
