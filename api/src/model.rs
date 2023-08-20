use std::sync::{Arc, Mutex};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::{Error, Result};

pub struct ModelController {
    db: Arc<Mutex<Surreal<Client>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let db = Surreal::new::<Ws>("localhost:8000")
            .await
            .map_err(|e| Error::DBError {
                data: e.to_string(),
            })?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map_err(|e| Error::DBError {
            data: e.to_string(),
        })?;

        db.use_ns("Prova")
            .use_db("Prova")
            .await
            .map_err(|e| Error::DBError {
                data: e.to_string(),
            })?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}
