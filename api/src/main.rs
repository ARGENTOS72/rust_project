use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use serde_json::json;
use std::net::SocketAddr;

pub use self::error::{Error, Result};
use crate::model::ModelController;

mod error;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes = Router::new().route("/", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello_handler() -> impl IntoResponse {
    let response = Html("<h1>Hello!!!</h1>");

    dbg!(response.into_response())
}
