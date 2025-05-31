ALTER TABLE users
ALTER COLUMN github_id TYPE VARCHAR USING github_id::varchar;