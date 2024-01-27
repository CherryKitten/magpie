CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  salt TEXT NOT NULL,
  email TEXT,
  is_admin BOOL NOT NULL DEFAULT false,
  UNIQUE (username)
);
