CREATE TABLE IF NOT EXISTS page_logs (
id SERIAL PRIMARY KEY NOT NULL,
page INTEGER NOT NULL,
created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);