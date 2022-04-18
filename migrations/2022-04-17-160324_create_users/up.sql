-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(),
    username VARCHAR(32) NOT NULL
);

CREATE TABLE secure_user_info (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    password TEXT NOT NULL
);