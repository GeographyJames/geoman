use geoman::ogc::types::common::{Collection, Collections};

use crate::app::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn get_collections_works() {
    let app = TestApp::spawn_with_db().await;

    let response = app.ogc_service.get_collections(&app.api_client).await;

    assert_ok(&response);

    let collections: Collections = handle_json_response(response)
        .await
        .expect("failed to retrieve collections");

    assert_eq!(collections.collections.len(), 0)
}

#[actix_web::test]
async fn get_collection_works() {
    let app = TestApp::spawn_with_db().await;

    // Insert a test team first (required for user foreign key)
    sqlx::query!(
        r#"
        INSERT INTO app.teams (id, name)
        OVERRIDING SYSTEM VALUE
        VALUES (1, 'test-team')
        "#
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert test team");

    // Insert a test user (required for collection foreign key)
    sqlx::query!(
        r#"
        INSERT INTO app.users (id, username, admin, team_id)
        OVERRIDING SYSTEM VALUE
        VALUES (1, 'test-user', false, 1)
        "#
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert test user");

    // Insert a test collection
    sqlx::query!(
        r#"
        INSERT INTO app.collections (title, slug, geometry_type, added_by, last_updated_by)
        VALUES ('Test Collection', 'test-collection', 'POINT', 1, 1)
        "#
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert test collection");

    let response = app
        .ogc_service
        .get_collection(&app.api_client, "test-collection")
        .await;

    assert_ok(&response);

    let collection: Collection = handle_json_response(response)
        .await
        .expect("failed to retrieve collection");

    assert_eq!(collection.id, "test-collection");
    assert_eq!(collection.title, "Test Collection");
    assert!(!collection.links.is_empty());
}

#[actix_web::test]
async fn get_collection_returns_404_when_not_found() {
    let app = TestApp::spawn_with_db().await;

    let response = app
        .ogc_service
        .get_collection(&app.api_client, &uuid::Uuid::new_v4().to_string())
        .await;

    assert_eq!(response.status(), 404);
}
