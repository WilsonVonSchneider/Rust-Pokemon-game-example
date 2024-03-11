use diesel::{Queryable, AsChangeset};
use infrastructure::{db::DbConnection, schema::user_pokedexes};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use error::Error;

use chrono::NaiveDateTime;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    AsChangeset,
)]
#[diesel(table_name = user_pokedexes)]
#[diesel(treat_none_as_null = true)]
pub struct UserPokedex {
    pub id: String,
    pub user_id: String,
    pub pokemon_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserPokedex {
    // helper function for get action token by token column
    pub fn get_all_by_user_id(
        user_id: &str,
        connection: &mut DbConnection
    ) -> Result<Vec<UserPokedex>, error::Error> {
        user_pokedexes::table
        .filter(user_pokedexes::user_id.eq(user_id))
        .load::<UserPokedex>(connection)
        .map_err(Error::from)
    }
}

