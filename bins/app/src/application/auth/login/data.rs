use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable};
use infrastructure::schema::users;
use validr::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestLoginData {
    pub email: Option<String>,
    pub password: Option<String>,
}
impl Validation for RequestLoginData {
    /// Modifiers for RequestUserData
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(email),
            modifier_trim!(password),
        ]
    }
    /// Rules for RequestUserData
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(password),
        ]
    }
}

impl RequestLoginData {
    pub fn insertable(self) -> LoginData  {
        let email = self.email.unwrap();
        let password = self.password.unwrap();
        LoginData{
            email,
            password
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct JwtTokensData {
    pub refresh_token: String,
    pub access_token: String,
}
impl JwtTokensData {
    pub fn new(refresh_token: String, access_token: String) -> Self {
        Self {
            refresh_token,
            access_token
        }
    }
    
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ResponseData {
    pub access_token: String
}
impl ResponseData {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token
        }
    }
    
}
