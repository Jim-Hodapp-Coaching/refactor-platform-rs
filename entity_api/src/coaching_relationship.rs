use super::error::{EntityApiErrorCode, Error};
use crate::uuid_parse_str;
use chrono::Utc;
use entity::{
    coaching_relationships::{self, ActiveModel, Model}, users, Id
};
use sea_orm::{entity::prelude::*, Condition, DatabaseConnection, JoinType, QuerySelect, QueryTrait, Set, sea_query::Alias};

use log::*;

pub async fn create(
    db: &DatabaseConnection,
    coaching_relationship_model: Model,
) -> Result<Model, Error> {
    debug!(
        "New Coaching Relationship Model to be inserted: {:?}",
        coaching_relationship_model
    );

    let now = Utc::now();

    let coaching_relationship_active_model: ActiveModel = ActiveModel {
        organization_id: Set(coaching_relationship_model.organization_id),
        coach_id: Set(coaching_relationship_model.coach_id),
        coachee_id: Set(coaching_relationship_model.coachee_id),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    };

    Ok(coaching_relationship_active_model.insert(db).await?)
}

pub async fn find_by_user(db: &DatabaseConnection, user_id: Id) -> Result<Vec<Model>, Error> {
    let coaching_relationships: Vec<coaching_relationships::Model> =
        coaching_relationships::Entity::find()
            .filter(
                Condition::any()
                    .add(coaching_relationships::Column::CoachId.eq(user_id))
                    .add(coaching_relationships::Column::CoacheeId.eq(user_id)),
            )
            .all(db)
            .await?;

    Ok(coaching_relationships)
}

pub async fn find_by_organization(
    db: &DatabaseConnection,
    organization_id: Id,
) -> Result<Vec<Model>, Error> {
    let query = by_organization(coaching_relationships::Entity::find(), organization_id).await;

    Ok(query.all(db).await?)
}

pub async fn find_by_organization_with_user_names(
    db: &DatabaseConnection,
    organization_id: Id,
) -> Result<Vec<Model>, Error> {
    let query = by_organization(coaching_relationships::Entity::find(), organization_id).await
        .join_as(JoinType::Join, users::Relation::Coach.def().rev(), Alias::new("coach"))
        // .join_as(JoinType::Join, coaching_relationships::Relation::Coach.def().rev(), Alias::new("coach"))
        // .join_as(JoinType::Join, coaching_relationships::Relation::Coachee.def().rev(), Alias::new("coachee"))
        .select_only();
        // .column(coaching_relationships::Column::Id)
        // .column(coaching_relationships::Column::OrganizationId)
        // .column(coaching_relationships::Column::CoachId)
        // .column(coaching_relationships::Column::CoacheeId)
        // .column_as(crate::users::Column::FirstName, "coach_first_name")
        // .column_as(crate::users::Column::LastName, "coach_last_name")
        // .column_as(crate::users::Column::FirstName, "coachee_first_name")
        // .column_as(crate::users::Column::LastName, "coachee_last_name");
        // .into_query();
        




    Ok(query.all(db).await?)
}

pub async fn find_by(
    db: &DatabaseConnection,
    params: std::collections::HashMap<String, String>,
) -> Result<Vec<Model>, Error> {
    let mut query = coaching_relationships::Entity::find();

    for (key, value) in params.iter() {
        match key.as_str() {
            "organization_id" => {
                query = by_organization(query, uuid_parse_str(value)?).await;
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

async fn by_organization(
    query: Select<coaching_relationships::Entity>,
    organization_id: Id,
) -> Select<coaching_relationships::Entity> {
    let organization_subquery = entity::organizations::Entity::find()
        .select_only()
        .column(entity::organizations::Column::Id)
        .filter(entity::organizations::Column::Id.eq(organization_id))
        .into_query();

    query.filter(
        coaching_relationships::Column::OrganizationId
            .in_subquery(organization_subquery.to_owned()),
    )
}

// A convenient combined struct that holds the results of looking up the Users associated
// with the coach/coachee ids. This should be used as an implementation detail only.
// struct CoachingRelationshipWithNames {
//     pub id: Id,
//     pub coach_id: Id,
//     pub coachee_id: Id,
//     pub coach_name: String,
//     pub coachee_name: String,
// }

// impl From<Model> for CoachingRelationshipWithNames {
//     fn from(model: Model) -> Self {
//         coach = 
//         Self {
//             id: model.id,
//             coach_id: model.coach_id,
//             coachee_id: model.coachee_id,
//             coach_name: ,
//             coachee_name: "Coachee".to_string(),
//         }
//     }
// }

// serialize the CoachingRelationshipWithNames struct so that it can be used in the API
// and appears to be a coaching_relationship JSON object.
// impl Serialize for CoachingRelationshipWithNames {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("Color", 3)?;
//         state.serialize_field("", &self.r)?;
//         state.serialize_field("g", &self.g)?;
//         state.serialize_field("b", &self.b)?;
//         state.end()
//     }
// }

#[cfg(test)]
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction};

    #[tokio::test]
    async fn find_by_user_returns_all_records_associated_with_user() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let user_id = Id::new_v4();
        let _ = find_by_user(&db, user_id).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_relationships"."id", "coaching_relationships"."organization_id", "coaching_relationships"."coach_id", "coaching_relationships"."coachee_id", "coaching_relationships"."created_at", "coaching_relationships"."updated_at" FROM "refactor_platform"."coaching_relationships" WHERE "coaching_relationships"."coach_id" = $1 OR "coaching_relationships"."coachee_id" = $2"#,
                [user_id.into(), user_id.into()]
            )]
        );

        Ok(())
    }

    #[tokio::test]
    async fn find_by_organization_queries_for_all_records_by_organization() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let organization_id = Id::new_v4();
        let _ = find_by_organization(&db, organization_id).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_relationships"."id", "coaching_relationships"."organization_id", "coaching_relationships"."coach_id", "coaching_relationships"."coachee_id", "coaching_relationships"."created_at", "coaching_relationships"."updated_at" FROM "refactor_platform"."coaching_relationships" WHERE "coaching_relationships"."organization_id" IN (SELECT "organizations"."id" FROM "refactor_platform"."organizations" WHERE "organizations"."id" = $1)"#,
                [organization_id.clone().into()]
            )]
        );

        Ok(())
    }

    #[tokio::test]
    async fn find_by_organization_with_user_names_returns_all_records_by_organization_with_user_names() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let organization_id = Id::new_v4();
        let _ = find_by_organization_with_user_names(&db, organization_id).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_relationships"."id", "coaching_relationships"."organization_id", "coaching_relationships"."coach_id", "coaching_relationships"."coachee_id", "coaching_relationships"."created_at", "coaching_relationships"."updated_at" FROM "refactor_platform"."coaching_relationships" WHERE "coaching_relationships"."organization_id" IN (SELECT "organizations"."id" FROM "refactor_platform"."organizations" WHERE "organizations"."id" = $1)"#,
                [organization_id.clone().into()]
            )]
        );

        Ok(())
    }
}
