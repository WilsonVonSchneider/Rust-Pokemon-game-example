use super::{
    contract::{PgRepositoryContract, PgServiceContract},
    data::{ActionTokenData, UserData},
};
use async_trait::async_trait;
use diesel::RunQueryDsl;
use error::Error;
use infrastructure::{
    db::Pg,
    schema::{action_tokens, users},
};
use support::helpers::generate_tokens;
use std::sync::Arc;
use support::store::models::{action_token::ActionToken, user::User};

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    // chek if user exists by email
    async fn check_user(&self, user_email: &str) -> Result<User, Error> {
        let mut connection = self.pg_pool.connection()?;
        User::get_by_email(user_email, &mut connection)
    }
}

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    // create new user
    async fn create(&self, data: UserData) -> Result<User, Error> {
        let mut connection = self.pg_pool.connection()?;
        diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(&mut connection)
            .map_err(Error::from)
    }
    // create new action token
    async fn create_action_token(&self, user_id: &str) -> Result<ActionToken, Error> {
        let mut connection = self.pg_pool.connection()?;
        let data = ActionTokenData {
            entity_id: user_id.to_string(),
            action_name: "email verification".to_string(),
            token: generate_tokens::generate_random_action_token(10)
        };
        diesel::insert_into(action_tokens::table)
            .values(data)
            .get_result::<ActionToken>(&mut connection)
            .map_err(Error::from)
    }
}
