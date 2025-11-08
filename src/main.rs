use axum::{serve, Router,};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer,};

use axum::http::{HeaderValue, Method};


mod db;
mod models;
mod repository;
mod services;
mod controllers;
mod routes;
mod utils;

use routes::download_routes::create_download_routes;
use crate::db::init_db;

#[tokio::main]
async fn main() {

 
    dotenv().ok();

    let pool = init_db().await.expect("Failed to connect to DB");

   
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![Method::GET,Method::POST,Method::PUT,Method::DELETE,])
        .allow_headers(Any);


    let app = create_download_routes(pool).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    serve(listener, app).await.expect("Server failed");
}
