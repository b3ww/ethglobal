-- 001_create_checkpoint.sql

CREATE TABLE IF NOT EXISTS checkpoint (
    id SERIAL PRIMARY KEY,
    value NUMERIC(20, 0) NOT NULL
);
