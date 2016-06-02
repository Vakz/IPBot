A means to learn Rust, while testing the Discord API. Project has no other
specific usage, and will probably never be useful to anyone else.

Assumes an sqlite3 database is available as "db.sqlite", with currently a single
table, Files:

CREATE TABLE files(
  fileid Integer PRIMARY KEY,
  name varchar(40) UNIQUE,
  dest varchar(200),
  user varchar(20),
  inserted DATETIME DEFAULT CURRENT_TIMESTAMP
);


# Depends on:

- libsodium-dev
- libsqlite3-dev
