use super::error::Error;
use entity::notes::{ActiveModel, Model};
use sea_orm::{entity::prelude::*, DatabaseConnection, Set, TryIntoModel};

use log::*;

pub async fn create(db: &DatabaseConnection, note_model: Model) -> Result<Model, Error> {
    debug!("New Note Model to be inserted: {:?}", note_model);

    let now = chrono::Utc::now();

    let note_active_model: ActiveModel = ActiveModel {
        coaching_session_id: Set(note_model.coaching_session_id),
        body: Set(note_model.body),
        user_id: Set(note_model.user_id),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    };

    Ok(note_active_model.save(db).await?.try_into_model()?)
}

#[cfg(test)]
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod tests {
    use super::*;
    use entity::{notes::Model, Id};
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn create_returns_a_new_note_model() -> Result<(), Error> {
        let now = chrono::Utc::now();

        let note_model = Model {
            id: Id::new_v4(),
            coaching_session_id: Id::new_v4(),
            body: Some("This is a note".to_owned()),
            user_id: Id::new_v4(),
            created_at: now.into(),
            updated_at: now.into(),
        };

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![note_model.clone()]])
            .into_connection();

        let note = create(&db, note_model.clone().into()).await?;

        assert_eq!(note.id, note_model.id);

        Ok(())
    }
}
