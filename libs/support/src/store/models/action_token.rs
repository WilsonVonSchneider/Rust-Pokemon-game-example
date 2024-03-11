use diesel::{Queryable, AsChangeset};
use infrastructure::{db::DbConnection, schema::action_tokens};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use chrono::NaiveDateTime;

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    AsChangeset,
)]
#[diesel(table_name = action_tokens)]
#[diesel(treat_none_as_null = true)]
pub struct ActionToken {
    pub id: String,
    pub entity_id: String,
    pub token: String,
    pub action_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub executed_at: Option<NaiveDateTime>
}

impl ActionToken {
    // helper function for get action token by token column
    pub fn get_by_token(
        token: &str,
        connection: &mut DbConnection
    ) -> Result<ActionToken, error::Error> {
        action_tokens::table
        .filter(action_tokens::token.eq(token)) 
        .first(connection)
        .map_err(error::Error::from)
        .map_err(|e| e.add_cause_if_not_found("action token not found")) 
    }
}