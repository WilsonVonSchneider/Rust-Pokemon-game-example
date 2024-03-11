CREATE TABLE action_tokens (
    id varchar(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    entity_id VARCHAR NOT NULL,
    token VARCHAR NOT NULL,
    action_name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + interval '15 minutes'),
    executed_at TIMESTAMPTZ DEFAULT NULL,
    -- Add foreign key constraint
    CONSTRAINT fk_action_tokens_entity_id
        FOREIGN KEY (entity_id)
        REFERENCES users (id)
        ON DELETE CASCADE
);


