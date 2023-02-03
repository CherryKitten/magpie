// @generated automatically by Diesel CLI.

diesel::table! {
    albumArtists (id) {
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
    trackArtists (id) {
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

diesel::joinable!(albumArtists -> albums (album));
diesel::joinable!(albumArtists -> artists (artist));
diesel::joinable!(trackArtists -> artists (artist));
diesel::joinable!(trackArtists -> tracks (track));
diesel::joinable!(tracks -> albums (album));

diesel::allow_tables_to_appear_in_same_query!(
    albumArtists,
    albums,
    artists,
    trackArtists,
    tracks,
);
