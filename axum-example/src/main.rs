#![feature(thread_id_value)]

use std::{fmt::Debug, sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{Extension, Path},
    routing::{get, post},
    AddExtension, AddExtensionLayer, Error, Json, Router,
};
use chrono::Local;
use hyper::{Response, StatusCode};
use tower_http::trace::TraceLayer;

mod algorithm;
mod error;
mod response;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub status: usize,
    pub created_at: Option<chrono::DateTime<Local>>,
    pub updated_at: Option<chrono::DateTime<Local>>,
}

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_testing=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
#[allow(dead_code)]
async fn app() -> Router {
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(3600 * 24))
        .min_connections(5)
        .test_before_acquire(true)
        .connect("mysql://root:password@127.0.0.1:3306/testing")
        .await
        .unwrap();

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .route("/algorithms", post(algorithm::create))
        .layer(AddExtensionLayer::new(pool))
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use hyper::Method;
    use serde_json::{json, Value};
    use std::net::{SocketAddr, TcpListener};
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn hello_world() {
        let app = app().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn json() {
        let app = app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/json")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }

    #[tokio::test]
    async fn not_found() {
        let app = app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }

    // You can also spawn a server and talk to it like any other HTTP server:
    #[tokio::test]
    async fn the_real_deal() {
        let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{}", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn test_create_algorithm() {
        let app = app().await;

        let req = super::algorithm::CreateAlgorithmRequest {
            name: "alg-0000002".to_string(),
            location: "/aaaaa/bbbbbb/ccccc/ddddd".to_string(),
            image: 1000,
        };

        let req_body = serde_json::to_string(&req).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/algorithms")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .method(Method::POST)
                    .body(req_body.clone().into())
                    .unwrap(),
            )
            .await
            .unwrap();

        let (status, body) = read_response::<algorithm::CreateAlgorithmResponse>(response).await;

        match body {
            Ok(body) => {
                if body.code.eq("000000") {
                    println!("create algorithm success. id: {:?}", body.data);
                } else {
                    println!("code: {} message: {:?}", body.code, body.message);
                }
            }
            Err(e) => {
                println!("read response with error: {}", e);
            }
        }

        // let status = response.status();
        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // println!("status code: {}", status);
        // println!(
        //     "body: {}",
        //     String::from_utf8(body.into_iter().collect()).unwrap()
        // );

        assert_eq!(status, StatusCode::OK);
    }
}

async fn read_response<R>(
    resp: Response<axum::body::BoxBody>,
) -> (StatusCode, Result<response::Response<R>, serde_json::Error>)
where
    R: serde::de::DeserializeOwned,
{
    let status = resp.status();

    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let body = body.as_ref();

    (
        status,
        serde_json::from_slice::<response::Response<R>>(body),
    )
}
