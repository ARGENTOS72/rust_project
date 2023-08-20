use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    model::{ModelController, Worker, WorkerForCreate},
    Result,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/workers", post(create_worker).get(list_workers))
        .route("/workers/:id", delete(delete_worker))
        .with_state(mc)
}

async fn create_worker(
    State(mc): State<ModelController>,
    Json(worker_fc): Json<WorkerForCreate>,
) -> Result<Json<Worker>> {
    let worker = mc.create_worker(worker_fc).await?;

    Ok(Json(worker))
}

async fn list_workers(State(mc): State<ModelController>) -> Result<Json<Vec<Worker>>> {
    let workers = mc.list_workers().await?;

    Ok(Json(workers))
}

async fn delete_worker(
    State(mc): State<ModelController>,
    Path(id): Path<String>,
) -> Result<Json<Worker>> {
    let worker = mc.delete_worker(id).await?;

    Ok(Json(worker))
}
