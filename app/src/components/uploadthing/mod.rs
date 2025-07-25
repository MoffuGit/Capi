use gloo_file::{Blob, File};
use serde::Deserialize;

pub async fn upload_file(file: &File, url: String) -> anyhow::Result<UploadResult> {
    let chunks = gloo_file::futures::read_as_bytes(&Blob::from(file.clone())).await?;

    let result = reqwest::Client::new()
        .post(url)
        .header("Content-Type", file.raw_mime_type())
        .body(chunks)
        .send()
        .await?;
    Ok(result.json::<UploadResult>().await?)
}

#[derive(Debug, Deserialize)]
pub struct UploadResult {
    #[serde(rename = "storageId")]
    pub storage_id: String,
}
