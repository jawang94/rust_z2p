use common::spawn_app;
use reqwest;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::routes::SubscriptionData;
mod common;

#[tokio::test]
async fn health_check_works() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke `PgConnection::connect`
    // It is not an inherent method of the struct...kinda crazy
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    // Act
    let body = "name=jason%20wang&email=jason_wang@gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // Below will error since our /subscriptions route is not implemented yet
    // let saved = sqlx::query_as!(SubscriptionData, "SELECT email, name FROM subscriptions",)
    //     .fetch_one(&mut connection)
    //     .await
    //     .expect("Failed to fetch saved subscription.");

    // assert_eq!(saved.email, "jason_wang@gmail.com");
    // assert_eq!(saved.name, "jason wang");
}
