create table users (
    email varchar not null,
    password varchar not null,
    apikey varchar not null,
    balance varchar not null
);

create unique index email on users (email);

create unique index apikeys on users (apikey);
