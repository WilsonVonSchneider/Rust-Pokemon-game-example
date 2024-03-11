use diesel::prelude::*;
use diesel::Queryable;
use infrastructure::{db::DbConnection, schema::pokemon_stats};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = pokemon_stats)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonStat {
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub base: Option<i32>,
    pub effort: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonStat {
    // get pokemon stat by pokemon id and name
    pub fn get_by_pokemon_id_and_name(
        pokemon_id: String,
        name: Option<String>,
        connection: &mut DbConnection,
    ) -> Result<PokemonStat, error::Error> {
        pokemon_stats::table
            .filter(pokemon_stats::pokemon_id.eq(pokemon_id))
            .filter(pokemon_stats::name.eq(name))
            .first(connection)
            .map_err(error::Error::from)
            .map_err(|e| e.add_cause_if_not_found("pokemon_stat_not_found"))
    }
    // create new pokemon stat
    pub fn create(
        data: NewDatabasePokemonStat,
        connection: &mut DbConnection,
    ) -> Result<PokemonStat, error::Error> {
        diesel::insert_into(pokemon_stats::table)
            .values(data)
            .get_result::<PokemonStat>(connection)
            .map_err(error::Error::from)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = pokemon_stats)]
pub struct NewDatabasePokemonStat {
    pub name: Option<String>,
    pub pokemon_id: String,
    pub base: Option<i32>,
    pub effort: Option<i32>,
}
