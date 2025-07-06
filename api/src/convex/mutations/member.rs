use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;

#[server]
pub async fn set_last_visited_channel(
    member: String,
    channel: String,
) -> Result<(), ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let mut client = convex()?;

    let auth = auth()
        .await?
        .current_user
        .ok_or(ServerFnError::new("You need to be auth"))?;

    let _resutl = client
        .mutation(
            "user:setLastVisitedChannel",
            btreemap! {
                "auth".into() => auth.user().id.into(),
                "member".into() => member.into(),
                "channel".into() => channel.into(),
                // "category".into() => category.into()
            },
        )
        .await
        .or(Err(ServerFnError::new("something go wrong")))?;
    Ok(())
}
