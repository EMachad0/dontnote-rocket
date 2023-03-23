CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    uuid     UUID         NOT NULL DEFAULT gen_random_uuid(),
    name     VARCHAR(255) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);