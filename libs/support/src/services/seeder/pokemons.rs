use super::{abilities, stats, types};
use crate::store::models::pokemon::{NewDatabasePokemon, Pokemon};
use error::Error;
use infrastructure::db::DbConnection;
use log::{error, info};

pub async fn create_pokemons(connection: &mut DbConnection) -> Result<(), Error> {
    //Get all 151 pokemons containing name of pokemon and url of single page
    let all_api_pokemons = Pokemon::get_all_pokemons_api().await?;
    //For every pokemon in 151 pokemons, use url and get single pokemon data
    for api_pokemon in all_api_pokemons {
        //Get pokemon from API with types, abilities and stats
        let api_pokemon = Pokemon::get_pokemons_one_by_one_api(api_pokemon).await?;
        let api_abilities = api_pokemon.abilities.clone();
        let api_stats = api_pokemon.stats.clone();
        let api_types = api_pokemon.types.clone();
        let pokemon_id = match api_pokemon.id {
            Some(value) => value,
            None => {
                error!("Error in fetching pokemons from API (pokemon_id not found)");
                break;
            }
        };
        //find by id in database, if exists continue, if not create pokemon
        match Pokemon::get_by_pokemon_id(pokemon_id, connection) {
            //If there is pokemon in database continue in new iteration of loop
            Ok(pokemon) => {
                info!("Pokemon already exists in DB. Skipping insertion of this pokemon!");
                //Check and seed abilities
                abilities::create_abilities(api_abilities, &pokemon.id, connection).await?;
                //Check and seed stats
                stats::create_stats(api_stats, &pokemon.id, connection).await?;
                //Check and seed types
                types::create_types(api_types, &pokemon.id, connection).await?;
                continue;
            }
            //If there is no pokemon in database create it
            Err(e) => {
                if let Error::NotFoundWithCause(cause) = e {
                    if cause == "pokemon_not_found" {
                        // Convert api_pokemon to database_pokemon using Into (or From)
                        let database_pokemon: NewDatabasePokemon = api_pokemon.into();
                        //insert pokemon in database with data prepared for database
                        let created_pokemon = match Pokemon::create(database_pokemon, connection) {
                            //If pokemon is created return pokemon object. Id of inserted pokemon will be used for inserting abilities
                            Ok(created_pokemon) => created_pokemon,
                            //If pokemon is not created return error
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        //Check and seed abilities
                        abilities::create_abilities(
                            api_abilities,
                            &created_pokemon.id,
                            connection,
                        )
                        .await?;
                        //Check and seed stats
                        stats::create_stats(api_stats, &created_pokemon.id, connection).await?;
                        //Check and seed types
                        types::create_types(api_types, &created_pokemon.id, connection).await?;
                    }
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
