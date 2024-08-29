-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id text PRIMARY KEY,
    name text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);
