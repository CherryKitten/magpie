CREATE TABLE tracks
(
    id           INTEGER PRIMARY KEY NOT NULL,
    album        INTEGER             NOT NULL,
    path         TEXT                NOT NULL,
    track_number INTEGER,
    disc_number  INTEGER,
    title        TEXT,
    year         INTEGER,
    FOREIGN KEY (album) REFERENCES albums (id) ON DELETE RESTRICT,
    UNIQUE (path) ON CONFLICT REPLACE
);

CREATE TABLE albums
(
    id    INTEGER PRIMARY KEY NOT NULL,
    year  INTEGER,
    title TEXT,
    UNIQUE (title)
);

CREATE TABLE artists
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL,
    UNIQUE (name)
);

CREATE TABLE track_artists
(
    id     INTEGER PRIMARY KEY NOT NULL,
    track  INTEGER             NOT NULL,
    artist INTEGER             NOT NULL,
    FOREIGN KEY (track) REFERENCES tracks (id),
    FOREIGN KEY (artist) REFERENCES artists (id)
);

CREATE TABLE album_artists
(
    id     INTEGER PRIMARY KEY NOT NULL,
    album  INTEGER             NOT NULL,
    artist INTEGER             NOT NULL,
    FOREIGN KEY (album) REFERENCES albums (id),
    FOREIGN KEY (artist) REFERENCES artists (id)
)
