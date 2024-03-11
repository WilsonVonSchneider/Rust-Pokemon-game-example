use async_trait::async_trait;
use error::Error;
use support::store::models::{
    pokemon::Pokemon, user_attempts::UserAttempt, user_pokedex::UserPokedex,
};

use super::data::{ResponseData, PaginatedData};

#[async_trait]
pub trait PokemonGameContract {
    async fn start_game(&self, authenticated_user_id: &str) -> Result<ResponseData, Error>;
    async fn check_guess(
        &self,
        authenticated_user_id: &str,
        guess: &str,
        attempt_id: &str,
    ) -> Result<(), Error>;
    async fn paginated(&self, authenticated_user_id: &str, data: PaginatedData) -> Result<Vec<Pokemon>, Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_all_pokemons(&self) -> Result<Vec<Pokemon>, Error>;
    async fn get_all_user_pokemons(&self, user_id: &str) -> Result<Vec<UserPokedex>, Error>;
    async fn get_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error>;
    async fn get_attempt_by_pokemon_id(&self, pokemon_id: &str) -> Result<UserAttempt, Error>;
    async fn check_attempt(&self, attempt_id: &str, authenticated_user_id: &str) -> Result<UserAttempt, Error>;
    async fn paginated(&self, user_pokemon_ids: Vec<&str>, page: i32, limit: i32) -> Result<Vec<Pokemon>, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract {
    async fn create_new_attempt(
        &self,
        pokemon_id: &str,
        authenticated_user_id: &str,
    ) -> Result<UserAttempt, Error>;
    async fn create_new_pokemon_in_user_pokedex(
        &self,
        pokemon_id: &str,
        authenticated_user_id: &str,
    ) -> Result<UserPokedex, Error>;
    async fn update_attempt_is_successful(&self, attempt_id: &str) -> Result<(), Error>; 
}
