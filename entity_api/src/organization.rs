use super::error::Error;
use entity::organization;
use organization::{Entity, Model};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde_json::json;
use service::AppState;

pub async fn find_all(app_state: &AppState) -> Result<Vec<Model>, Error> {
    let db = app_state.database_connection.as_ref().unwrap();
    Ok(Entity::find().all(db).await?)
}

pub async fn find_by_id(app_state: &AppState, id: i32) -> Result<Option<Model>, Error> {
    let db = app_state.database_connection.as_ref().unwrap();
    Ok(Entity::find_by_id(id).one(db).await?)
}

pub(crate) async fn seed_database(app_state: &AppState) {
    let organization_names = [
        "Jim Hodapp Coaching",
        "Caleb Coaching",
        "Enterprise Software",
    ];

    let db = app_state.database_connection.as_ref().unwrap();

    for name in organization_names {
        let organization = organization::ActiveModel::from_json(json!({
            "name": name,
        }))
        .unwrap();

        assert_eq!(
            organization,
            organization::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(name.to_owned()),
            }
        );

        organization.insert(db).await.unwrap();
    }
}
