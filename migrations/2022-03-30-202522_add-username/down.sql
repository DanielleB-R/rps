-- This file should undo anything in `up.sql`
ALTER TABLE users
    DROP COLUMN username;

ALTER TABLE users
    ADD CONSTRAINT users_name_key UNIQUE (name);
