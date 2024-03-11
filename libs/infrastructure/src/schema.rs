// @generated automatically by Diesel CLI.

diesel::table! {
    action_tokens (id) {
        #[max_length = 36]
        id -> Varchar,
        entity_id -> Varchar,
        token -> Varchar,
        action_name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_at -> Timestamptz,
        executed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    pokemon_abilities (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        pokemon_id -> Varchar,
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pokemon_stats (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        pokemon_id -> Varchar,
        name -> Nullable<Varchar>,
        base -> Nullable<Int4>,
        effort -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pokemon_types (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        pokemon_id -> Varchar,
        name -> Nullable<Varchar>,
        slot -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pokemons (id) {
        #[max_length = 36]
        id -> Varchar,
        name -> Nullable<Varchar>,
        base_experience -> Nullable<Int4>,
        height -> Nullable<Int4>,
        pokemon_id -> Nullable<Int4>,
        is_default -> Nullable<Bool>,
        order -> Nullable<Int4>,
        image -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_attempts (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 36]
        pokemon_id -> Varchar,
        is_successful -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_pokedexes (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 36]
        pokemon_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        email_verified_at -> Nullable<Timestamptz>,
        refresh_token -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(action_tokens -> users (entity_id));
diesel::joinable!(user_attempts -> pokemons (pokemon_id));
diesel::joinable!(user_attempts -> users (user_id));
diesel::joinable!(user_pokedexes -> pokemons (pokemon_id));
diesel::joinable!(user_pokedexes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    action_tokens,
    pokemon_abilities,
    pokemon_stats,
    pokemon_types,
    pokemons,
    user_attempts,
    user_pokedexes,
    users,
);
