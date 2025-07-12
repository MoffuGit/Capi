use gloo_file::futures::read_as_bytes;
use gloo_file::{Blob, File, FileReadError};
use leptos::task::spawn_local_scoped_with_cancellation;
use uploadthing::{FileData, UploadthingFile};

pub mod input;

fn read_file<F>(file: File, callback: F)
where
    F: FnOnce(Result<UploadthingFile, FileReadError>) + 'static,
{
    let name = file.name();
    let file_type = file.raw_mime_type();
    let size = file.size() as usize;
    spawn_local_scoped_with_cancellation(async move {
        callback(
            read_as_bytes(&Blob::from(file))
                .await
                .map(|chunks| UploadthingFile {
                    data: FileData {
                        name,
                        file_type,
                        size,
                    },
                    chunks,
                }),
        )
    });
}
