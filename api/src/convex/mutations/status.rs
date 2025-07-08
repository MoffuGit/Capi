use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;

#[server]
pub async fn heart_beat(user: String, session: String) -> Result<(), ServerFnError> {
    use common::state::convex;
    let mut convex = convex()?;
    log!("{user}-{session}");
    let _ = convex
        .mutation(
            "presence:heartbeat",
            btreemap! {
                "user".into() => user.into(),
                "sessionId".into() => session.into()
            },
        )
        .await;
    Ok(())
}
