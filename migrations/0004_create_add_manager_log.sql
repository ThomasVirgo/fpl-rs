CREATE TABLE IF NOT EXISTS add_manager_logs (
id SERIAL PRIMARY KEY NOT NULL,
start_idx INTEGER NOT NULL,
created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);