use super::error::{EntityApiErrorCode, Error};
use crate::uuid_parse_str;
use entity::{
    coaching_sessions::{self, Entity, Model},
    Id,
};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use std::collections::HashMap;

pub async fn find_by_coaching_relationship(
    db: &DatabaseConnection,
    coaching_relationship_id: Id,
) -> Result<Vec<Model>, Error> {
    let query = by_coaching_relationship(Entity::find(), coaching_relationship_id).await;

    Ok(query.all(db).await?)
}

pub async fn find_by(
    db: &DatabaseConnection,
    params: HashMap<String, String>,
) -> Result<Vec<Model>, Error> {
    let mut query = Entity::find();

    for (key, value) in params {
        match key.as_str() {
            "coaching_relationship_id" => {
                let coaching_relationship_id = uuid_parse_str(&value)?;
                query = by_coaching_relationship(query, coaching_relationship_id).await;
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

async fn by_coaching_relationship(
    query: Select<Entity>,
    coaching_relationship_id: Id,
) -> Select<Entity> {
    query.filter(coaching_sessions::Column::CoachingRelationshipId.eq(coaching_relationship_id))
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
    async fn find_by_coaching_relationships_returns_all_records_associated_with_coaching_relationship(
    ) -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let coaching_relationship_id = Id::new_v4();
        let _ = find_by_coaching_relationship(&db, coaching_relationship_id).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_sessions"."id", "coaching_sessions"."coaching_relationship_id", "coaching_sessions"."date", "coaching_sessions"."timezone", "coaching_sessions"."created_at", "coaching_sessions"."updated_at" FROM "refactor_platform"."coaching_sessions" WHERE "coaching_sessions"."coaching_relationship_id" = $1"#,
                [coaching_relationship_id.into()]
            )]
        );

        Ok(())
    }
}
