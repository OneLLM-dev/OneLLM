create table users (
    id serial primary key,
    email varchar not null unique,
    password varchar not null,
    balance integer not null default 0
);

-- API KEYS TABLE
create table api_keys (
    id serial primary key,
    user_id integer not null references users(id) on delete cascade,
    key varchar not null unique
);

create unique index email on users (email);
