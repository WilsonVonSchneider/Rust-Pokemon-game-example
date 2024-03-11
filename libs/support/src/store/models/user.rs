use diesel::{AsChangeset, Queryable};
use infrastructure::{db::DbConnection, schema::users};
use serde::{Deserialize, Serialize};
use error::Error;

use diesel::prelude::*;

use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    // get user by email
    pub fn get_by_email(email: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::email.eq(email))
            .first(connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("user not found"))
    }
    //get user by id
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::id.eq(id))
            .first(connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("user not found"))
    }
    //get user by refresh token
    pub fn get_by_refresh_token(refresh_token: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::refresh_token.eq(refresh_token))
            .first(connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("user not found"))
    }
    //update user
    pub fn update_user(conn: &mut DbConnection, user: &User) -> Result<User, Error> {
        diesel::update(users::dsl::users)
            .set(user)
            .get_result::<User>(conn)
            .map_err(Error::from)
    }
}
