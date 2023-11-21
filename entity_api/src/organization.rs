use super::error::Error;
use entity::organization;
use organization::{Entity, Model};
use sea_orm::{entity::prelude::*, ActiveValue, DatabaseConnection};
use serde_json::json;

pub async fn find_all(db: &DatabaseConnection) -> Vec<Model> {
    Entity::find().all(db).await.unwrap_or(vec![])
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, Error> {
    match Entity::find_by_id(id).one(db).await {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into()),
    }
}

pub(crate) async fn seed_database(db: &DatabaseConnection) {
    let organization_names = [
        "Jim Hodapp Coaching",
        "Caleb Coaching",
        "Enterprise Software",
    ];

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
