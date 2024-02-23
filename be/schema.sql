CREATE TABLE IF NOT EXISTS records (
  id serial PRIMARY KEY,
  name varchar(50) NOT NULL UNIQUE,
  score INTEGER NOT NULL,
  time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_score_time ON records (score, time);
