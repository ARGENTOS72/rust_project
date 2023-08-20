use axum::{Router, Server};
use std::net::SocketAddr;

pub use self::error::{Error, Result};
use crate::model::ModelController;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes = Router::new().nest("/api", web::routes_workers::routes(mc.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
