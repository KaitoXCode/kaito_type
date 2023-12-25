// region:      --- Modules
mod script_rpc;

use crate::{
    ctx::Ctx,
    model::ModelManager,
    web::{
        rpc::script_rpc::{
            create_script, delete_script, get_script_rand, list_scripts, update_script,
        },
        Error, Result,
    },
};

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;
// endregion:   --- Modules

// region:      --- Modules
// JSON-RPC (not REST) Request Body
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}
// endregion:   --- Modules

// region:      --- Routing: AXUM handlers
pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    ctx: Ctx,
    State(mm): State<ModelManager>,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // create rpc info
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };
    // exec + store RpcInfo in response
    let mut resp = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    resp.extensions_mut().insert(rpc_info);
    return resp;
}

/// RPC basic info halding id and method for further logging
#[derive(Debug, Clone)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

macro_rules! exec_rpc_fn {
    // without params
    // {function},   {ctx}  , {ModelManager}
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
    // with params
    // {function}, {ctx}, {ModelManager}, {rpc_params}
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    // destructure RpcRequest
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;
    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");
    let result_json: Value = match rpc_method.as_str() {
        // Script RPC methods
        "create_script" => exec_rpc_fn!(create_script, ctx, mm, rpc_params),
        "list_scripts" => exec_rpc_fn!(list_scripts, ctx, mm),
        "get_script_rand" => exec_rpc_fn!(get_script_rand, ctx, mm),
        "update_script" => exec_rpc_fn!(update_script, ctx, mm, rpc_params),
        "delete_script" => exec_rpc_fn!(delete_script, ctx, mm, rpc_params),
        // fallback as err
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };
    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });
    Ok(Json(body_response))
}

// endregion:   --- Routing: AXUM handlers
