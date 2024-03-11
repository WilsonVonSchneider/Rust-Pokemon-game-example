#[allow(unused_imports)]
use crate::services::seeder::run;
#[allow(unused_imports)]
use crate::store::models::{pokemon::Pokemon, pokemon_ability::PokemonAbility, pokemon_stat::PokemonStat, pokemon_type::PokemonType};
#[allow(unused_imports)]
use infrastructure::db::Pg;
#[allow(unused_imports)]
use std::sync::Arc;

#[tokio::test]
async fn test_seed() {
    //initializing config, if IS_TEST is set to true test db will be used
    config::initialize().await;

    if &config::get_default("IS_TEST", "")[..] == "true" {
        let pg = Arc::new(Pg::new());
        //runing seeder
        let _result = run(pg.clone()).await;
        //checking if data was seeded by seraching for certian data in test DB
        let mut connection = pg.connection().unwrap();
        //pokemons assertion
        let first_pokemon = Pokemon::get_by_pokemon_id(1, &mut connection).unwrap();
        let last_pokemon = Pokemon::get_by_pokemon_id(151, &mut connection).unwrap();
        assert_eq!(first_pokemon.name.unwrap(), "bulbasaur");
        assert_eq!(last_pokemon.name.unwrap(), "mew");
        //abilities assertion
        let first_pokemon_ability = PokemonAbility::get_by_pokemon_id_and_name(first_pokemon.id.clone(), Some("overgrow".to_string()), &mut connection);
        let last_pokemon_ability = PokemonAbility::get_by_pokemon_id_and_name(last_pokemon.id.clone(), Some("synchronize".to_string()), &mut connection);
        assert_eq!(first_pokemon_ability.unwrap().name, Some("overgrow".to_string()));
        assert_eq!(last_pokemon_ability.unwrap().name, Some("synchronize".to_string()));
        //stats assertion
        let first_pokemon_stat = PokemonStat::get_by_pokemon_id_and_name(first_pokemon.id.clone(), Some("hp".to_string()), &mut connection);
        let last_pokemon_stat = PokemonStat::get_by_pokemon_id_and_name(last_pokemon.id.clone(), Some("speed".to_string()), &mut connection);
        assert_eq!(first_pokemon_stat.unwrap().name, Some("hp".to_string()));
        assert_eq!(last_pokemon_stat.unwrap().name, Some("speed".to_string()));
        //types assertion
        let first_pokemon_type = PokemonType::get_by_pokemon_id_and_name(first_pokemon.id.clone(), Some("grass".to_string()), &mut connection);
        let last_pokemon_type = PokemonType::get_by_pokemon_id_and_name(last_pokemon.id.clone(), Some("psychic".to_string()), &mut connection);
        assert_eq!(first_pokemon_type.unwrap().name, Some("grass".to_string()));
        assert_eq!(last_pokemon_type.unwrap().name, Some("psychic".to_string()));
    } else {
        assert!(
            !true,
            "IS_TEST variable must be true in order to test seeder"
        );
    }
}
