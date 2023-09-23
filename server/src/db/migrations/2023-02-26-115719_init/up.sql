CREATE TABLE tracks
(
    id                 INTEGER PRIMARY KEY NOT NULL,
    album_id           INTEGER,
    path               TEXT,
    filesize           INTEGER             NOT NULL,
    track_number       INTEGER DEFAULT 1,
    disc_number        INTEGER DEFAULT 1,
    disc_title         TEXT,
    content_group      TEXT,
    title              TEXT                NOT NULL,
    subtitle           TEXT,
    year               INTEGER,
    release_date       TEXT,
    bpm                TEXT,
    length             INTEGER,
    initial_key        TEXT,
    language           TEXT,
    label_id           INTEGER,
    original_title     TEXT,
    added_at           TEXT    DEFAULT CURRENT_TIMESTAMP,
    art_id             INTEGER,
    fallback_artist_id INTEGER,
    FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE RESTRICT,
    FOREIGN KEY (label_id) REFERENCES record_labels (id) ON DELETE SET NULL,
    FOREIGN KEY (fallback_artist_id) REFERENCES artists (id) ON DELETE SET NULL,
    FOREIGN KEY (art_id) REFERENCES art (id) ON DELETE SET NULL,
    UNIQUE (path) ON CONFLICT REPLACE
);

CREATE TABLE albums
(
    id     INTEGER PRIMARY KEY NOT NULL,
    year   INTEGER,
    title  TEXT                NOT NULL,
    art_id INTEGER,
    FOREIGN KEY (art_id) REFERENCES art (id) ON DELETE SET NULL,
    UNIQUE (title, year)
);

CREATE TABLE artists
(
    id     INTEGER PRIMARY KEY NOT NULL,
    name   TEXT                NOT NULL,
    art_id INTEGER,
    FOREIGN KEY (art_id) REFERENCES art (id) ON DELETE SET NULL,
    UNIQUE (name)
);

CREATE TABLE art
(
    id   INTEGER PRIMARY KEY NOT NULL,
    hash DOUBLE              NOT NULL,
    data BLOB                NOT NULL
);

CREATE TABLE track_artists
(
    id        INTEGER PRIMARY KEY NOT NULL,
    track_id  INTEGER,
    artist_id INTEGER,
    FOREIGN KEY (track_id) REFERENCES tracks (id),
    FOREIGN KEY (artist_id) REFERENCES artists (id),
    UNIQUE (track_id, artist_id)
);

CREATE TABLE album_artists
(
    id        INTEGER PRIMARY KEY NOT NULL,
    album_id  INTEGER             NOT NULL,
    artist_id INTEGER             NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums (id),
    FOREIGN KEY (artist_id) REFERENCES artists (id),
    UNIQUE (album_id, artist_id)
);

CREATE TABLE genres
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL,
    UNIQUE (name)
);

CREATE TABLE track_genres
(
    id       INTEGER PRIMARY KEY NOT NULL,
    track_id INTEGER             NOT NULL,
    genre_id INTEGER             NOT NULL,
    FOREIGN KEY (track_id) REFERENCES tracks (id),
    FOREIGN KEY (genre_id) REFERENCES genres (id),
    UNIQUE (track_id, genre_id)
);

CREATE TABLE record_labels
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL,
    UNIQUE (name)
);

CREATE TABLE users
(
    id       INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    password TEXT                NOT NULL,
    salt TEXT NOT NULL,
    email    TEXT,
    role     TEXT                NOT NULL DEFAULT 'user',
    UNIQUE (username)
);
