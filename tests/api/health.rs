use crate::utils::setup::init;

#[tokio::test]
async fn health_check_internal_works() {
    let setup = init().await;
    let response = setup.client.get_healthcheck_internal().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        json!({"status": "success" }).to_string(),
        response.text().await.unwrap()
    );
}

#[tokio::test]
async fn health_check_external_works() {
    let setup = init().await;
    let response = setup.client.get_healthcheck_external().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        json!({"status": "success" }).to_string(),
        response.text().await.unwrap()
    );
}
