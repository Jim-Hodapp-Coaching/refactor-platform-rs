use super::error::{EntityApiErrorCode, Error};
use crate::{naive_date_parse_str, uuid_parse_str};
use entity::coaching_sessions::{self, Entity, Model};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use std::collections::HashMap;

pub async fn find_by(
    db: &DatabaseConnection,
    params: HashMap<String, String>,
) -> Result<Vec<Model>, Error> {
    let mut query = Entity::find();

    for (key, value) in params {
        match key.as_str() {
            "coaching_relationship_id" => {
                let coaching_relationship_id = uuid_parse_str(&value)?;
                query = query.filter(
                    coaching_sessions::Column::CoachingRelationshipId.eq(coaching_relationship_id),
                )
            }
            "from_date" => {
                let from_date = naive_date_parse_str(&value)?;
                query = query.filter(coaching_sessions::Column::Date.gt(from_date));
            }
            "to_date" => {
                let to_date = naive_date_parse_str(&value)?;
                query = query.filter(coaching_sessions::Column::Date.lt(to_date));
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

#[cfg(test)]
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction};

    #[tokio::test]
    async fn find_by_coaching_relationships_returns_all_records_associated_with_coaching_relationship(
    ) -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let mut query_params = HashMap::new();
        let coaching_relationship_id = Id::new_v4();

        query_params.insert(
            "coaching_relationship_id".to_owned(),
            coaching_relationship_id.to_string(),
        );

        let _ = find_by(&db, query_params).await;

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

    #[tokio::test]
    async fn find_by_from_date_returns_all_records_after_date() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let mut query_params = HashMap::new();
        let from_date = NaiveDate::from_ymd(2021, 1, 1);

        query_params.insert("from_date".to_owned(), from_date.to_string());

        let _ = find_by(&db, query_params).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_sessions"."id", "coaching_sessions"."coaching_relationship_id", "coaching_sessions"."date", "coaching_sessions"."timezone", "coaching_sessions"."created_at", "coaching_sessions"."updated_at" FROM "refactor_platform"."coaching_sessions" WHERE "coaching_sessions"."date" > $1"#,
                [from_date.into()]
            )]
        );

        Ok(())
    }

    #[tokio::test]
    async fn find_by_to_date_returns_all_records_before_date() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let mut query_params = HashMap::new();
        let to_date = NaiveDate::from_ymd(2027, 1, 1);

        query_params.insert("to_date".to_owned(), to_date.to_string());

        let _ = find_by(&db, query_params).await;

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "coaching_sessions"."id", "coaching_sessions"."coaching_relationship_id", "coaching_sessions"."date", "coaching_sessions"."timezone", "coaching_sessions"."created_at", "coaching_sessions"."updated_at" FROM "refactor_platform"."coaching_sessions" WHERE "coaching_sessions"."date" < $1"#,
                [to_date.into()]
            )]
        );

        Ok(())
    }
}
