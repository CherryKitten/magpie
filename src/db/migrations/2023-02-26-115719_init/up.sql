CREATE TABLE tracks
(
    id             INTEGER PRIMARY KEY NOT NULL,
    album_id       INTEGER DEFAULT NULL,
    path           TEXT    DEFAULT NULL,
    filesize       INTEGER             NOT NULL,
    track_number   INTEGER DEFAULT 1,
    disc_number    INTEGER DEFAULT 1,
    disc_title     TEXT    DEFAULT NULL,
    content_group  TEXT    DEFAULT NULL,
    title          TEXT    DEFAULT NULL,
    subtitle       TEXT    DEFAULT NULL,
    year           INTEGER DEFAULT NULL,
    release_date   TEXT    DEFAULT NULL,
    bpm            TEXT    DEFAULT NULL,
    length         INTEGER DEFAULT NULL,
    initial_key    TEXT    DEFAULT NULL,
    language       TEXT    DEFAULT NULL,
    /*TODO
    label_id     INTEGER,
    */
    original_title TEXT,
    added_at       TEXT    DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE RESTRICT,
    UNIQUE (path) ON CONFLICT REPLACE
);

CREATE TABLE albums
(
    id    INTEGER PRIMARY KEY NOT NULL,
    year  INTEGER,
    title TEXT,
    art   BLOB,
    UNIQUE (title, year)
);

CREATE TABLE artists
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT,
    UNIQUE (name)
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
)
