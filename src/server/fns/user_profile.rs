use dioxus_fullstack::prelude::*;

#[cfg(feature = "server")]
use log::debug;
use server_fn::codec::PostUrl;

use crate::domain::model::Id;

#[cfg(feature = "server")]
use crate::server::session::Session;

#[server(endpoint = "save_user_profile_primary_info", input = PostUrl)]
pub async fn save_user_profile_primary_info(id: Id, username: String, email: String, bio: String) -> Result<(), ServerFnError> {
    //
    use crate::domain::model::UserAccount;

    debug!(
        "[save_user_profile_primary_info] Received: id: {}, username: {}, email: {}, bio: {}",
        id, username, email, bio
    );

    let session: Session = extract().await?;
    let mut ua = UserAccount::default();
    ua.id = id;
    ua.username = username;
    ua.email = email;
    ua.bio = bio;
    session
        .1
        .update_user_account(ua)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server(endpoint = "set_user_profile_new_password", input = PostUrl)]
pub async fn set_user_profile_new_password(
    user_id: Id,
    curr_password: String,
    new_password: String,
) -> Result<Result<(), String>, ServerFnError> {
    //
    debug!(
        "[set_user_profile_new_password] Received: user_id: {}, curr_password: {}, new_password: {}",
        user_id, curr_password, new_password
    );

    let session: Session = extract().await?;

    if let Err(err) = session.user_mgmt().update_password(&user_id, curr_password, new_password).await {
        return Ok(Err(err.to_string()));
    }

    Ok(Ok(()))
}
