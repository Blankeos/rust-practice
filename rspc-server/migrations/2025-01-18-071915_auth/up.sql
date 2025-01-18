-- Your SQL goes here
CREATE TABLE user (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT UNIQUE,
    hashed_password TEXT NOT NULL
);
