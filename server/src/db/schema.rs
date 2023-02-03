// @generated automatically by Diesel CLI.

diesel::table! {
    album_artists (id) {
        id -> Integer,
        album -> Integer,
        artist -> Integer,
    }
}

diesel::table! {
    albums (id) {
        id -> Integer,
        year -> Nullable<Integer>,
        title -> Nullable<Text>,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    track_artists (id) {
        id -> Integer,
        track -> Integer,
        artist -> Integer,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        album -> Integer,
        path -> Text,
        track_number -> Nullable<Integer>,
        disc_number -> Nullable<Integer>,
        title -> Nullable<Text>,
        year -> Nullable<Integer>,
    }
}

diesel::joinable!(album_artists -> albums (album));
diesel::joinable!(album_artists -> artists (artist));
diesel::joinable!(track_artists -> artists (artist));
diesel::joinable!(track_artists -> tracks (track));
diesel::joinable!(tracks -> albums (album));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    track_artists,
    tracks,
);
