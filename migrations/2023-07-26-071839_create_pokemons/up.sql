CREATE TABLE pokemons (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    "name" varchar,
    base_experience integer,
    height integer,
    pokemon_id integer,
    is_default BOOLEAN,
    "order" integer,
    "image" varchar,
    "weight" integer,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



