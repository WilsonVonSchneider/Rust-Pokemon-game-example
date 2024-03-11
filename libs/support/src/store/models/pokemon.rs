use diesel::Queryable;
use infrastructure::{db::DbConnection, schema::pokemons};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use reqwest;

use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = pokemons)]
#[diesel(treat_none_as_null = true)]
pub struct Pokemon {
    pub id: String,
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub pokemon_id: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub image: Option<String>,
    pub weight: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Pokemon {
    // get pokemon by id
    pub fn get_by_pokemon_id(
        pokemon_id: i32,
        connection: &mut DbConnection,
    ) -> Result<Pokemon, error::Error> {
        pokemons::table
            .filter(pokemons::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(error::Error::from)
            .map_err(|e| e.add_cause_if_not_found("pokemon_not_found"))
    }
    // create new pokemon
    pub fn create(
        data: NewDatabasePokemon,
        connection: &mut DbConnection,
    ) -> Result<Pokemon, error::Error> {
        diesel::insert_into(pokemons::table)
            .values(data)
            .get_result::<Pokemon>(connection)
            .map_err(error::Error::from)
    }
    //API request to get all 151 pokemons in paginated list
    pub async fn get_all_pokemons_api() -> Result<Vec<AllAPIPokemons>, error::Error> {
        //API request to get all 151 pokemons in paginated list
        let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/?limit=151")
            .await?
            .json::<PokemonAPIResponse>()
            .await?;
        Ok(response.results)
    }
    //API request to get all 151 pokemons, one by one, using urls from pokemon_all vector
    pub async fn get_pokemons_one_by_one_api(
        pokemon: AllAPIPokemons,
    ) -> Result<APIPokemon, error::Error> {
        //API request to get all 151 pokemons, one by one, using urls from pokemon_all vector
        let url = pokemon.url.unwrap();
        let pokemon = reqwest::get(url).await?.json::<APIPokemon>().await?;
        Ok(pokemon)
    }

    // get all pokemons
    pub fn get_all_pokemons(
        connection: &mut DbConnection,
    ) -> Result<Vec<Pokemon>, error::Error> {
        pokemons::table
            .load::<Pokemon>(connection)
            .map_err(error::Error::from)
            .map_err(|e| e.add_cause_if_not_found("no pokemons in database"))
    }

        // get  by id
        pub fn get_by_id(
            id: &str,
            connection: &mut DbConnection,
        ) -> Result<Pokemon, error::Error> {
            pokemons::table
                .filter(pokemons::id.eq(id))
                .first(connection)
                .map_err(error::Error::from)
                .map_err(|e| e.add_cause_if_not_found("pokemon_not_found"))
        }
}

//Structs we need for mapping the data from API
//Pokemon API is responding with Results vector containing AllPokemons. Each pokemon from AllPokemons is struct with name and url.
#[derive(Deserialize, Debug)]
pub struct PokemonAPIResponse {
    results: Vec<AllAPIPokemons>,
}
//AllPokemons struct containing  name and url
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct AllAPIPokemons {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct APIPokemon {
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub id: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub sprites: Option<APISprites>,
    pub abilities: Option<Vec<APIAbilities>>,
    pub stats: Option<Vec<APIStats>>,
    pub types: Option<Vec<APITypes>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = pokemons)]
pub struct NewDatabasePokemon {
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub pokemon_id: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub image: Option<String>,
}

impl From<APIPokemon> for NewDatabasePokemon {
    fn from(api_pokemon: APIPokemon) -> Self {
        let image = api_pokemon.sprites.unwrap().front_default;
        NewDatabasePokemon {
            name: api_pokemon.name,
            base_experience: api_pokemon.base_experience,
            height: api_pokemon.height,
            pokemon_id: api_pokemon.id,
            is_default: api_pokemon.is_default,
            order: api_pokemon.order,
            weight: api_pokemon.weight,
            image,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct APIAbilities {
    pub ability: Option<APIPokemonAbility>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct APIPokemonAbility {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct APIStats {
    pub base_stat: Option<i32>,
    pub effort: Option<i32>,
    pub stat: Option<APIPokemonStat>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct APIPokemonStat {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct APITypes {
    pub slot: Option<i32>,
    pub r#type: Option<APIPokemonType>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct APIPokemonType {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct APISprites {
    pub front_default: Option<String>,
}
