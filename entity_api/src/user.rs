use super::error::{EntityApiErrorCode, Error};
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use entity::users::*;
use log::*;
use password_auth::{generate_hash, verify_password};
use sea_orm::{entity::prelude::*, prelude::Uuid, sea_query, ActiveValue, DatabaseConnection};
use serde::Deserialize;
use std::sync::Arc;

use crate::user::Entity;


pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<Model>, Error> {
    let user: Option<Model> = Entity::find()
        .filter(Column::Email.contains(email))
        .one(db)
        .await?;

    debug!("User find_by_email result: {:?}", user);

    Ok(user)
}

async fn authenticate_user(
    creds: Credentials,
    user: Model,
) -> Result<Option<Model>, Error> {
   match verify_password(&creds.password, &user.password)  {
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

pub(crate) async fn seed_database(db: &DatabaseConnection) {
    let users = vec![
        ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set("james.hodapp@gmail.com".to_owned()),
            external_id: ActiveValue::Set(Uuid::new_v4()),
            first_name: ActiveValue::Set(Some("Jim".to_owned())),
            last_name: ActiveValue::Set(Some("Hodapp".to_owned())),
            display_name: ActiveValue::Set(Some("Jim H".to_owned())),
            password: ActiveValue::Set(generate_hash("password1").to_owned()),
            github_username: ActiveValue::Set(Some("jhodapp".to_owned())),
            github_profile_url: ActiveValue::Set(Some("https://github.com/jhodapp".to_owned())),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        },
        ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set("test@gmail.com".to_owned()),
            first_name: ActiveValue::Set(Some("Test First".to_owned())),
            external_id: ActiveValue::Set(Uuid::new_v4()),
            last_name: ActiveValue::Set(Some("Test Last".to_owned())),
            display_name: ActiveValue::Set(Some("Test User".to_owned())),
            password: ActiveValue::Set(generate_hash("password2").to_owned()),
            github_username: ActiveValue::Set(Some("test".to_owned())),
            github_profile_url: ActiveValue::Set(Some("https://github.com/test".to_owned())),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        },
    ];

    for user in users {
        debug!("user: {:?}", user);

        // Upserts seeded user data:
        match Entity::insert(user)
            .on_conflict(
                // on conflict do update
                sea_query::OnConflict::column(Column::Email)
                    .update_column(Column::FirstName)
                    .update_column(Column::LastName)
                    .update_column(Column::Password)
                    .to_owned(),
            )
            .exec(db)
            .await
        {
            Ok(_) => info!("Succeeded in seeding user entity."),
            Err(e) => error!("Failed to insert or update user entity when seeding user data: {e}"),
        };
    }
}
