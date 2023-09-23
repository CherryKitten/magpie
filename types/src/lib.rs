#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MagpieResponse<T: MagpieData> {
    status: MagpieStatus,
    count: Option<usize>,
    page: Option<usize>,
    data: Vec<T>,
}

impl<T: MagpieData> MagpieResponse<T> {
    pub fn status(self) -> MagpieStatus {
        self.status
    }
    pub fn count(self) -> Option<usize> {
        self.count
    }
    pub fn page(self) -> Option<usize> {
        self.page
    }
    pub fn data(self) -> Vec<T> {
        self.data
    }
    pub fn set_status(mut self, status: impl Into<MagpieStatus>) -> Self {
        self.status = status.into();
        self
    }
    pub fn set_data(mut self, data: Vec<T>) -> Self {
        self.data = data;
        self.count = Some(self.data.len());
        self
    }
    pub fn append_data(mut self, data: &mut Vec<T>) -> Self {
        self.data.append(data);
        self.count = Some(self.data.len());
        self
    }
    pub fn push_data(mut self, data: T) -> Self {
        self.data.push(data);
        self.count = Some(self.data.len());
        self
    }
    pub fn set_page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }
}

pub trait MagpieData {}

#[derive(
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "PascalCase")]
pub enum MagpieStatus {
    Ok,
    Error,
    Unimplemented,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    id: usize,
    name: String,
    albums: Vec<SimpleAlbum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SimpleArtist {
    id: usize,
    name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Album {
    id: usize,
    title: String,
    year: usize,
    artist: Vec<SimpleArtist>,
    tracks: Vec<SimpleTrack>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SimpleAlbum {
    id: usize,
    title: String,
    year: usize,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    id: usize,
    title: String,
    year: usize,
    artist: Vec<SimpleArtist>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SimpleTrack {
    id: usize,
    title: String,
}

impl MagpieData for Artist {}
impl MagpieData for Album {}
impl MagpieData for Track {}
impl MagpieData for SimpleTrack {}
impl MagpieData for SimpleAlbum {}
impl MagpieData for SimpleArtist {}
