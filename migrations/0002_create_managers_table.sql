CREATE TABLE IF NOT EXISTS managers (
    manager_id INTEGER PRIMARY KEY,
    player_name VARCHAR NOT NULL,
    entry_name VARCHAR NOT NULL
);

CREATE INDEX idx_player_name ON managers (player_name);