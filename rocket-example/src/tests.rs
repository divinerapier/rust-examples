#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use rocket::{
    http::{Header, Status},
    local::blocking::Client,
};

#[test]
fn test_hello_world() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client.get("/api").dispatch();
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn test_greeting() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client.get("/api/rainer").dispatch();
    assert_eq!(response.into_string(), Some("Hello rainer".into()));
}

#[test]
fn test_querystring_without_salutation() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client.get("/api/hello?name=rainer").dispatch();
    assert_eq!(response.into_string(), Some("Hello rainer".into()));
}

#[test]
fn test_querystring_with_salutation() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client
        .get("/api/hello?name=rainer&salutation=Hi")
        .dispatch();
    assert_eq!(response.into_string(), Some("Hi rainer".into()));
}

#[test]
fn test_protected_without_key() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client.get("/api/protected").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_protected_with_invalid_key() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client
        .get("/api/protected")
        .header(Header::new("x-api-key", "c2VjcmV"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_protected_with_key() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client
        .get("/api/protected")
        .header(Header::new("x-api-key", base64::encode("secret")))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_set_cookie() {
    let runtime = rocket::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = Client::tracked(runtime.block_on(super::rocket())).unwrap();
    let response = client.get("/api/login").dispatch();
    assert!(response.cookies().get("Session").is_some())
}
