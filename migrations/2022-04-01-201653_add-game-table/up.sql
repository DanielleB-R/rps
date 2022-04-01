-- Your SQL goes here
CREATE TABLE games (
    id serial PRIMARY KEY,
    player_1 integer NOT NULL REFERENCES users (id),
    player_2 integer NOT NULL REFERENCES users (id),
    winner integer REFERENCES users (id),
    rounds integer
);
