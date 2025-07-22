use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server;
use maplit::btreemap;

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
//             "server:create",
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
//             "channel:create",
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
