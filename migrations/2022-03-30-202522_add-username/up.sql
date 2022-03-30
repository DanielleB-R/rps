-- Your SQL goes here
ALTER TABLE users
    ADD COLUMN username VARCHAR UNIQUE NOT NULL;

ALTER TABLE users
    DROP CONSTRAINT users_name_key;
