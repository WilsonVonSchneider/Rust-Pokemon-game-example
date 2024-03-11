use async_trait::async_trait;
use error::Error;
use super::data::{LoginData, JwtTokensData};
use support::store::models::{user::User, action_token::ActionToken};


#[async_trait]
pub trait LoginContract {
    async fn login(&self, attributes: LoginData) -> Result<JwtTokensData, Error>;
    async fn check_action_token(&self, action_token_id: &str) -> Result<ActionToken, Error>;
    async fn verify_email_and_login(&self, action_token: ActionToken) -> Result<JwtTokensData, Error>;
    async fn logout(&self, refresh_token: &str) -> Result<(), Error>;
    async fn refresh(&self, refresh_token: &str) -> Result<String, Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract { 
    async fn check_user(&self, user_email: &str) -> Result<User, Error>;
    async fn get_action_token_by_token(&self, acction_token: &str) -> Result<ActionToken, Error>;
    async fn get_user_by_action_token_entity_id(&self, action_token_entity_id: &str) -> Result<User, Error>;
    async fn get_user_by_refresh_token( &self, refresh_token: &str) -> Result<User, Error>;
}
  
// setters
#[async_trait]
pub trait PgServiceContract {
    async fn user_update(&self, user: &User) -> Result<(), Error>;
    async fn action_token_update(&self, action_token_id: &str) -> Result<(), Error>;
}