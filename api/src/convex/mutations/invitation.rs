use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;
use serde_json::Value;

#[server]
pub async fn create_invitation(server: String, member: String) -> Result<String, ServerFnError> {
    use common::state::convex;
    let mut client = convex()?;
    let expiration_time = 1.0 * 60.0 * 7.0;
    let invitation = client
        .mutation(
            "invitations:createInvitation",
            btreemap! {
                "server".into() => server.into(),
                "member".into() => member.into(),
                "expiresInMinutes".into() => expiration_time.into()
            },
        )
        .await
        .or(Err(ServerFnError::new("we can't create the invitation")))?;
    match invitation {
        convex::FunctionResult::Value(value) => {
            let res: Value = value.into();
            if let Some(invitation) = res.get("invitationCode") {
                serde_json::from_value(invitation.clone())
                    .or(Err(ServerFnError::new("i need to improve all of this")))
            } else {
                Err(ServerFnError::new("or is null or i mess it up"))
            }
        }
        _ => Err(ServerFnError::new("Convex return an error")),
    }
}

#[server]
pub async fn join_with_invitation(invitation: String, user: String) -> Result<(), ServerFnError> {
    use common::state::convex;
    let mut client = convex()?;

    let result = client
        .mutation(
            "invitations:joinServerWithInvitation",
            btreemap! {
                "invitationCode".into() => invitation.into(),
                "userId".into() => user.into()
            },
        )
        .await
        .or(Err(ServerFnError::new("we can't create the invitation")))?;
    match result {
        convex::FunctionResult::Value(value) => {
            let res: Value = value.into();
            if res.is_string() {
                Ok(())
            } else {
                Err(ServerFnError::new("or is null or i mess it up"))
            }
        }
        _ => Err(ServerFnError::new("Convex return an error")),
    }
}
