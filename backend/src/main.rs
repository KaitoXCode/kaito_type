// region:      --- Modules
// sub modules
mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)] // commented during early dev
pub mod _dev_utils;

// re-exports: facilitates -> e.g. use::crate::config
pub use self::error::{Error, Result};
pub use config::config;
use tower_http::services::ServeDir;

// deps
use crate::model::ModelManager;
use crate::web::html;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static, rpc};
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion:   --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false) // TODO: test without
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Launching Backend...");

    // dev only
    _dev_utils::init_dev().await;

    // initialise modelmanager
    let mm = ModelManager::new().await?;

    // def routes
    let routes_rpc = rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_comps =
        html::routes_components(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api/v1", routes_rpc)
        .nest("/api/v2", routes_comps)
        .merge(html::routes_pages())
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());
    // region: --- Start Server 0.7
    let listner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("LISTENING on {:?}\n", listner.local_addr().unwrap());
    axum::serve(listner, routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}
