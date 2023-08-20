use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use tokio::sync::Mutex;

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: Thing,
    pub name: String,
    pub salary: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerForCreate {
    pub name: String,
    pub salary: f64,
}

#[derive(Clone)]
pub struct ModelController {
    db: Arc<Mutex<Surreal<Client>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let db = Surreal::new::<Ws>("localhost:8000")
            .await
            .map_err(|e| Error::DBError {
                error: e.to_string(),
            })?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map_err(|e| Error::DBError {
            error: e.to_string(),
        })?;

        db.use_ns("Prova")
            .use_db("Prova")
            .await
            .map_err(|e| Error::DBError {
                error: e.to_string(),
            })?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub async fn create_worker(&self, worker_fc: WorkerForCreate) -> Result<Worker> {
        let store = self.db.lock().await;

        let worker: Worker = store
            .create("Worker")
            .content(worker_fc)
            .await
            .map_err(|e| Error::DBError {
                error: e.to_string(),
            })?;

        Ok(worker)
    }

    pub async fn list_workers(&self) -> Result<Vec<Worker>> {
        let store = self.db.lock().await;

        let workers: Vec<Worker> = store.select("Worker").await.map_err(|e| Error::DBError {
            error: e.to_string(),
        })?;

        Ok(workers)
    }

    pub async fn delete_worker(&self, id: String) -> Result<Worker> {
        let store = self.db.lock().await;

        let worker: Option<Worker> =
            store
                .delete(("Worker", id.clone()))
                .await
                .map_err(|e| Error::DBError {
                    error: e.to_string(),
                })?;

        worker.ok_or(Error::WorkerNotFound { id })
    }
}
