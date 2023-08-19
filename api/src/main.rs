use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("Prova").use_db("Prova").await?;

    let routes = Router::new().route("/", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello_handler() -> impl IntoResponse {
    Html("<h1>Hello!!!</h1>")
}
