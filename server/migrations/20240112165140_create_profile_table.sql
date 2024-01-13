CREATE TABLE IF NOT EXISTS profiles
(
    user_id    INTEGER PRIMARY KEY NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    first_name VARCHAR(255),
    last_name  VARCHAR(255),
    pronouns   VARCHAR(50),
    headline   VARCHAR(255),
    country    VARCHAR(255),
    city       VARCHAR(255),
    bio        TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
