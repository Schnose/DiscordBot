CREATE TABLE IF NOT EXISTS users (
	name VARCHAR(255) NOT NULL,
	discord_id BIGINT NOT NULL UNIQUE,
	steam_id INT,
	mode VARCHAR(255),

	PRIMARY KEY (discord_id)
);
