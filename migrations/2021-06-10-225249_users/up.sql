CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    user_id uuid NOT NULL DEFAULT uuid_generate_v4(),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(32) NOT NULL UNIQUE,
    hash VARCHAR NOT NULL,
    PRIMARY KEY (user_id)
);