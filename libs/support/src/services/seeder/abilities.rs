use crate::store::models::{
    pokemon::APIAbilities,
    pokemon_ability::{NewDatabasePokemonAbility, PokemonAbility},
};
use error::Error;
use infrastructure::db::DbConnection;
use log::info;

pub async fn create_abilities(
    api_abilities: Option<Vec<APIAbilities>>,
    pokemon_id: &String,
    connection: &mut DbConnection,
) -> Result<(), Error> {
    if let Some(api_abilities) = api_abilities {
        for ability in api_abilities {
            if let Some(ability) = ability.ability {
                match PokemonAbility::get_by_pokemon_id_and_name(
                    pokemon_id.to_string(),
                    ability.name.clone(),
                    connection,
                ) {
                    Ok(_) => {
                        info!(
                            "Pokemon ability already exists. Skipping insertion of this ability!"
                        );
                        continue;
                    }
                    Err(e) => {
                        if let Error::NotFoundWithCause(cause) = e {
                            if cause == "pokemon_abilitiy_not_found" {
                                let database_ability = NewDatabasePokemonAbility {
                                    name: ability.name,
                                    pokemon_id: pokemon_id.to_string(),
                                };
                                PokemonAbility::create(database_ability, connection)?;
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
