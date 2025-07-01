use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: -1,
            email: "example@example.com".into(),
            permissions,
        }
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::collections::HashSet;

    use sqlx::PgPool;

    use super::User;

    #[derive(sqlx::FromRow, Clone, Debug)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlCsrfToken {
        pub csrf_token: String,
        pub pkce_token: String,
    }

    #[derive(sqlx::FromRow, Clone, Debug)]
    pub struct SqlUser {
        pub id: i64,
        pub email: String,
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlRefreshToken {
        pub secret: String,
    }

    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
                email: self.email,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }
    impl User {
        pub async fn get(id: i64, pool: &PgPool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            // let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            //     "SELECT token FROM user_permissions WHERE user_id = $1;",
            // )
            // .bind(id)
            // .fetch_all(pool)
            // .await
            // .ok()?;

            Some(sqluser.into_user(Some(vec![])))
        }

        pub async fn get_from_email(email: &str, pool: &PgPool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = $1;",
            )
            .bind(sqluser.id)
            .fetch_all(pool)
            .await
            .ok()
            .unwrap_or_default();

            Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }
}
