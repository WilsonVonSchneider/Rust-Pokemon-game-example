use async_trait::async_trait;
use error::Error;
use super::data::UserData;
use support::store::models::{
    user::User, action_token::ActionToken};


#[async_trait]
pub trait RegisterContract {
    async fn create(&self, attributes: UserData, base_url: &str) -> Result<ActionToken, Error>;
    async fn create_action_token(&self, user_email: &str, base_url: &str) -> Result<ActionToken, Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn check_user(&self, user_email: &str) -> Result<User, Error>;
}
  
// setters
#[async_trait]
pub trait PgServiceContract {
    async fn create(&self, data: UserData) -> Result<User, Error>;
    async fn create_action_token(&self, user_id: &str) -> Result<ActionToken, Error>; 
}