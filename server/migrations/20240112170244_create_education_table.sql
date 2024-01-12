CREATE TABLE IF NOT EXISTS educations
(
    id         INTEGER PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    school     TEXT NOT NULL,
    degree     TEXT,
    field      TEXT,
    start_date TEXT,
    end_date   TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
)