CREATE TABLE "user"
(
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
  email VARCHAR(100) NOT NULL UNIQUE,
  username VARCHAR(100) NOT NULL,
  password BYTEA NOT NULL
);

ALTER TABLE item
ADD user_id INTEGER NOT NULL REFERENCES "user"(id);