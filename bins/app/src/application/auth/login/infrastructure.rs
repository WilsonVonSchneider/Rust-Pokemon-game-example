use super::contract::{PgRepositoryContract, PgServiceContract};
use async_trait::async_trait;
use error::Error;
use std::sync::Arc;
use support::store::models::{user::User, action_token::ActionToken};
use infrastructure::{
    db::Pg,
    schema::action_tokens,
};
use chrono::Utc;
use diesel::prelude::*;

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    // check if user exist, with provided email, in db 
    async fn check_user(&self, user_email: &str) -> Result<User, Error> {
        let mut connection = self.pg_pool.connection()?;
        User::get_by_email(user_email, &mut connection)
    }
    // get action token by token 
    async fn get_action_token_by_token(&self, action_token: &str) -> Result<ActionToken, Error> {
        let mut connection = self.pg_pool.connection()?;
        let action_token = ActionToken::get_by_token(action_token, &mut connection)?;
        Ok(action_token)
    }
    // get user by entity_id column in action token
    async fn get_user_by_action_token_entity_id(&self, action_token_entity_id: &str) -> Result<User, Error> {
        let mut connection = self.pg_pool.connection()?;
        User::get_by_id(action_token_entity_id, &mut connection)
    }
    // get user by refresh token
    async fn get_user_by_refresh_token( &self, refresh_token: &str) -> Result<User, Error> {
        let mut connection = self.pg_pool.connection()?;
        User::get_by_refresh_token(refresh_token, &mut connection)
    }
}

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {

    // update action token
    async fn action_token_update(&self, action_token_id: &str) -> Result<(), Error> {
        let mut connection = self.pg_pool.connection()?;
        diesel::update(action_tokens::table)
            .filter(action_tokens::id.eq(action_token_id))
            .set(action_tokens::executed_at.eq(Utc::now()))
            .execute(&mut connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("Message not found"))?;

        Ok(())
    }
    
    // update user
    async fn user_update(&self, user: &User) -> Result<(), Error> {
        let mut connection = self.pg_pool.connection()?;
        User::update_user(&mut connection, user)?;
        Ok(())
    }
}