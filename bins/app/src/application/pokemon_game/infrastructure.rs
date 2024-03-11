use super::{
    contract::{PgRepositoryContract, PgServiceContract},
    data::{UserAttemptData, UserPokedexData},
};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use error::Error;
use infrastructure::{
    db::Pg,
    schema::{user_attempts, user_pokedexes, pokemons},
};
use std::sync::Arc;
use support::store::models::{
    pokemon::Pokemon, user_attempts::UserAttempt, user_pokedex::UserPokedex,
};

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    // get all pokemons from pokemons db
    async fn get_all_pokemons(&self) -> Result<Vec<Pokemon>, Error> {
        let mut connection = self.pg_pool.connection()?;
        let all_pokemons = match Pokemon::get_all_pokemons(&mut connection) {
            Ok(all_pokemons) => all_pokemons,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(all_pokemons)
    }
    // get all pokemons from user pokedex
    async fn get_all_user_pokemons(&self, user_id: &str) -> Result<Vec<UserPokedex>, Error> {
        let mut connection = self.pg_pool.connection()?;
        let all_user_pokemons = UserPokedex::get_all_by_user_id(user_id, &mut connection)?; 
        Ok(all_user_pokemons)
    }
    // get pokemon by id from pokemons db
    async fn get_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error> {
        let mut connection = self.pg_pool.connection()?;
        let pokemon = Pokemon::get_by_id(pokemon_id, &mut connection)?;
        Ok(pokemon)
    }
    // get attempt by pokemon id from user_attempts db
    async fn get_attempt_by_pokemon_id(&self, pokemon_id: &str) -> Result<UserAttempt, Error> {
        let mut connection = self.pg_pool.connection()?;
        Ok(UserAttempt::get_by_pokemon_id(pokemon_id, &mut connection)?)
    }
    // check if attempt exists, is corelated with user and if it is not success
    async fn check_attempt(
        &self,
        attempt_id: &str,
        authenticated_user_id: &str,
    ) -> Result<UserAttempt, Error> {
        let mut connection = self.pg_pool.connection()?;
        user_attempts::table
            .filter(
                user_attempts::id
                    .eq(attempt_id)
                    .and(user_attempts::user_id.eq(authenticated_user_id))
                    .and(user_attempts::is_successful.eq(false)),
            )
            .first(&mut connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("attempt not found"))
    }
    // return paginated pokemons from user pokedex
    async fn paginated(&self, user_pokemon_ids: Vec<&str>, page: i32, limit: i32) -> Result<Vec<Pokemon>, Error> {
        let mut connection = self.pg_pool.connection()?;
        pokemons::table
        .filter(pokemons::id.eq_any(user_pokemon_ids))
        .offset((page * limit) as i64) // Apply the offset
        .limit(limit as i64)   // Apply the limit
        .load::<Pokemon>(&mut connection)
        .map_err(Error::from)
    }
}

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    //create new attempt with user id and pokemon id
    async fn create_new_attempt(
        &self,
        pokemon_id: &str,
        authenticated_user_id: &str,
    ) -> Result<UserAttempt, Error> {
        let mut connection = self.pg_pool.connection()?;
        let data = UserAttemptData::new(authenticated_user_id, pokemon_id);
        diesel::insert_into(user_attempts::table)
            .values(data)
            .get_result::<UserAttempt>(&mut connection)
            .map_err(Error::from)
    }

    //create new pokemon in user pokedex
    async fn create_new_pokemon_in_user_pokedex(
        &self,
        pokemon_id: &str,
        authenticated_user_id: &str,
    ) -> Result<UserPokedex, Error> {
        let mut connection = self.pg_pool.connection()?;
        let data = UserPokedexData::new(authenticated_user_id, pokemon_id);
        diesel::insert_into(user_pokedexes::table)
            .values(data)
            .get_result::<UserPokedex>(&mut connection)
            .map_err(Error::from)
    }
    async fn update_attempt_is_successful(&self, attempt_id: &str) -> Result<(), Error> {
        // update attempt is successful
        let mut connection = self.pg_pool.connection()?;
        diesel::update(user_attempts::table)
            .filter(user_attempts::id.eq(attempt_id))
            .set(user_attempts::is_successful.eq(true))
            .execute(&mut connection)
            .map_err(Error::from)
            .map_err(|e| e.add_cause_if_not_found("Message not found"))?;

        Ok(())
    }
}
