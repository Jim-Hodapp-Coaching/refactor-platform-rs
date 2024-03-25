use super::error::Error;
use entity::{
    coaching_relationships,
    coaching_relationships::{ActiveModel, Model},
    Id,
};
use sea_orm::{entity::prelude::*, Condition, DatabaseConnection};

use log::*;

pub async fn create(
    db: &DatabaseConnection,
    coaching_relationship_active_model: ActiveModel,
) -> Result<Model, Error> {
    debug!(
        "New Coaching Relationship ActiveModel to be inserted: {:?}",
        coaching_relationship_active_model
    );

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
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // .append_query_results(coaching_relationships)
            .into_connection();

        let user_id = 1;
        let _ = find_by_user(&db, user_id).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_relationships"."id", "coaching_relationships"."external_id", "coaching_relationships"."organization_id", "coaching_relationships"."coach_id", "coaching_relationships"."coachee_id", "coaching_relationships"."created_at", "coaching_relationships"."updated_at" FROM "refactor_platform"."coaching_relationships" WHERE "coaching_relationships"."coach_id" = $1 OR "coaching_relationships"."coachee_id" = $2"#,
                [user_id.into(), user_id.into()]
            )]
        );

        Ok(())
    }
}
