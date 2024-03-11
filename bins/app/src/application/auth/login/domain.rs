use super::{
    contract::{LoginContract, PgRepositoryContract, PgServiceContract},
    data::{JwtTokensData, LoginData},
};
use async_trait::async_trait;
use error::Error;
use chrono::Utc;
use pwhash::bcrypt;
use support::{helpers::{generate_tokens, verify_refresh_token}, store::models::action_token::ActionToken};

pub struct Login<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> LoginContract for Login<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    async fn login(&self, data: LoginData) -> Result<JwtTokensData, Error> {
        //check if user exist, with provided email, in db 
        match self.repository.check_user(&data.email).await {
            Ok(mut user) => {
                // validate provided password with hashed password retrieved from DB
                if !bcrypt::verify(data.password, &user.password) {
                    return Err(Error::BadRequest("Bad request".to_string()));
                }
                // ensure that user email is verified
                if user.email_verified_at.is_none() {
                    return Err(Error::BadRequest("Email not verified".to_string()));
                }
                // generate refresh and access token with helper function
                let tokens = generate_tokens::generate_tokens(&user.id)?;
                // update user with refresh token
                user.refresh_token = Some(tokens.refresh_token.clone());
                self.service.user_update(&user).await?;
                //return refresh and access token
                Ok(JwtTokensData::new(tokens.refresh_token, tokens.access_token))
            }
            Err(e) => Err(e)
        }
    }

    async fn check_action_token(&self, action_token: &str) -> Result<ActionToken, Error> {
        //check if action token exists in database
        let action_token = self.repository.get_action_token_by_token(action_token).await?;
        // check if action token is already executed
        if action_token.executed_at.is_some() {
            return Err(Error::BadRequest("Bad request".to_string()));
        }
        // check if action token is not expired
        if action_token.expires_at.timestamp() < Utc::now().timestamp() {
            return Err(Error::BadRequest("Bad request".to_string()));
        }
        Ok(action_token)
    }

    async fn verify_email_and_login(&self, action_token: ActionToken) -> Result<JwtTokensData, Error> {
        //check if user asociated with action token exists
        match self.repository.get_user_by_action_token_entity_id(&action_token.entity_id).await {
            Ok(mut user) => {
                // check if user's email is already verified
                if user.email_verified_at.is_some() {
                    return Err(Error::BadRequest("Bad request".to_string()));
                }
                // generate refresh and access token with helper function
                let tokens = generate_tokens::generate_tokens(&user.id)?;
                // update user with refresh token and email verified at
                user.refresh_token = Some(tokens.refresh_token.clone());
                user.email_verified_at = Some(Utc::now().naive_utc());
                self.service.user_update(&user).await?;
                // update action token executed_at
                self.service.action_token_update(&action_token.id).await?;
                //return refresh and access token
                Ok(JwtTokensData::new(tokens.refresh_token, tokens.access_token))
            }
            Err(e) => Err(e)
        }
    }

    async fn logout(&self, refresh_token: &str) -> Result<(), Error> {
        
        //check if user with refresh token exists in database
        match self.repository.get_user_by_refresh_token(refresh_token).await {
            Ok(mut user) => {
                // update user's refresh token to None
                user.refresh_token = None;
                self.service.user_update(&user).await?;
                Ok(())
            }
            Err(e) => Err(e)
        }

    }

    async fn refresh(&self,refresh_token: &str) -> Result<String, Error> {
        //check if user with refresh token exists in database
        match self.repository.get_user_by_refresh_token(refresh_token).await {
            Ok(user) => {
                // verify integrity of refresh token
                verify_refresh_token::verify_refresh_token(&user, refresh_token)?;
                // generate tokens
                let tokens = generate_tokens::generate_tokens(&user.id)?;
                // return access token
                Ok(tokens.access_token)
            }
            Err(e) => Err(e)
        }
    }
}

