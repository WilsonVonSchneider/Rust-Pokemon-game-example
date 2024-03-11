use diesel::Insertable;
use infrastructure::schema::{user_attempts, user_pokedexes};
use serde::{Deserialize, Serialize};
use validr::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestAttemptData {
    pub guess: Option<String>,
}

impl Validation for RequestAttemptData {
    /// Modifiers for RequestAttemptrData
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(guess),
            modifier_lowercase!(guess)
            ]
    }
    /// Rules for RequestAttemptData
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![rule_required!(guess)]
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = user_attempts)]
#[diesel(treat_none_as_null = true)]
pub struct UserAttemptData {
    pub user_id: String,
    pub pokemon_id: String,
}

impl UserAttemptData {
    pub fn new(user_id: &str, pokemon_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            pokemon_id: pokemon_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = user_pokedexes)]
#[diesel(treat_none_as_null = true)]
pub struct UserPokedexData {
    pub user_id: String,
    pub pokemon_id: String,
}

impl UserPokedexData {
    pub fn new(user_id: &str, pokemon_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            pokemon_id: pokemon_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ResponseData {
    pub pokemon_image: String,
    pub attempt_id: String,
}
impl ResponseData {
    pub fn new(pokemon_image: String, attempt_id: String) -> Self {
        Self {
            pokemon_image,
            attempt_id,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestPaginatedData {
    pub page: Option<i32>,
    pub limit: Option<i32>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct PaginatedData {
    pub page: i32,
    pub limit: i32
}


impl RequestPaginatedData {
    pub fn check(&self) -> PaginatedData {
        let mut page = self.page.unwrap_or(1);
        if page <= 0 {
            page = 1;
        }
        let limit = self.limit.unwrap_or(10);
        PaginatedData {
            page: page - 1,
            limit
        }

    }
}

