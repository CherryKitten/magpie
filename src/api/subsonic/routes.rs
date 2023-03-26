use super::dto::*;
use crate::api::{AppState, Response};
use axum::{routing::get, Json, Router};

/// http://www.subsonic.org/pages/api.jsp
pub fn subsonic_compat_routes() -> Router<AppState> {
    Router::new()
        // System
        .route("/ping", get(ping))
        .route("/getLicense", get(get_license))
        // Browsing
        .route("/getMusicFolders", get(unimplemented))
        .route("/getIndexes", get(unimplemented))
        .route("/getMusicDirectory", get(unimplemented))
        .route("/getGenres", get(unimplemented))
        .route("/getArtists", get(unimplemented))
        .route("/getArtist", get(unimplemented))
        .route("/getAlbum", get(unimplemented))
        .route("/getSong", get(unimplemented))
        .route("/getVideos", get(unimplemented))
        .route("/getVideoInfo", get(unimplemented))
        .route("/getArtistInfo", get(unimplemented))
        .route("/getArtistInfo2", get(unimplemented))
        .route("/getAlbumInfo", get(unimplemented))
        .route("/getAlbumInfo2", get(unimplemented))
        .route("/getSimilarSongs", get(unimplemented))
        .route("/getSimilarSongs2", get(unimplemented))
        .route("/getTopSongs", get(unimplemented))
        // Album/song lists
        .route("/getAlbumList", get(unimplemented))
        .route("/getAlbumList2", get(unimplemented))
        .route("/getRandomSongs", get(unimplemented))
        .route("/getSongsByGenre", get(unimplemented))
        .route("/getNowPlaying", get(unimplemented))
        .route("/getStarred", get(unimplemented))
        .route("/getStarred2", get(unimplemented))
        // Searching
        .route("/search", get(unimplemented))
        .route("/search2", get(unimplemented))
        .route("/search3", get(unimplemented))
        // Playlists
        .route("/getPlaylists", get(unimplemented))
        .route("/getPlaylist", get(unimplemented))
        .route("/createPlaylist", get(unimplemented))
        .route("/updatePlaylist", get(unimplemented))
        .route("/deletePlaylist", get(unimplemented))
        // Media retrieval
        .route("/stream", get(unimplemented))
        .route("/download", get(unimplemented))
        .route("/hls", get(unimplemented))
        .route("/getCaptions", get(unimplemented))
        .route("/getCoverArt", get(unimplemented))
        .route("/getLyrics", get(unimplemented))
        .route("/getAvatar", get(unimplemented))
        // Media Annotation
        .route("/star", get(unimplemented))
        .route("/unstar", get(unimplemented))
        .route("/setRating", get(unimplemented))
        .route("/scrobble", get(unimplemented))
        // Sharing
        .route("/getShares", get(unimplemented))
        .route("/createShare", get(unimplemented))
        .route("/updateShare", get(unimplemented))
        .route("/deleteShare", get(unimplemented))
        // Podcasts (why???)
        .route("/getPodcasts", get(unimplemented))
        .route("/getNewestPodcasts", get(unimplemented))
        .route("/refreshPodcasts", get(unimplemented))
        .route("/createPodcastChannel", get(unimplemented))
        .route("/deletePodcastChannel", get(unimplemented))
        .route("/deletePodcastEpisode", get(unimplemented))
        .route("/downloadPodcastEpisode", get(unimplemented))
        // Jukebox
        .route("/jukeboxControl", get(unimplemented))
        // Internet Radio
        .route("/getInternetRadioStations", get(unimplemented))
        .route("/createInternetRadioStation", get(unimplemented))
        .route("/updateInternetRadioStation", get(unimplemented))
        .route("/deleteInternetRadioStation", get(unimplemented))
        // Chat
        .route("/getChatMessages", get(unimplemented))
        .route("/addChatMessage", get(unimplemented))
        // User management
        .route("/getUser", get(unimplemented))
        .route("/getUsers", get(unimplemented))
        .route("/createUser", get(unimplemented))
        .route("/updateUser", get(unimplemented))
        .route("/deleteUser", get(unimplemented))
        .route("/changePassword", get(unimplemented))
        // Bookmarks
        .route("/getBookmarks", get(unimplemented))
        .route("/createBookmark", get(unimplemented))
        .route("/deleteBookmark", get(unimplemented))
        .route("/getPlayQueue", get(unimplemented))
        .route("/savePlayQueue", get(unimplemented))
        // Media Library Scanning
        .route("/getScanStatus", get(unimplemented))
        .route("/startScan", get(unimplemented))
}

async fn unimplemented() -> Json<Response> {
    let mut res = SubsonicResponse::new();
    res.status = SubsonicStatus::Unimplemented;
    Json(Response::SubsonicResponse(res))
}

async fn ping() -> Json<Response> {
    let mut res = SubsonicResponse::new();
    res.status = SubsonicStatus::Ok;

    Json(Response::SubsonicResponse(res))
}

async fn get_license() -> Json<Response> {
    let mut res = SubsonicResponse::new();
    res.license = Some(SubsonicLicense::new());

    Json(Response::SubsonicResponse(res))
}
