use super::{
    contract::{PgRepositoryContract, PgServiceContract, RegisterContract},
    data::UserData,
};
use async_trait::async_trait;
use error::Error;
use support::helpers::send_email;
use support::store::models::action_token::ActionToken;

pub struct Register<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> RegisterContract for Register<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    //Create user
    async fn create(&self, data: UserData, base_url: &str) -> Result<ActionToken, Error> {
        //check if user already exist in DB. If user exists return bad request
        match self.repository.check_user(&data.email).await {
            Ok(_) => Err(Error::BadRequest("Bad request".to_string())),
            Err(e) => {
                match e {
                    Error::NotFoundWithCause(_) => {
                        //create user
                        let user = self.service.create(data).await?;
                        //make action token used to verify email
                        let action_token = self.service.create_action_token(&user.id).await?;
                        //helper function to send mail
                        send_email::send(&action_token.token, base_url)?;
                        Ok(action_token)
                    }
                    _ => {
                        return Err(e);
                    }
                }
            }
        }
    }
    //Create action token
    async fn create_action_token(
        &self,
        user_email: &str,
        base_url: &str,
    ) -> Result<ActionToken, Error> {
        //check is user exist in db regarding email
        match self.repository.check_user(user_email).await {
            Ok(user) => {
                //check if email is already verified
                if user.email_verified_at.is_some() {
                    return Err(Error::BadRequest("Email already verified".to_string()));
                }
                //create action token
                let action_token = self.service.create_action_token(&user.id).await?;
                //helper function to send mail
                send_email::send(&action_token.token, base_url)?;
                Ok(action_token)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
