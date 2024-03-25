use chrono::Utc;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, TryIntoModel};
use serde_json::json;

use entity::coaching_relationships;
use entity::organizations;
use entity::users;
use log::*;

pub mod coaching_relationship;
pub mod error;
pub mod organization;
pub mod user;

pub async fn seed_database(db: &DatabaseConnection) {
    let now = Utc::now();

    info!("Seeding database with initial data");
    info!("Creating Users");

    let jim_hodapp_params = json!({
        "email": "james.hodapp@gmail.com",
        "first_name": "Jim",
        "last_name": "Hodapp",
        "display_name": "Jim H",
        "password": "password",
        "github_username": "jhodapp",
        "github_profile_url": "https://github.com/jhodapp",
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let caleb_bourg_params = json!({
        "email": "calebbourg2@gmail.com",
        "first_name": "Caleb",
        "last_name": "Bourg",
        "display_name": "Caleb B",
        "password": "password",
        "github_username": "calebbourg",
        "github_profile_url": "https://github.com/calebbourg",
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let other_user_params = json!({
        "email": "other_user@gmail.com",
        "first_name": "Other",
        "last_name": "User",
        "display_name": "Other U",
        "password": "password",
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let jim_hodapp_model = users::ActiveModel::from_json(jim_hodapp_params)
        .unwrap()
        .try_into_model()
        .unwrap();
    let caleb_bourg_model = users::ActiveModel::from_json(caleb_bourg_params)
        .unwrap()
        .try_into_model()
        .unwrap();
    let other_user_model = users::ActiveModel::from_json(other_user_params)
        .unwrap()
        .try_into_model()
        .unwrap();

    let jim_hodapp = user::create(db, jim_hodapp_model).await.unwrap();
    let caleb_bourg = user::create(db, caleb_bourg_model).await.unwrap();
    let other_user = user::create(db, other_user_model).await.unwrap();

    info!("Creating Organizations");

    let jim_hodapp_coaching_params = json!({
        "name": "Jim Hodapp's Coaching",
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let jim_hodapp_other_org_params = json!({
        "name": "Jim Hodapp's Other Organization",
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let jim_hodapp_coaching_model =
        organizations::ActiveModel::from_json(jim_hodapp_coaching_params)
            .unwrap()
            .try_into_model()
            .unwrap();
    let jim_hodapp_other_org_model =
        organizations::ActiveModel::from_json(jim_hodapp_other_org_params)
            .unwrap()
            .try_into_model()
            .unwrap();

    let jim_hodapp_coaching = organization::create(db, jim_hodapp_coaching_model)
        .await
        .unwrap();
    let jim_hodapp_other_org = organization::create(db, jim_hodapp_other_org_model)
        .await
        .unwrap();

    info!("Creating Coaching Relationships");

    let jim_hodapp_coaching_coaching_relationship_params = json!({
        "coach_id": jim_hodapp.id,
        "coachee_id": caleb_bourg.id,
        "organization_id": jim_hodapp_coaching.id,
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let jim_hodapp_other_org_coaching_relationship_params = json!({
        "coach_id": jim_hodapp.id,
        "coachee_id": other_user.id,
        "organization_id": jim_hodapp_other_org.id,
        "external_id": Uuid::new_v4(),
        "created_at": now,
        "updated_at": now,
    });

    let jim_hodapp_coaching_coaching_relationship_model =
        coaching_relationships::ActiveModel::from_json(
            jim_hodapp_coaching_coaching_relationship_params,
        )
        .unwrap()
        .try_into_model()
        .unwrap();
    let jim_hodapp_other_org_coaching_relationship_model =
        coaching_relationships::ActiveModel::from_json(
            jim_hodapp_other_org_coaching_relationship_params,
        )
        .unwrap()
        .try_into_model()
        .unwrap();

    coaching_relationship::create(db, jim_hodapp_coaching_coaching_relationship_model)
        .await
        .unwrap()
        .try_into_model()
        .unwrap();
    coaching_relationship::create(db, jim_hodapp_other_org_coaching_relationship_model)
        .await
        .unwrap()
        .try_into_model()
        .unwrap();
}
