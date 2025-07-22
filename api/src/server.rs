use common::convex::{Member, Server};
use convex_client::leptos::Query;
use leptos::server;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SideBarData {
    pub server: Server,
    pub member: Member,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetServers {
    pub auth: i64,
}

impl Query<Vec<SideBarData>> for GetServers {
    fn name(&self) -> String {
        "user:getServers".into()
    }
}

#[server]
pub async fn preload_server_data() -> Result<Vec<SideBarData>, ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let auth = auth().await?;
    let mut client = convex()?;
    if let Some(user) = auth.current_user {
        client
            .query(GetServers {
                auth: user.user().id,
            })
            .await
            .map_err(|err| ServerFnError::new(format!("{err}")))
    } else {
        Ok(vec![])
    }
}

// #[server]
// pub async fn create_server(name: String) -> Result<(), ServerFnError> {
//     use auth::auth;
//     use common::state::convex;
//     let mut client = convex()?;
//     let user = auth()
//         .await?
//         .current_user
//         .ok_or(ServerFnError::new("You need to be auth"))?;
//     let resutl = client
//         .mutation(
//,
//             btreemap! {
//                 "auth".into() => user.user().id.into(),
//                 "name".into() => name.into(),
//             },
//         )
//         .await
//         .or(Err(ServerFnError::new("something go wrong")))?;
//     log!("{resutl:?}");
//     Ok(())
// }
//
// #[server]
// pub async fn create_category(name: String, server: String) -> Result<(), ServerFnError> {
//     use auth::auth;
//     use common::state::convex;
//     let mut client = convex()?;
//     let auth = auth()
//         .await?
//         .current_user
//         .ok_or(ServerFnError::new("You need to be auth"))?;
//     let resutl = client
//         .mutation(
//             "category:create",
//             btreemap! {
//                 "auth".into() => auth.user().id.into(),
//                 "server".into() => server.into(),
//                 "name".into() => name.into(),
//             },
//         )
//         .await
//         .or(Err(ServerFnError::new("something go wrong")))?;
//     log!("{resutl:?}");
//     Ok(())
// }
//
// #[server]
// pub async fn create_channel(
//     name: String,
//     server: String,
//     category: Option<String>,
// ) -> Result<(), ServerFnError> {
//     use auth::auth;
//     use common::state::convex;
//     let mut client = convex()?;
//     let auth = auth()
//         .await?
//         .current_user
//         .ok_or(ServerFnError::new("You need to be auth"))?;
//     let resutl = client
//         .mutation(
//,
//             btreemap! {
//                 "auth".into() => auth.user().id.into(),
//                 "server".into() => server.into(),
//                 "name".into() => name.into(),
//                 // "category".into() => category.into()
//             },
//         )
//         .await
//         .or(Err(ServerFnError::new("something go wrong")))?;
//     log!("{resutl:?}");
//     Ok(())
// }
