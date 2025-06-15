mod auth;
mod database;
mod pricing;
mod requests;
mod server;
mod testing;
mod utils;

use server::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = database::init_db().await?;
    let _ok = server().await;

    Ok(())
}
