create table users (
    id serial primary key,
    email varchar not null unique,
    password varchar not null,
    balance integer not null default 0,
    verified boolean not null default false
);

-- API KEYS TABLE
CREATE TABLE api_keys (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key VARCHAR NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    CONSTRAINT unique_user_api_key_name UNIQUE (user_id, name)
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL
);
