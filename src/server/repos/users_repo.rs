use crate::{
    domain::model::{Id, UserAccount, UserEntry, UserPasswordSalt},
    server::{AppError, AppResult, AppUseCase},
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct UsersRepo {
    dbcp: Arc<PgPool>,
}

impl UsersRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_by_email(&self, email: &String, usecase: AppUseCase) -> AppResult<UserEntry> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, is_anonymous FROM user_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_id(id: &Id, pool: &PgPool) -> Option<UserAccount> {
        //
        let mut user_account =
            sqlx::query_as::<_, UserAccount>("SELECT id, email, username, bio, is_anyonymous FROM user_accounts WHERE id = $1")
                .bind(id.as_str())
                .fetch_one(pool)
                .await
                .ok()?;

        let mut permissions = sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
            .map(|r: PgRow| r.get("permission"))
            .fetch_all(pool)
            .await
            .ok()?;

        user_account.permissions.append(&mut permissions);
        Some(user_account)
    }

    pub async fn get_password_by_id(&self, user_id: &Id) -> AppResult<UserPasswordSalt> {
        //
        sqlx::query_as::<_, UserPasswordSalt>("SELECT password, salt FROM user_accounts WHERE id = $1")
            .bind(user_id.as_str())
            .fetch_one(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err))
    }

    pub async fn update_password(&self, user_id: &Id, pwd: String) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET password = $1 WHERE id = $2")
            .bind(pwd)
            .bind(user_id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub async fn save(&self, email: String, username: String, pwd: String, salt: String) -> AppResult<Id> {
        //
        let id = Id::new();
        match sqlx::query(
            "INSERT INTO user_accounts (id, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id.as_str())
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        {
            Ok(_) => Ok(id),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegistration))),
        }
    }

    pub async fn save_with_permissions(
        &self,
        email: &String,
        username: &String,
        pwd: &String,
        salt: &String,
        permissions: Vec<String>,
    ) -> AppResult<Id> {
        //
        let id = Id::new();
        let res = sqlx::query(
            "INSERT INTO user_accounts (id, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id.as_str())
        .bind(&email)
        .bind(&username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| {
            AppError::from((
                err,
                AppUseCase::UserRegistration,
                "Self registration of admin user account".to_string(),
            ))
        });

        if res.is_ok() {
            for permission in permissions.iter() {
                let res = sqlx::query("INSERT INTO user_permissions (user_id, permission) VALUES ($1, $2)")
                    .bind(&id.as_str())
                    .bind(&permission)
                    .execute(self.dbcp.as_ref())
                    .await
                    .map_err(|err| {
                        AppError::from((
                            err,
                            AppUseCase::UserRegistration,
                            "Self registration of admin user permissions".to_string(),
                        ))
                    });
                if res.is_err() {
                    return AppResult::Err(res.err().unwrap());
                }
            }
        } else {
            return AppResult::Err(res.err().unwrap());
        }
        AppResult::Ok(id)
    }

    pub async fn update(&self, ua: UserAccount) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET username=$1, email=$2, bio=$3 WHERE id = $4")
            .bind(ua.username)
            .bind(ua.email)
            .bind(ua.bio)
            .bind(ua.id.as_str())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }
}

// -----------------------------------
//    sqlx::FromRow implementations
// -----------------------------------

impl FromRow<'_, PgRow> for UserAccount {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id::new_from(row.get("id")),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        })
    }
}

impl FromRow<'_, PgRow> for UserEntry {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: UserAccount {
                id: Id::new_from(row.try_get("id").unwrap_or_default()),
                email: row.get("email"),
                username: row.get("username"),
                bio: row.get("bio"),
                is_anonymous: row.get("is_anonymous"),
                permissions: Vec::new(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl FromRow<'_, PgRow> for UserPasswordSalt {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        //
        let err = &ctx.0;
        let uc = &ctx.1;
        match uc {
            AppUseCase::UserRegistration => match &err.as_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        // 23505 is postgres specific code for duplicate entry (named "unique_violation").
                        // See: https://www.postgresql.org/docs/16/errcodes-appendix.html.
                        "23505" => AppError::AlreadyExists("".into()),
                        _ => log_and_return_internal_err(ctx),
                    },
                    None => log_and_return_internal_err(ctx),
                },
                None => log_and_return_internal_err(ctx),
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => log_and_return_internal_err(ctx),
            },
        }
    }
}

impl From<(sqlx::Error, AppUseCase, String)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase, String)) -> Self {
        //
        let err = &ctx.0;
        let uc_info = &ctx.2;
        match ctx.1 {
            AppUseCase::UserRegistration => match &err.as_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists(uc_info.clone()),
                        _ => log_and_return_internal_err_ext(ctx),
                    },
                    None => log_and_return_internal_err_ext(ctx),
                },
                None => log_and_return_internal_err_ext(ctx),
            },
            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => log_and_return_internal_err_ext(ctx),
            },
        }
    }
}

fn log_and_return_internal_err(ctx: (sqlx::Error, AppUseCase)) -> AppError {
    log::debug!("InternalErr due to sql err={:?} on usecase:{:?}.", ctx.0, ctx.1);
    AppError::InternalErr
}

fn log_and_return_internal_err_ext(ctx: (sqlx::Error, AppUseCase, String)) -> AppError {
    log::debug!(
        "InternalErr due to sql err={:?} on usecase:{:?} and info:'{}'.",
        ctx.0,
        ctx.1,
        ctx.2
    );
    AppError::InternalErr
}

impl From<sqlx::Error> for AppError {
    //
    fn from(err: sqlx::Error) -> Self {
        //
        let mut app_err = AppError::Ignorable;
        log::debug!("from(sqlx:Error): err={:?}", err);
        if err.as_database_error().is_some() {
            // FYI: For now, any db error is considered as internal error.
            app_err = AppError::InternalErr
        }
        app_err
    }
}
