-- Add migration script here
CREATE TABLE IF NOT EXISTS players (
id SERIAL PRIMARY KEY NOT NULL,
player_id INTEGER NOT NULL,
first_name VARCHAR NOT NULL,
second_name VARCHAR NOT NULL,
now_cost INTEGER NOT NULL,
points_per_game REAL NOT NULL,
selected_by_percent REAL NOT NULL,
element_type INTEGER NOT NULL,
photo VARCHAR NOT NULL,
team INTEGER NOT NULL,
total_points INTEGER NOT NULL,
minutes INTEGER NOT NULL,
starts INTEGER NOT NULL,
created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
