CREATE TABLE IF NOT EXISTS users
(
    id         INTEGER PRIMARY KEY,
    username   TEXT UNIQUE NOT NULL,
    email      TEXT UNIQUE NOT NULL,
    password   TEXT        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);