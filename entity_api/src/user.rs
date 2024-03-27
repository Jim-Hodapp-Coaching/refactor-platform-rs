use super::error::{EntityApiErrorCode, Error};
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use chrono::Utc;
use entity::users::*;
use log::*;
use password_auth::{generate_hash, verify_password};
use sea_orm::{entity::prelude::*, DatabaseConnection, Set};
use serde::Deserialize;
use std::sync::Arc;

use crate::user::Entity;

pub async fn create(db: &DatabaseConnection, user_model: Model) -> Result<Model, Error> {
    debug!(
        "New User Relationship Model to be inserted: {:?}",
        user_model
    );

    let now = Utc::now();

    let user_active_model: ActiveModel = ActiveModel {
        external_id: Set(Uuid::new_v4()),
        email: Set(user_model.email),
        first_name: Set(user_model.first_name),
        last_name: Set(user_model.last_name),
        display_name: Set(user_model.display_name),
        password: Set(generate_hash(user_model.password)),
        github_username: Set(user_model.github_username),
        github_profile_url: Set(user_model.github_profile_url),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    };

    Ok(user_active_model.insert(db).await?)
}

pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<Model>, Error> {
    let user: Option<Model> = Entity::find()
        .filter(Column::Email.contains(email))
        .one(db)
        .await?;

    debug!("User find_by_email result: {:?}", user);

    Ok(user)
}

async fn authenticate_user(creds: Credentials, user: Model) -> Result<Option<Model>, Error> {
    match verify_password(creds.password, &user.password) {
        Ok(_) => Ok(Some(user)),
        Err(_) => Err(Error {
            inner: None,
            error_code: EntityApiErrorCode::RecordUnauthenticated,
        }),
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

impl Backend {
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        info!("** Backend::new()");
        Self {
            // Arc is cloned, but the inner DatabaseConnection refers to the same instance
            // as the one passed in to new() (see the Arc documentation for more info)
            db: Arc::clone(db),
        }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = Model;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        debug!("** authenticate(): {:?}:{:?}", creds.email, creds.password);

        match find_by_email(&self.db, &creds.email).await? {
            Some(user) => authenticate_user(creds, user).await,
            None => Err(Error {
                inner: None,
                error_code: EntityApiErrorCode::RecordUnauthenticated,
            }),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        debug!("** get_user(): {:?}", *user_id);

        let user: Option<Self::User> = Entity::find_by_id(*user_id).one(self.db.as_ref()).await?;

        debug!("Get user result: {:?}", user);

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
