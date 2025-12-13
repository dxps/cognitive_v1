use crate::domain::model::{Id, UserAccount};

use crate::server::UsersRepo;
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use sqlx::PgPool;

#[async_trait]
impl Authentication<UserAccount, Id, PgPool> for UserAccount {
    async fn load_user(user_id: Id, pool: Option<&PgPool>) -> Result<UserAccount, anyhow::Error> {
        let pool = pool.unwrap();
        UsersRepo::get_by_id(&user_id, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.is_anonymous
    }

    fn is_active(&self) -> bool {
        !self.is_anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for UserAccount {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(&perm.to_string())
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSession layer was not found!")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (http::status::StatusCode::INTERNAL_SERVER_ERROR, "AuthSession layer was not found!").into_response()
    }
}
