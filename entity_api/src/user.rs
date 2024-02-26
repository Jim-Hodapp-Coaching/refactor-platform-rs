use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use entity::user::{self, Column, Model};
use log::*;
use password_auth::{generate_hash, verify_password};
use sea_orm::{entity::prelude::*, sea_query, ActiveValue, DatabaseConnection};
use serde::Deserialize;
use std::sync::Arc;

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
    type Error = crate::error::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        debug!("** authenticate(): {:?}:{:?}", creds.email, creds.password);

        let user: Option<Self::User> = entity::user::Entity::find()
            .filter(Column::Email.contains(creds.email))
            .one(self.db.as_ref())
            .await?;

        debug!("Get user result: {:?}", user);

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
                .ok()
                .is_some()
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        debug!("** get_user(): {:?}", *user_id);

        let user: Option<Self::User> = entity::user::Entity::find_by_id(*user_id)
            .one(self.db.as_ref())
            .await?;

        debug!("Get user result: {:?}", user);

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

pub(crate) async fn seed_database(db: &DatabaseConnection) {
    let users = vec![
        user::ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set("james.hodapp@gmail.com".to_owned()),
            first_name: ActiveValue::Set("Jim".to_owned()),
            last_name: ActiveValue::Set("Hodapp".to_owned()),
            display_name: ActiveValue::Set("Jim H".to_owned()),
            password: ActiveValue::Set(generate_hash("password1").to_owned()),
            github_username: ActiveValue::Set("jhodapp".to_owned()),
            github_profile_url: ActiveValue::Set("https://github.com/jhodapp".to_owned()),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        },
        user::ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set("test@gmail.com".to_owned()),
            first_name: ActiveValue::Set("Test First".to_owned()),
            last_name: ActiveValue::Set("Test Last".to_owned()),
            display_name: ActiveValue::Set("Test User".to_owned()),
            password: ActiveValue::Set(generate_hash("password2").to_owned()),
            github_username: ActiveValue::Set("test".to_owned()),
            github_profile_url: ActiveValue::Set("https://github.com/test".to_owned()),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        },
    ];

    for user in users {
        debug!("user: {:?}", user);

        // Upserts seeded user data:
        match user::Entity::insert(user)
            .on_conflict(
                // on conflict do update
                sea_query::OnConflict::column(user::Column::Email)
                    .update_column(user::Column::FirstName)
                    .update_column(user::Column::LastName)
                    .update_column(user::Column::Password)
                    .to_owned(),
            )
            .exec(db)
            .await
        {
            Ok(_) => info!("Succeeded in seeding user data."),
            Err(e) => error!("Failed to insert or update user when seeding user data: {e}"),
        };
    }
}
