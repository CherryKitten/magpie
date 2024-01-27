diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        salt -> Text,
        email -> Nullable<Text>,
        is_admin -> Bool,
    }
}

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
        title -> Text,
        art_id -> Nullable<Integer>,
    }
}

diesel::table! {
    art (id) {
        id -> Integer,
        hash -> Double,
        data -> Binary,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        art_id -> Nullable<Integer>,
    }
}

diesel::table! {
    genres (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    record_labels (id) {
        id -> Integer,
        name -> Text,
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
    track_genres (id) {
        id -> Integer,
        track_id -> Integer,
        genre_id -> Integer,
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
        title -> Text,
        year -> Nullable<Integer>,
        release_date -> Nullable<Text>,
        length -> Nullable<Integer>,
        language -> Nullable<Text>,
        added_at -> Nullable<Text>,
        art_id -> Nullable<Integer>,
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));
diesel::joinable!(albums -> art (art_id));
diesel::joinable!(artists -> art (art_id));
diesel::joinable!(track_artists -> artists (artist_id));
diesel::joinable!(track_artists -> tracks (track_id));
diesel::joinable!(track_genres -> genres (genre_id));
diesel::joinable!(track_genres -> tracks (track_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> art (art_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    art,
    artists,
    genres,
    record_labels,
    track_artists,
    track_genres,
    tracks,
    users,
);
