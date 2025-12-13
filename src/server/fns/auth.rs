use crate::domain::model::UserAccount;

#[cfg(feature = "server")]
use crate::{domain::model::Id, server::Session};

#[cfg(feature = "server")]
use log::debug;

use dioxus_fullstack::prelude::*;
use server_fn::codec::{GetUrl, PostUrl};

// TODO: Use a proper result type, instead of `ServerFnError`.
// pub type LoginResult = AppResult<UserAccount>;

#[server(endpoint = "login", input = PostUrl)]
pub async fn login(email: String, password: String) -> Result<UserAccount, ServerFnError> {
    //
    let session: Session = extract().await?;
    let account = session.user_mgmt().authenticate_user(email, password).await?;
    session.login_user(account.id.clone());
    debug!("[login] Logged-in user with account: {:?}", account);
    Ok(account)
}

#[server(endpoint = "logout", input = PostUrl)]
pub async fn logout() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    session.logout_user();
    Ok(())
}

#[server(endpoint = "get_user_name", input = GetUrl)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    let name = match session.current_user() {
        Some(user) => user.username,
        None => "".to_string(),
    };
    Ok(name)
}

#[server(endpoint = "get_permissions", input = GetUrl)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    use axum_session_auth::Rights;

    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Let's check permissions only and not worry about if the user is anonymous or not.
    if !axum_session_auth::Auth::<UserAccount, Id, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
    {
        return Ok(format!(
            "User '{}' does not have permissions needed to view this page. Please login.",
            current_user.username
        ));
    }

    Ok(format!(
        "User '{}' has the needed permissions to view this page. Here are his permissions: {:?}",
        current_user.username, current_user.permissions
    ))
}

#[server(endpoint = "has_permissions", input = GetUrl)]
pub async fn has_admin_permissions() -> Result<bool, ServerFnError> {
    use axum_session_auth::Rights;

    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Let's check permissions only and not worry about if the user is anonymous or not.
    let res = !axum_session_auth::Auth::<UserAccount, Id, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(Rights::any([Rights::permission("Admin::Read"), Rights::permission("Admin::Write")]))
        .validate(&current_user, &method, None)
        .await;
    Ok(res)
}
