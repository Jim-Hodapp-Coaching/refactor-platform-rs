use super::error::{EntityApiErrorCode, Error};
use crate::{organization::Entity, uuid_parse_str};
use chrono::Utc;
use entity::prelude::Users;
use entity::{
    coaching_relationships, organizations::*, prelude::Organizations, users, ExternalId, Id,
};
use sea_orm::{
    entity::prelude::*, sea_query, ActiveValue::Set, ActiveValue::Unchanged, DatabaseConnection,
    JoinType, QuerySelect, QueryTrait, TryIntoModel,
};
use std::collections::HashMap;

use log::*;

pub async fn create(db: &DatabaseConnection, organization_model: Model) -> Result<Model, Error> {
    debug!(
        "New Organization Model to be inserted: {:?}",
        organization_model
    );

    let now = Utc::now();

    let organization_active_model: ActiveModel = ActiveModel {
        external_id: Set(Uuid::new_v4()),
        logo: Set(organization_model.logo),
        name: Set(organization_model.name),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    };

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
                external_id: Set(Uuid::new_v4()),
                logo: Set(model.logo),
                name: Set(model.name),
                updated_at: Unchanged(organization.updated_at),
                created_at: Unchanged(organization.created_at),
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

pub async fn find_by_external_id(
    db: &DatabaseConnection,
    external_id: &ExternalId,
) -> Result<Option<Model>, Error> {
    let uuid = uuid_parse_str(external_id)?;

    let organization = Entity::find()
        .filter(Column::ExternalId.eq(uuid))
        .one(db)
        .await?;

    Ok(organization)
}

pub async fn find_by(
    db: &DatabaseConnection,
    params: HashMap<String, String>,
) -> Result<Vec<Model>, Error> {
    let mut query = Entity::find();

    for (key, value) in params {
        match key.as_str() {
            "user_id" => {
                let user_uuid = uuid_parse_str(&value)?;
                query = by_user(query, user_uuid).await;
            }
            _ => {
                return Err(Error {
                    inner: None,
                    error_code: EntityApiErrorCode::InvalidQueryTerm,
                });
            }
        }
    }

    Ok(query.all(db).await?)
}

pub async fn find_by_user(
    db: &DatabaseConnection,
    user_id: &ExternalId,
) -> Result<Vec<Model>, Error> {
    let uuid = uuid_parse_str(user_id)?;
    let organizations = by_user(Entity::find(), uuid).await.all(db).await?;

    Ok(organizations)
}

async fn by_user(query: Select<Organizations>, user_uuid: Uuid) -> Select<Organizations> {
    // subquery to find user matching the given external_id (user_id)
    let user_subquery = Users::find()
        .select_only()
        .column(users::Column::Id)
        .filter(users::Column::ExternalId.eq(user_uuid))
        .into_query();

    query
        .join(JoinType::InnerJoin, Relation::CoachingRelationships.def())
        .filter(
            sea_query::Condition::any()
                .add(coaching_relationships::Column::CoachId.in_subquery(user_subquery.to_owned()))
                .add(
                    coaching_relationships::Column::CoacheeId.in_subquery(user_subquery.to_owned()),
                ),
        )
}

#[cfg(test)]
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod tests {
    use super::*;
    use entity::organizations;
    use sea_orm::{prelude::Uuid, DatabaseBackend, MockDatabase, Transaction};

    #[tokio::test]
    async fn find_all_returns_a_list_of_records_when_present() -> Result<(), Error> {
        let now = Utc::now();
        let organizations = vec![vec![
            organizations::Model {
                id: 1,
                name: "Organization One".to_owned(),
                created_at: now.into(),
                updated_at: now.into(),
                logo: None,
                external_id: Uuid::new_v4(),
            },
            organizations::Model {
                id: 2,
                name: "Organization One".to_owned(),
                created_at: now.into(),
                updated_at: now.into(),
                logo: None,
                external_id: Uuid::new_v4(),
            },
        ]];
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .into_connection();

        assert_eq!(find_all(&db).await?, organizations[0]);

        Ok(())
    }

    #[tokio::test]
    async fn find_by_user_returns_all_records_associated_with_user() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let user_id = "a98c3295-0933-44cb-89db-7db0f7250fb1".to_string();
        let _ = find_by_user(&db, &user_id).await;

        let user_uuid = Uuid::parse_str(&user_id).unwrap();

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "organizations"."id", "organizations"."external_id", "organizations"."name", "organizations"."logo", "organizations"."created_at", "organizations"."updated_at" FROM "refactor_platform"."organizations" INNER JOIN "refactor_platform"."coaching_relationships" ON "organizations"."id" = "coaching_relationships"."organization_id" WHERE "coaching_relationships"."coach_id" IN (SELECT "users"."id" FROM "refactor_platform"."users" WHERE "users"."external_id" = $1) OR "coaching_relationships"."coachee_id" IN (SELECT "users"."id" FROM "refactor_platform"."users" WHERE "users"."external_id" = $2)"#,
                [user_uuid.clone().into(), user_uuid.into()]
            )]
        );

        Ok(())
    }

    #[tokio::test]
    async fn find_by_external_id_queries_by_external_id() -> Result<(), Error> {
        let external_id = Uuid::new_v4();
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let _ = find_by_external_id(&db, &external_id.to_string()).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "organizations"."id", "organizations"."external_id", "organizations"."name", "organizations"."logo", "organizations"."created_at", "organizations"."updated_at" FROM "refactor_platform"."organizations" WHERE "organizations"."external_id" = $1 LIMIT $2"#,
                [external_id.into(), 1u64.into()]
            )]
        );

        Ok(())
    }

    #[tokio::test]
    async fn find_by_external_id_returns_error_for_invalid_uuid() {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let external_id = "invalid_uuid".to_string();
        let result = find_by_external_id(&db, &external_id).await;

        assert!(result.is_err());
    }
}
