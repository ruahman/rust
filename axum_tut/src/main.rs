#![allow(unused)]

use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async { Html("Hello world!!") }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listing on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("listing on {listener:?}\n");
    // axum::serve(listener, routes_hello).await.unwrap();
}
