use diesel::prelude::*;
use diesel::Queryable;
use infrastructure::{db::DbConnection, schema::pokemon_abilities};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = pokemon_abilities)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonAbility {
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonAbility {
        // get pokemon ability by pokemon id and name
    pub fn get_by_pokemon_id_and_name(
        pokemon_id: String,
        name: Option<String>,
        connection: &mut DbConnection,
    ) -> Result<PokemonAbility, error::Error> {
        pokemon_abilities::table
            .filter(pokemon_abilities::pokemon_id.eq(pokemon_id))
            .filter(pokemon_abilities::name.eq(name))
            .first(connection)
            .map_err(error::Error::from)
            .map_err(|e| e.add_cause_if_not_found("pokemon_abilitiy_not_found"))
    }
    // create new pokemon ability
    pub fn create(
        data: NewDatabasePokemonAbility,
        connection: &mut DbConnection,
    ) -> Result<PokemonAbility, error::Error> {
        diesel::insert_into(pokemon_abilities::table)
            .values(data)
            .get_result::<PokemonAbility>(connection)
            .map_err(error::Error::from)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = pokemon_abilities)]
pub struct NewDatabasePokemonAbility {
    pub name: Option<String>,
    pub pokemon_id: String,
}
