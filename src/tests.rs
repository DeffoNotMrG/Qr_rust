use super::*;
use rocket::http::{Header, Status};
use rocket::local::blocking::Client;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // Set up test-specific environment
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
    });
}

fn create_test_client() -> Client {
    setup();

    // Clean environment before setting test values
    std::env::remove_var("API_KEY");
    std::env::remove_var("PORT");

    // Set up test environment
    std::env::set_var("API_KEY", "test_key");
    std::env::set_var("PORT", "8080");

    match Client::tracked(rocket()) {
        Ok(client) => client,
        Err(e) => panic!("Failed to create test client: {:?}", e),
    }
}

#[test]
fn test_hello_endpoint() {
    let client = create_test_client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

#[test]
fn test_generate_qr_with_valid_api_key() {
    let client = create_test_client();
    let response = client
        .get("/generate?url=https://ghurmy.xyz")
        .header(Header::new("X-API-Key", "test_key"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("<svg"));
}

#[test]
fn test_generate_qr_without_api_key() {
    let client = create_test_client();
    let response = client.get("/generate?url=https://example.com").dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn test_generate_qr_with_invalid_api_key() {
    let client = create_test_client();
    let response = client
        .get("/generate?url=https://ghurmy.xyz")
        .header(Header::new("X-API-Key", "wrong_key"))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_cors_headers() {
    let client = create_test_client();
    let response = client.get("/").dispatch();

    assert_eq!(
        response.headers().get_one("Access-Control-Allow-Origin"),
        Some("*")
    );
    assert_eq!(
        response.headers().get_one("Access-Control-Allow-Methods"),
        Some("GET, POST, OPTIONS")
    );
    assert_eq!(
        response.headers().get_one("Access-Control-Allow-Headers"),
        Some("Content-Type, X-API-Key")
    );
}

#[test]
fn test_options_request() {
    let client = create_test_client();
    let response = client.options("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_generate_qr_with_invalid_url() {
    let client = create_test_client();
    let response = client
        .get("/generate?url=not_a_url")
        .header(Header::new("X-API-Key", "test_key"))
        .dispatch();

    // The QR code library should still generate a QR code even for invalid URLs
    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("<svg"));
}
