ALTER TABLE users
ALTER COLUMN github_id TYPE BIGINT USING github_id::bigint;