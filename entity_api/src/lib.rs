// async fn seed_database(db: DatabaseConnection) {
//     let organization = organization::ActiveModel::from_json(json!({
//         "name": "Jim Hodapp Coaching",
//     }))
//     .unwrap();

//     assert_eq!(
//         organization,
//         organization::ActiveModel {
//             id: ActiveValue::NotSet,
//             name: ActiveValue::Set("Jim Hodapp Coaching".to_owned()),
//         }
//     );

//     let persisted_org: organization::Model = organization.insert(&db).await.unwrap();

//     let queried_org: Option<organization::Model> =
//         organization::Entity::find_by_id(persisted_org.id)
//             .one(&db)
//             .await
//             .unwrap();

//     println!("queried_org: {:?}", queried_org);

//     let caleb = user::ActiveModel::from_json(json!({
//         "email": "calebbourg2@gmail.com",
//         "first_name": "Caleb",
//         "last_name": "Bourg"
//     }))
//     .unwrap();

//     let persisted_caleb = caleb.insert(&db).await.unwrap();

//     let queried_caleb: Option<user::Model> = user::Entity::find_by_id(persisted_caleb.id)
//         .one(&db)
//         .await
//         .unwrap();

//     println!("queried_caleb: {:?}", queried_caleb);

//     let jim = user::ActiveModel::from_json(json!({
//         "email": "jim@jimhodappcoaching.com",
//         "first_name": "Jim",
//         "last_name": "Hodapp"
//     }))
//     .unwrap();

//     let persisted_jim = jim.insert(&db).await.unwrap();

//     let queried_jim: Option<user::Model> = user::Entity::find_by_id(persisted_jim.id)
//         .one(&db)
//         .await
//         .unwrap();

//     println!("queried_jim: {:?}", queried_jim);

//     let coaching_relationship = coaching_relationship::ActiveModel::from_json(json!({
//         "coach_id": queried_jim.unwrap().id.to_string(),
//         "coachee_id": queried_caleb.unwrap().id.to_string(),
//         "organization_id": queried_org.unwrap().id.to_string()
//     }))
//     .unwrap();

//     let persisted_coaching_relationship = coaching_relationship.insert(&db).await.unwrap();

//     let queried_coaching_relationship: Option<coaching_relationship::Model> =
//         coaching_relationship::Entity::find_by_id(persisted_coaching_relationship.id)
//             .one(&db)
//             .await
//             .unwrap();

//     println!(
//         "queried_coaching_relationship: {:?}",
//         queried_coaching_relationship
//     );
// }
