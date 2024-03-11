use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use error::Error;
use infrastructure::{db::DbConnection, schema::user_attempts};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = user_attempts)]
#[diesel(treat_none_as_null = true)]
pub struct UserAttempt {
    pub id: String,
    pub user_id: String,
    pub pokemon_id: String,
    pub is_successful: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserAttempt {
    // get attempt by pokemon id
    pub fn get_by_pokemon_id(
        pokemon_id: &str,
        connection: &mut DbConnection,
    ) -> Result<UserAttempt, Error> {
        user_attempts::table
            .filter(user_attempts::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("attempt not found"))
    }
}
