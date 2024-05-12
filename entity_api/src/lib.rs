use chrono::Utc;
use password_auth::generate_hash;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, Set};

use entity::coaching_relationships;
use entity::organizations;
use entity::users;

pub mod coaching_relationship;
pub mod error;
pub mod organization;
pub mod user;

pub async fn seed_database(db: &DatabaseConnection) {
    let now = Utc::now();

    let jim_hodapp: users::ActiveModel = users::ActiveModel {
        external_id: Set(Uuid::new_v4()),
        email: Set("james.hodapp@gmail.com".to_owned()),
        first_name: Set(Some("Jim".to_owned())),
        last_name: Set(Some("Hodapp".to_owned())),
        display_name: Set(Some("Jim H".to_owned())),
        password: Set(generate_hash("password")),
        github_username: Set(Some("jhodapp".to_owned())),
        github_profile_url: Set(Some("https://github.com/jhodapp".to_owned())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    let caleb_bourg: users::ActiveModel = users::ActiveModel {
        external_id: Set(Uuid::new_v4()),
        email: Set("calebbourg2@gmail.com".to_owned()),
        first_name: Set(Some("Caleb".to_owned())),
        last_name: Set(Some("Bourg".to_owned())),
        display_name: Set(Some("cbourg2".to_owned())),
        password: Set(generate_hash("password")),
        github_username: Set(Some("calebbourg".to_owned())),
        github_profile_url: Set(Some("https://github.com/calebbourg".to_owned())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    let other_user: users::ActiveModel = users::ActiveModel {
        external_id: Set(Uuid::new_v4()),
        email: Set("other_user@gmail.com".to_owned()),
        first_name: Set(Some("Other".to_owned())),
        last_name: Set(Some("User".to_owned())),
        display_name: Set(Some("Other U.".to_owned())),
        password: Set(generate_hash("password")),
        github_username: Set(None),
        github_profile_url: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    let jim_hodapp_coaching = organizations::ActiveModel {
        external_id: Set(Uuid::new_v4()),
        name: Set("Jim Hodapp's Coaching".to_owned()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    let jim_hodapp_other_org = organizations::ActiveModel {
        external_id: Set(Uuid::new_v4()),
        name: Set("Jim Hodapp's Other Organization".to_owned()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    coaching_relationships::ActiveModel {
        coach_id: Set(jim_hodapp.id.clone().unwrap()),
        coachee_id: Set(caleb_bourg.id.clone().unwrap()),
        organization_id: Set(jim_hodapp_coaching.id.unwrap()),
        external_id: Set(Uuid::new_v4()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    coaching_relationships::ActiveModel {
        coach_id: Set(jim_hodapp.id.clone().unwrap()),
        coachee_id: Set(other_user.id.clone().unwrap()),
        organization_id: Set(jim_hodapp_other_org.id.unwrap()),
        external_id: Set(Uuid::new_v4()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();
}
