-- Add migration script here
CREATE TABLE IF NOT EXISTS user_
(
    username TEXT PRIMARY KEY UNIQUE NOT NULL,
    password TEXT NOT NULL
);