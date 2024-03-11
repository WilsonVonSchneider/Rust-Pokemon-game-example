CREATE TABLE users (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    "password" VARCHAR NOT NULL,
    email_verified_at TIMESTAMPTZ DEFAULT NULL,
    refresh_token VARCHAR DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

