use super::error::{EntityApiErrorCode, Error};
use entity::organization;
use organization::{ActiveModel, Entity, Model};
use sea_orm::{entity::prelude::*, ActiveValue, DatabaseConnection, TryIntoModel};
use serde_json::json;

pub async fn create(db: &DatabaseConnection, organization_model: Model) -> Result<Model, Error> {
    let organization_active_model: ActiveModel = organization_model.into();
    Ok(organization_active_model.insert(db).await?)
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    organization_model: Model,
) -> Result<Model, Error> {
    let result = find_by_id(db, id).await?;

    match result {
        Some(_) => {
            let active_model: ActiveModel = organization_model.into();
            Ok(active_model.save(db).await?.try_into_model()?)
        }
        None => Err(Error {
            inner: None,
            error_code: EntityApiErrorCode::RecordNotFound,
        }),
    }
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<(), Error> {
    let result = find_by_id(db, id).await?;

    match result {
        Some(organization_model) => {
            organization_model.delete(db).await?;
            Ok(())
        }
        None => Err(Error {
            inner: None,
            error_code: EntityApiErrorCode::RecordNotFound,
        }),
    }
}

pub async fn find_all(db: &DatabaseConnection) -> Vec<Model> {
    Entity::find().all(db).await.unwrap_or(vec![])
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, Error> {
    Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|err| err.into())
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
