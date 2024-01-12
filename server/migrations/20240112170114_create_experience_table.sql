CREATE TABLE IF NOT EXISTS experiences
(
    id          INTEGER PRIMARY KEY,
    user_id     INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    company     TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    start_date  TEXT,
    end_date    TEXT,
    type        TEXT,
    description TEXT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP
)