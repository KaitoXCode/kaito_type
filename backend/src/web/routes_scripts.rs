use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

use crate::ctx::Ctx;
use crate::model::{ModelController, Script, ScriptForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        // here post and get are chained, as they have the same route path
        .route("/scripts", post(create_script).get(list_scripts))
        .route("/scripts/:id", delete(delete_script))
        .with_state(mc)
}

// region:      --- REST Handlers
async fn create_script(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(script_fc): Json<ScriptForCreate>,
) -> Result<Json<Script>> {
    println!("->> {:<12} - create_script", "HANDLER");
    let script = mc.create_script(ctx, script_fc).await?;
    Ok(Json(script))
}

async fn list_scripts(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Script>>> {
    println!("->> {:<12} - list_scripts", "HANDLER");
    let scripts = mc.list_scripts(ctx).await?;
    Ok(Json(scripts))
}

async fn delete_script(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Script>> {
    println!("->> {:<12} - list_scripts", "HANDLER");
    let script = mc.delete_script(id, ctx).await?;
    Ok(Json(script))
}

// endregion:   --- REST Handlers
