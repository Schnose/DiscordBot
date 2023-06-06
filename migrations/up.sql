CREATE TABLE IF NOT EXISTS users (
  discord_id BIGINT       NOT NULL,
  name       VARCHAR(255) NOT NULL,
  steam_id   INT,
  mode       CHAR(3),

  PRIMARY KEY (discord_id)
);

CREATE TABLE IF NOT EXISTS logs (
  id         BIGSERIAL NOT NULL PRIMARY KEY,
  level      CHAR(5)   NOT NULL,
  content    TEXT      NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- vim: et ts=2 sw=2 sts=2 ai si ft=sql
