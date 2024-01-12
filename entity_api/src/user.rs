use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId, tracing::field::debug};
use entity::user::{Column, Model};
//use super::error::{EntityApiErrorCode, Error};
use log::*;
use password_auth::verify_password;
use sea_orm::{entity::prelude::*, DatabaseConnection};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Backend {
    db: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

impl Backend {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: Arc::new(db.clone()) }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    // TODO: I think we need to wrap entity::user::Model so that
    // the DeriveEntityModel doesn't "erase" the AuthUser impl
    type User = Model;
    type Credentials = Credentials;
    type Error = crate::error::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        debug("** authenticate()");

        let user: Option<Self::User> = entity::user::Entity::find()
            .filter(Column::Email.contains(creds.username))
            .one(self.db.as_ref())
            .await?;

        debug!("User found: {:?}", user);

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
            .ok()
            .is_some()
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        debug("** get_user()");

        let user: Option<Self::User> = entity::user::Entity::find_by_id(*user_id)
            .one(self.db.as_ref())
            .await?;

        debug!("User found: {:?}", user);

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
