#[cfg(feature = "ssr")]
pub mod server;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileData {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadthingFile {
    pub data: FileData,
    pub chunks: Vec<u8>,
}

#[derive(Debug, Display, EnumString, PartialEq, Default)]
pub enum FileType {
    #[strum(serialize = "image/jpeg", serialize = "image/jpg")]
    Jpeg,
    #[strum(serialize = "image/png")]
    Png,
    #[strum(serialize = "image/gif")]
    Gif,
    #[strum(serialize = "image/webp")]
    Webp,
    #[strum(serialize = "application/pdf")]
    Pdf,
    #[strum(serialize = "text/plain")]
    Text,
    #[strum(serialize = "application/msword")]
    Doc,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    Docx,
    #[strum(serialize = "application/vnd.ms-excel")]
    Xls,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    Xlsx,
    #[strum(serialize = "application/zip")]
    Zip,
    #[strum(serialize = "audio/mpeg")]
    Mp3,
    #[strum(serialize = "audio/wav")]
    Wav,
    #[strum(serialize = "video/mp4")]
    Mp4,
    #[strum(serialize = "video/webm")]
    Webm,
    #[strum(serialize = "application/json")]
    Json,
    #[strum(serialize = "text/csv")]
    Csv,
    #[strum(serialize = "text/html")]
    Html,
    #[default]
    Unknown,
}
