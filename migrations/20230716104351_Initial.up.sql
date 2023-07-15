create table tracks
(
    id           int primary key not null,
    path         varchar(512)    not null,
    title        varchar(512)    not null,
    subtitle     varchar(512),
    disc_number  int             not null default 1,
    disc_title   varchar(512),
    track_number int             not null default 1,
    year         int,
    release_date date,
    bpm          int,
    language     varchar(8),
    lyrics       text
);

create table albums
(
    id    int primary key not null,
    year  int,
    title varchar(512)    not null
);

create table artists
(
    id   int primary key not null,
    name varchar(512)    not null
);

create table genres
(
    id   int primary key,
    name varchar(32)
);

create table track_artists
(
    track  int references tracks (id),
    artist int references artists (id)
);

create table album_artists
(
    album  int references albums (id),
    artist int references artists (id)
);

create table track_genres
(
    track int references tracks (id),
    genre int references genres (id)
);

alter table tracks
    add column album int references albums (id);
