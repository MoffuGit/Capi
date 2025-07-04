use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;

#[server]
pub async fn create_server(name: String) -> Result<(), ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let mut client = convex()?;
    let user = auth()
        .await?
        .current_user
        .ok_or(ServerFnError::new("You need to be auth"))?;
    let resutl = client
        .mutation(
            "server:create",
            btreemap! {
                "auth".into() => user.user().id.into(),
                "name".into() => name.into(),
            },
        )
        .await
        .or(Err(ServerFnError::new("something go wrong")))?;
    log!("{resutl:?}");
    Ok(())
}
