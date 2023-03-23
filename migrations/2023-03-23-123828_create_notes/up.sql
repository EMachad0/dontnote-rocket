-- Your SQL goes here
CREATE TABLE notes
(
    id      SERIAL PRIMARY KEY,
    uuid    UUID         NOT NULL DEFAULT gen_random_uuid(),
    title   VARCHAR(512) NOT NULL,
    content TEXT         NOT NULL,
    user_id INTEGER      NOT NULL REFERENCES users (id) ON DELETE CASCADE
);
