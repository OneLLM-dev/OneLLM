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
    let _db = database::init_db().await?;
    let _server = server().await;

    Ok(())
}
