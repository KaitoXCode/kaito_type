//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    u64,
};

// region:      --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Script {
    pub id: u64,
    pub cid: u64, // creator user id
    pub text: String,
}

#[derive(Deserialize)]
pub struct ScriptForCreate {
    pub text: String,
}

// endregion:   --- Ticket Types

// region:      -- Model Controller
#[derive(Clone)]
pub struct ModelController {
    // TODO: make real db store
    script_store: Arc<Mutex<Vec<Option<Script>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            script_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_script(&self, ctx: Ctx, script_fc: ScriptForCreate) -> Result<Script> {
        let mut store = self.script_store.lock().unwrap();
        let id = store.len() as u64;
        let script = Script {
            id,
            cid: ctx.user_id(),
            text: script_fc.text,
        };
        store.push(Some(script.clone()));
        Ok(script)
    }

    pub async fn list_scripts(&self, _ctx: Ctx) -> Result<Vec<Script>> {
        let store = self.script_store.lock().unwrap();
        let scripts = store.iter().filter_map(|s| s.clone()).collect();
        Ok(scripts)
    }

    pub async fn delete_script(&self, id: u64, _ctx: Ctx) -> Result<Script> {
        let mut store = self.script_store.lock().unwrap();
        let script = store.get_mut(id as usize).and_then(|s| s.take());
        script.ok_or(Error::ScriptDeleteFailIdNotFound { id })
    }
}

// endregion:      -- Model Controller
