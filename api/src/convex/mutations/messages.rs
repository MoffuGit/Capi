use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;

#[server]
pub async fn send_message(
    // _server: String,
    channel: String,
    message: String,
    member: String,
    // _msg_reference: Option<String>,
) -> Result<(), ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let _ = auth()
        .await?
        .current_user
        .ok_or(ServerFnError::new("You need to be auth"))?;
    let mut client = convex()?;
    client
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
    Ok(())
}
