use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

use crate::mockservices::{rust, RustService, Web3Service, WorldService};

pub async fn start_server() {
    // here we initialize logging
    tracing_subscriber::fmt().init();

    // here we create 3 instance
    let rust_service = RustService::new();
    let world_service = WorldService::new();
    let web3_service = Web3Service::new();

    // create a basic app of Axum with a handler to response 'Hello World'
    let app = Router::new()
        .route(
            "/helloworld",
            get(move || async { world_service.hello().await }),
        )
        .route(
            "/hellorust",
            get(move || async { rust_service.hello().await }),
        )
        .route(
            "/helloweb3",
            get(move || async { web3_service.hello().await }),
        );

    // declare the port for the server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3033".to_string());

    // create the address this server deploy
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse().unwrap();

    info!(address = %addr, "Server listening");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
