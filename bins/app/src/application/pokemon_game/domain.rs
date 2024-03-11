use super::{
    contract::{PgRepositoryContract, PgServiceContract, PokemonGameContract},
    data::{ResponseData, PaginatedData},
};
use async_trait::async_trait;
use error::Error;
use support::{helpers::random_pokemon_id, store::models::pokemon::Pokemon};

pub struct PokemonGame<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> PokemonGameContract for PokemonGame<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    // initialize new game
    async fn start_game(&self, authenticated_user_id: &str) -> Result<ResponseData, Error> {
        //get all pokemons from pokemon database
        let all_pokemons = self.repository.get_all_pokemons().await?;
        //get all users pokemons from user pokedex database
        let all_users_pokemons = self
            .repository
            .get_all_user_pokemons(authenticated_user_id)
            .await?;
        //return random pokemon id
        let pokemon_id = random_pokemon_id::random_pokemon_id(all_pokemons, all_users_pokemons)?;
        //return pokemon from pokemon database
        let pokemon = self.repository.get_pokemon_by_id(&pokemon_id).await?;
        // check if there is image for that pokemon
        let pokemon_image = match pokemon.image {
            Some(image) => image,
            None => {
                return Err(Error::InternalError("Something went wrong".to_string()));
            }
        };
        // if attempt already exist return it, if not create it and return it
        let attempt = match self.repository.get_attempt_by_pokemon_id(&pokemon.id).await {
            Ok(attempt) => attempt,
            Err(e) => {
                match e {
                    Error::NotFoundWithCause(_) => {
                        //create new attempt and return attempt id
                        self.service
                            .create_new_attempt(&pokemon.id, authenticated_user_id)
                            .await?
                    }
                    _ => {
                        return Err(e);
                    }
                }
            }
        };
        //return attempt id and pokemon image
        Ok(ResponseData::new(pokemon_image, attempt.id))
    }

    // check name from input with name of pokemon from attempt
    async fn check_guess(
        &self,
        authenticated_user_id: &str,
        guess: &str,
        attempt_id: &str,
    ) -> Result<(), Error> {
        // check attempt if it exists with attempt id, user id and if it is not successful
        let attempt = self.repository.check_attempt(attempt_id, authenticated_user_id).await?;
        // get pokemon by attempt.pokemon_id
        let pokemon = self
            .repository
            .get_pokemon_by_id(&attempt.pokemon_id)
            .await?;
        // check if name exists in pokemon object
        let pokemon_name = match pokemon.name {
            Some(name) => name,
            None => {
                return Err(Error::InternalError("Something went wrong".to_string()));
            }
        };
        // check guess against pokemon.name and return error if guess is not good
        if guess != pokemon_name {
            return Err(Error::WrongGuess("You guessed wrong".to_string()));
        }
        // save pokemon in user pokedex with pokemon id and authenticated user id
        self.service
            .create_new_pokemon_in_user_pokedex(&pokemon.id, authenticated_user_id)
            .await?;
        // update is successful
        self.service.update_attempt_is_successful(attempt_id).await?;
        Ok(())
    }

    // return paginated pokemons from user pokedex
    async fn paginated(&self, authenticated_user_id: &str, data: PaginatedData) -> Result<Vec<Pokemon>, Error> {
        // get all user pokemons
        let user_pokemons = self.repository.get_all_user_pokemons(authenticated_user_id).await?;
        // map into vector of pokemon_ids
        let user_pokemons_ids: Vec<&str> = user_pokemons.iter().map(|pokemon| pokemon.pokemon_id.as_str()).collect();
        // return all pokemons with matching user pokemon ids, paginated
        Ok(self.repository.paginated(user_pokemons_ids, data.page, data.limit).await?)
    }
}
