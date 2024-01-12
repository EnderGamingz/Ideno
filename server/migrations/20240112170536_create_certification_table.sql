CREATE TABLE IF NOT EXISTS certification
(
    id              INTEGER PRIMARY KEY,
    user_id         INTEGER      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name            VARCHAR(255) NOT NULL,
    organization    VARCHAR(255) NOT NULL,
    issue_date      TIMESTAMP,
    expiration_date TIMESTAMP,
    credential_id   VARCHAR(255),
    credential_url  VARCHAR(255),
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP
)