use anyhow::anyhow;
use gloo_file::futures::read_as_bytes;
use gloo_file::{Blob, File};
use leptos::task::spawn_local_scoped_with_cancellation;
use std::str::FromStr;
use web_sys::Url;

use crate::convex::FileType;

#[derive(Debug, Clone)]
pub struct ClientFile {
    pub chunks: Vec<u8>,
    pub metadata: FileMetaData,
}

#[derive(Debug, Clone)]
pub struct FileMetaData {
    pub name: String,
    pub size: usize,
    pub content_type: FileType,
    pub url: String,
}

fn infer_file_type_from_name(file_name: &str) -> FileType {
    file_name
        .rsplit('.')
        .next()
        .map(|ext| match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => FileType::Jpeg,
            "png" => FileType::Png,
            "gif" => FileType::Gif,
            "webp" => FileType::Webp,
            "pdf" => FileType::Pdf,
            "txt" => FileType::Text,
            "doc" => FileType::Doc,
            "docx" => FileType::Docx,
            "xls" => FileType::Xls,
            "xlsx" => FileType::Xlsx,
            "zip" => FileType::Zip,
            "mp3" => FileType::Mp3,
            "wav" => FileType::Wav,
            "mp4" => FileType::Mp4,
            "webm" => FileType::Webm,
            "json" => FileType::Json,
            "csv" => FileType::Csv,
            "html" | "htm" => FileType::Html,
            "md" => FileType::Md,
            _ => FileType::Unknown,
        })
        .unwrap_or(FileType::Unknown)
}

pub fn read_file<F>(file: File, callback: F)
where
    F: FnOnce(anyhow::Result<ClientFile>) + 'static,
{
    let name = file.name();
    let raw_mime_type = file.raw_mime_type();

    let mut content_type = FileType::from_str(&raw_mime_type).unwrap_or_default();

    if content_type == FileType::Unknown {
        content_type = infer_file_type_from_name(&name);
    }

    let size = file.size() as usize;
    let file_blob: Blob = file.into();
    let file_blob_clone = file_blob.clone();

    spawn_local_scoped_with_cancellation(async move {
        let result: Result<ClientFile, anyhow::Error> = async {
            let url = Url::create_object_url_with_blob(&file_blob_clone.into())
                .map_err(|e| anyhow!("Failed to create object URL: {:?}", e))?;

            let chunks = read_as_bytes(&file_blob)
                .await
                .map_err(|e| anyhow!("Failed to read file bytes: {:?}", e))?;

            Ok(ClientFile {
                chunks,
                metadata: FileMetaData {
                    name,
                    size,
                    content_type,
                    url,
                },
            })
        }
        .await;

        callback(result);
    });
}
