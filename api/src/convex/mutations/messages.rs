use futures::TryStreamExt;
use serde_json::Value;
use std::str::FromStr as _;

use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;
use server_fn::Bytes;
use server_fn::codec::{MultipartData, MultipartFormData};
use uploadthing::{FileData, FileType, UploadthingFile};

#[server]
pub async fn send_message(
    // _server: String,
    channel: String,
    message: String,
    member: String,
    // _msg_reference: Option<String>,
) -> Result<String, ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let _ = auth()
        .await?
        .current_user
        .ok_or(ServerFnError::new("You need to be auth"))?;
    let mut client = convex()?;
    let result = client
        .mutation(
            "messages:createMessage",
            btreemap! {
                "channelId".into() => channel.into(),
                "senderId".into() => member.into(),
                "content".into() => message.into()
            },
        )
        .await
        .or(Err(ServerFnError::new("we can't create the invitation")))?;
    match result {
        convex::FunctionResult::Value(value) => {
            let res: Value = value.into();
            if res.is_string() {
                Ok(serde_json::from_value(res).or(Err(ServerFnError::new("asl;kdasl;fkj")))?)
            } else {
                Err(ServerFnError::new("some erorr asfdjkasklf"))
            }
        }
        convex::FunctionResult::ErrorMessage(_) => Err(ServerFnError::new("some error")),
        convex::FunctionResult::ConvexError(convex_error) => {
            Err(ServerFnError::new("anothe error"))
        }
    }
}

#[server(name = SendMessageAttachments, prefix = "/api", input = MultipartFormData)]
pub async fn send_message_attachments(data: MultipartData) -> Result<(), ServerFnError> {
    use auth::auth;
    use common::state::convex;
    use common::state::uploadthing;
    use uploadthing::server::upload_file::UploadFileResponse;
    let _ = auth()
        .await?
        .current_user
        .ok_or(ServerFnError::new("You need to be auth"))?;
    let mut data = data.into_inner().unwrap();
    let mut message_id: Option<String> = None;
    let mut channel_id: Option<String> = None;
    let mut files: Vec<UploadthingFile> = vec![];
    while let Ok(Some(mut field)) = data.next_field().await {
        match field.name().unwrap_or_default() {
            "message_id" => {
                if let Ok(Some(chunk)) = field.chunk().await
                    && let Ok(id) = String::from_utf8(chunk.to_vec())
                {
                    message_id = id.into();
                }
            }
            "channel_id" => {
                if let Ok(Some(chunk)) = field.chunk().await
                    && let Ok(id) = String::from_utf8(chunk.to_vec())
                {
                    channel_id = id.into();
                }
            }
            _ => {
                let content_type = field.content_type().expect("mime type").as_ref();
                let file_type = if let Ok(file_type) = FileType::from_str(content_type) {
                    file_type
                } else {
                    continue;
                };

                let file_name = field.file_name().expect("file name").to_string();
                if file_type != FileType::Unknown {
                    let chunks = field
                        .try_collect::<Vec<Bytes>>()
                        .await
                        .or(Err(ServerFnError::new("Something go wrong in our servers")))?
                        .concat();
                    files.push(UploadthingFile {
                        data: FileData {
                            name: file_name,
                            file_type: file_type.to_string(),
                            size: chunks.len(),
                        },
                        chunks,
                    });
                }
            }
        }
    }
    let message_id =
        message_id.ok_or_else(|| ServerFnError::new("Something go wrong in our servers"))?;
    // let channel_id =
    //     channel_id.ok_or_else(|| ServerFnError::new("Something go wrong in our servers"))?;
    let uploadthing = uploadthing()?;
    let mut client = convex()?;

    for file in files {
        if file.data.size != 0 {
            let file_type = file.data.file_type.clone();
            if let Ok(UploadFileResponse { url, name, .. }) =
                uploadthing.upload_file(file.chunks, file.data, true).await
            {
                let _ = client
                    .mutation(
                        "messages:addAttachmentToMessage",
                        btreemap! {
                            "messageId".into() => message_id.clone().into(),
                            "name".into() => name.into(),
                            "type".into() => file_type.into(),
                            "url".into() => url.into()
                        },
                    )
                    .await;
            }
        }
    }

    Ok(())
}
