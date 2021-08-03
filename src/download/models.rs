use serde::{Deserialize};

#[derive(Deserialize)]
pub struct NhentaiDoujin {
    // id: usize,
    pub media_id: String,
    pub title: NhentaiTitle,
    pub images: NhentaiImages,
    pub num_pages: u64,
}

#[derive(Deserialize)]
pub struct NhentaiTitle {
    pub english: String,
    pub japanese: String,
    pub pretty: String,
}

#[derive(Deserialize)]
pub struct NhentaiImages {
    pub pages: Vec<NhentaiImage>,
    pub cover: NhentaiImage,
    pub thumbnail: NhentaiImage,
}

#[derive(Deserialize)]
pub struct NhentaiImage {
    pub t: String,
    pub w: usize,
    pub h: usize,
}