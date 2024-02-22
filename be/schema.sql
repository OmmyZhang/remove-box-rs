DROP TABLE IF EXISTS records;

CREATE TABLE records (
  id serial PRIMARY KEY,
  name varchar(50) NOT NULL UNIQUE,
  score INTEGER NOT NULL,
  time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_score_time ON records (score, time);
