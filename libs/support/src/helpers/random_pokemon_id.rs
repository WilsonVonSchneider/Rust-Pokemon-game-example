use error::Error;
use rand::Rng;

use crate::store::models::{pokemon::Pokemon, user_pokedex::UserPokedex};

// take vector of all pokemons and all user pokemons (from pokedex), make exclusion vector (all pokemons minus user pokemons) and return random pokemon id
pub fn random_pokemon_id(
    all_pokemons: Vec<Pokemon>,
    all_user_pokemons: Vec<UserPokedex>,
) -> Result<String, Error> {
    // convert vectors to vector that contains only ids
    let source= all_pokemons.iter().map(|pokemon| &pokemon.id).collect();
    let exclude = all_user_pokemons.iter().map(|user_pokemon| &user_pokemon.pokemon_id).collect();
    // get all pokemons that are already not guessed
    let pokemons_not_collected = exclude_vector(source, exclude);
    // get random pokemon id
    if let Some(random_member) = get_random_member(&pokemons_not_collected) {
        Ok(random_member.to_string())
    } else {
        Err(Error::InternalError("Something went wrong".to_string()))
    }
}

// exclude source vector from vector to get vector of pokemons that are not in user pokedex
fn exclude_vector(source: Vec<&String>, exclude: Vec<&String>) -> Vec<String> {
    source
        .into_iter()
        .filter(|item| !exclude.iter().any(|excluded| *excluded == *item))
        .cloned()
        .collect()
}
// get random member of an vector
fn get_random_member(vector: &Vec<String>) -> Option<&String> {
    if vector.is_empty() {
        return None;
    }
    let index = rand::thread_rng().gen_range(0..vector.len());
    Some(&vector[index])
}
