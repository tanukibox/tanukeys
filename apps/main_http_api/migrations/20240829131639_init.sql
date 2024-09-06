-- Add migration script here
-- CREATE DATABASE tanukeys;
-- CREATE SCHEMA kernel;

DROP TABLE IF EXISTS kernel.users;
CREATE TABLE IF NOT EXISTS kernel.users (
    id text PRIMARY KEY,
    name text NOT NULL,
    description text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);

DROP TABLE IF EXISTS kernel.cryptokeys;
CREATE TABLE IF NOT EXISTS kernel.cryptokeys (
    id text PRIMARY KEY,
    name text NOT NULL,
    payload text NOT NULL,
    user_id text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);
