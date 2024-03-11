mod abilities;
mod pokemons;
mod stats;
mod types;
mod test;

use infrastructure::db::{DbConnection, Pg};
use log::{error, info};
use std::sync::Arc;

/// Run all the seeders, this will run the dev seeders if we are in dev env.
pub async fn run(pg: Arc<Pg>) -> Result<(), error::Error> {
    info!("Starting seeding process...");

    if &config::get_default("IS_DEV", "")[..] == "true" {
        match pg.connection() {
            Ok(connection) => run_dev(connection).await?,
            Err(e) => error!("Couldn't set database connection. Cause: {}", e),
        };
    }

    Ok(info!("Seeding completed."))
}

/// Runs all the seeders that should only run in dev environment
pub async fn run_dev(mut connection: DbConnection) -> Result<(), error::Error> {
    pokemons::create_pokemons(&mut connection).await?;
    Ok(())
}
