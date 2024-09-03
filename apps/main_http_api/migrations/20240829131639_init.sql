-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id text PRIMARY KEY,
    name text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS cryptokeys (
    id text PRIMARY KEY,
    name text NOT NULL,
    payload text NOT NULL,
    user_id text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);
