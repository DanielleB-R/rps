-- Your SQL goes here
CREATE TABLE users (
    id serial PRIMARY KEY,
    name varchar UNIQUE NOT NULL,
    pronouns varchar NOT NULL,
    age integer NOT NULL,
    deleted boolean NOT NULL DEFAULT FALSE
);
