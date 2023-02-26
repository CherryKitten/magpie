// @generated automatically by Diesel CLI.

diesel::table! {
    album_artists (id) {
        id -> Integer,
        album_id -> Integer,
        artist_id -> Integer,
    }
}

diesel::table! {
    albums (id) {
        id -> Integer,
        year -> Nullable<Integer>,
        title -> Nullable<Text>,
        art -> Nullable<Binary>,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    track_artists (id) {
        id -> Integer,
        track_id -> Nullable<Integer>,
        artist_id -> Nullable<Integer>,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        album_id -> Nullable<Integer>,
        path -> Nullable<Text>,
        filesize -> Integer,
        track_number -> Nullable<Integer>,
        disc_number -> Nullable<Integer>,
        title -> Nullable<Text>,
        year -> Nullable<Integer>,
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));
diesel::joinable!(track_artists -> artists (artist_id));
diesel::joinable!(track_artists -> tracks (track_id));
diesel::joinable!(tracks -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    track_artists,
    tracks,
);
