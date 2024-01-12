CREATE TABLE IF NOT EXISTS profiles
(
    id         INTEGER PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    first_name VARCHAR(255),
    last_name  VARCHAR(255),
    pronouns   VARCHAR(50),
    headline   VARCHAR(255),
    country    VARCHAR(255),
    city       VARCHAR(255),
    bio        TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
)
