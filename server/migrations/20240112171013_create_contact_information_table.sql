CREATE TABLE IF NOT EXISTS contact_information
(
    id         INTEGER PRIMARY KEY,
    user_id    INTEGER      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    type       VARCHAR(255) NOT NULL,
    value      TEXT         NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
)