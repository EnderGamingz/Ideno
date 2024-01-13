CREATE TABLE IF NOT EXISTS educations
(
    id         INTEGER PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    school     TEXT NOT NULL,
    degree     TEXT,
    field      TEXT,
    start_date TIMESTAMP,
    end_date   TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)