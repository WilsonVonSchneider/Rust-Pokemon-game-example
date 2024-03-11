CREATE TABLE pokemon_stats (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    pokemon_id varchar(36) NOT NULL,
    "name" varchar,
    base integer,
    effort integer,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_pokemon_abilities_user_id
        FOREIGN KEY (pokemon_id)
        REFERENCES pokemons (id)
        ON DELETE CASCADE
);
