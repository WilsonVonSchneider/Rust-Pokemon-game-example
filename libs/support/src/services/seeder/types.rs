use crate::store::models::{
    pokemon::APITypes,
    pokemon_type::{NewDatabasePokemonType, PokemonType},
};
use error::Error;
use infrastructure::db::DbConnection;
use log::info;

pub async fn create_types(
    api_types: Option<Vec<APITypes>>,
    pokemon_id: &String,
    connection: &mut DbConnection,
) -> Result<(), Error> {
    if let Some(api_types) = api_types {
        for r#type in api_types {
            let slot = r#type.slot;
            if let Some(r#type) = r#type.r#type {
                match PokemonType::get_by_pokemon_id_and_name(
                    pokemon_id.to_string(),
                    r#type.name.clone(),
                    connection,
                ) {
                    Ok(_) => {
                        info!("Pokemon type already exists. Skipping insertion of this type!");
                        continue;
                    }
                    Err(e) => {
                        if let Error::NotFoundWithCause(cause) = e {
                            if cause == "pokemon_type_not_found" {
                                let database_ability = NewDatabasePokemonType {
                                    slot,
                                    name: r#type.name,
                                    pokemon_id: pokemon_id.to_string(),
                                };
                                PokemonType::create(database_ability, connection)?;
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
