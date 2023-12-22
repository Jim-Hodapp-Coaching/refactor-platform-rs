use super::error::{EntityApiErrorCode, Error};
use entity::{organization, Id};
use organization::{ActiveModel, Entity, Model};
use sea_orm::{
    entity::prelude::*, ActiveValue, ActiveValue::Set, ActiveValue::Unchanged, DatabaseConnection,
    TryIntoModel,
};
use serde_json::json;

extern crate log;
use log::*;

pub async fn create(db: &DatabaseConnection, organization_model: Model) -> Result<Model, Error> {
    let organization_active_model: ActiveModel = ActiveModel {
        name: Set(organization_model.name.to_owned()),
        ..Default::default()
    };
    debug!(
        "New Organization ActiveModel to be inserted: {:?}",
        organization_active_model
    );

    Ok(organization_active_model.insert(db).await?)
}

pub async fn update(db: &DatabaseConnection, id: Id, model: Model) -> Result<Model, Error> {
    let result = find_by_id(db, id).await?;

    match result {
        Some(organization) => {
            debug!(
                "Existing Organization model to be Updated: {:?}",
                organization
            );

            let active_model: ActiveModel = ActiveModel {
                id: Unchanged(organization.id),
                name: Set(model.name),
            };
            Ok(active_model.update(db).await?.try_into_model()?)
        }
        None => Err(Error {
            inner: None,
            error_code: EntityApiErrorCode::RecordNotFound,
        }),
    }
}

pub async fn delete_by_id(db: &DatabaseConnection, id: Id) -> Result<(), Error> {
    let result = find_by_id(db, id).await?;

    match result {
        Some(organization_model) => {
            debug!(
                "Existing Organization model to be deleted: {:?}",
                organization_model
            );

            organization_model.delete(db).await?;
            Ok(())
        }
        None => Err(Error {
            inner: None,
            error_code: EntityApiErrorCode::RecordNotFound,
        }),
    }
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, Error> {
    Ok(Entity::find().all(db).await?)
}

pub async fn find_by_id(db: &DatabaseConnection, id: Id) -> Result<Option<Model>, Error> {
    let organization = Entity::find_by_id(id).one(db).await?;
    debug!("Organization found: {:?}", organization);

    Ok(organization)
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

#[cfg(test)]
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_all_returns_a_list_of_records_when_present() -> Result<(), Error> {
        let organizations = vec![vec![
            organization::Model {
                id: 1,
                name: "Organization One".to_owned(),
            },
            organization::Model {
                id: 2,
                name: "Organization One".to_owned(),
            },
        ]];
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .into_connection();

        assert_eq!(find_all(&db).await?, organizations[0]);

        Ok(())
    }
}
