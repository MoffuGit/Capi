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

pub fn read_file<F>(file: File, callback: F)
where
    F: FnOnce(anyhow::Result<ClientFile>) + 'static,
{
    let name = file.name();
    let content_type = FileType::from_str(&file.raw_mime_type()).unwrap_or_default();
    let size = file.size() as usize;
    let file_blob: Blob = file.into(); // Convert gloo_file::File to gloo_file::Blob
    let file_blob_clone = file_blob.clone(); // Clone for creating object URL

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
