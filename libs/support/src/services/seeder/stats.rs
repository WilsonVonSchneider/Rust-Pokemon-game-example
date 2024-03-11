use crate::store::models::{
    pokemon::APIStats,
    pokemon_stat::{NewDatabasePokemonStat, PokemonStat},
};
use error::Error;
use infrastructure::db::DbConnection;
use log::info;

pub async fn create_stats(
    api_stats: Option<Vec<APIStats>>,
    pokemon_id: &String,
    connection: &mut DbConnection,
) -> Result<(), Error> {
    if let Some(api_stats) = api_stats {
        for stat in api_stats {
            let effort = stat.effort;
            let base = stat.base_stat;
            if let Some(stat) = stat.stat {
                match PokemonStat::get_by_pokemon_id_and_name(
                    pokemon_id.to_string(),
                    stat.name.clone(),
                    connection,
                ) {
                    Ok(_) => {
                        info!("Pokemon stat already exists. Skipping insertion of this stat!");
                        continue;
                    }
                    Err(e) => {
                        if let Error::NotFoundWithCause(cause) = e {
                            if cause == "pokemon_stat_not_found" {
                                let database_ability = NewDatabasePokemonStat {
                                    effort,
                                    base,
                                    name: stat.name,
                                    pokemon_id: pokemon_id.to_string(),
                                };
                                PokemonStat::create(database_ability, connection)?;
                            }
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
