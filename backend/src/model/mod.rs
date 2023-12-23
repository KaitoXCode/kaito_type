//! Model Layer
//!
//! Design:
//! - The Model Layer - normalises the app's data type
//!   structures and access
//! - All app cod data access must go through the Model Layer
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data
//!   (e.g., db_pool, S3 client, redis client)
//! - Model Controllers (e.g., `ScriptBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Script`, `Project`)
//!   (`Bmc` is short for Backend Model Controller)
//! - In frameworks like Axum `ModelManager` are used as App State
//! - ModelManager are designed to be passed as an arg to all
//!   the ModelControllers functions
//!
// region:      --- Modules
mod base;
mod error;
pub mod script;
mod store;
pub mod user;

use crate::model::store::Db;

pub use self::error::{Error, Result};
use self::store::new_db_pool;
// endregion:   --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    // expose the all that have access to ModelManager: e.g. main.rs
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference
    /// Only for the model layer
    // only the model layer needs access to the store
    // expose the db (db pool) only the the model layer
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
