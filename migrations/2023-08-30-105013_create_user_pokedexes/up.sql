CREATE TABLE user_pokedexes (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    user_id varchar(36) NOT NULL,
    pokemon_id varchar(36) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Add foreign key constraint
    CONSTRAINT fk_user_pokedexes_user_id
        FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE,
    -- Add foreign key constraint
    CONSTRAINT fk_user_pokedexes_pokemon_id
        FOREIGN KEY (pokemon_id)
        REFERENCES pokemons (id)
        ON DELETE CASCADE
);
