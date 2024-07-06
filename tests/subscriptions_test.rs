mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=jason%20wang&email=jasonemail%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing() {
    let app = common::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=jason%20wang", "missing email"),
        ("email=jawango%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
