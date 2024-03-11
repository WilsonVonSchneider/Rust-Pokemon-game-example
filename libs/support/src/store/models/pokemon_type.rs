use diesel::prelude::*;
use diesel::Queryable;
use infrastructure::{db::DbConnection, schema::pokemon_types};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = pokemon_types)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonType {
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub slot: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonType {
    // get pokemon type by pokemon id and name
    pub fn get_by_pokemon_id_and_name(
        pokemon_id: String,
        name: Option<String>,
        connection: &mut DbConnection,
    ) -> Result<PokemonType, error::Error> {
        pokemon_types::table
        .filter(pokemon_types::pokemon_id.eq(pokemon_id))
        .filter(pokemon_types::name.eq(name))
        .first(connection)
        .map_err(error::Error::from)
        .map_err(|e| e.add_cause_if_not_found("pokemon_type_not_found"))
    }
    // create new pokemon type
    pub fn create(
        data: NewDatabasePokemonType,
        connection: &mut DbConnection,
    ) -> Result<PokemonType, error::Error> {
        diesel::insert_into(pokemon_types::table)
        .values(data)
        .get_result::<PokemonType>(connection)
        .map_err(error::Error::from)
    }

}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = pokemon_types)]
pub struct NewDatabasePokemonType {
    pub pokemon_id: String,
    pub name: Option<String>,
    pub slot: Option<i32>,
}
