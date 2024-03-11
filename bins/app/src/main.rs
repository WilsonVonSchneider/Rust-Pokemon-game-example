mod application;
mod middleware;
pub mod web;
use infrastructure::db::Pg;
use log::error;
use std::sync::Arc;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    config::initialize().await;

    let pg = Arc::new(Pg::new());

    // Run the seeder in order to generate everything needed for the application to run
    // or to test the application.
    match support::services::seeder::run(pg.clone()).await {
        Ok(()) => (),
        Err(e) => error!("Error seeding DB. Cause: {}", e),
    };

    //Start web server
    web::start_server(pg).await.expect("Error while starting/running http server");
}
