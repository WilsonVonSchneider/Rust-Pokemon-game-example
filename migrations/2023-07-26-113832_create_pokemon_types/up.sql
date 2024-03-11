CREATE TABLE pokemon_types (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    pokemon_id varchar(36) NOT NULL,
    "name" varchar,
    slot integer,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_pokemon_abilities_user_id
        FOREIGN KEY (pokemon_id)
        REFERENCES pokemons (id)
        ON DELETE CASCADE
);

