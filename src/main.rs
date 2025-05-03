// #![allow(unused)]

use auth::signup;
use database::User;
// use oneAI::Input;
mod auth;
mod database;
mod requests;
mod server;

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

    let _user = signup(
        String::from("something@gmail.com"),
        String::from("Something__132;;"),
    )
    .unwrap();

    let get = User::get_row("oa-94763257652657558374".to_string()).await?;

    println!("{get:#?}");

    // let user_something = user.new_user().await.expect("Error");

    // let _ = user
    //     .update_db(database::TableFields::Password, "somethingElse--_:;")
    //     .await?;
    //
    Ok(())
}
