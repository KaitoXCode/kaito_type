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

// deps
use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static, rpc};
// use axum::response::Html; // for hello routes
// use axum::routing::get; // for hello routes
use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::net::TcpListener; // for 0.7
                             // use std::net::SocketAddr; // for 0.6
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use askama::Template;
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

    // let routes_hello = Router::new()
    //     .route("/hello", get(|| async { Html("Hello World") }))
    //     .route_layer(middleware::from_fn(mw_ctx_resolve));

    let routes_all = Router::new()
        // .merge(routes_hello)
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .route("/", get(root_home))
        .route("/login-page", get(login_page))
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
    // region: --- Start Server 0.6
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // info!("{:<12} - {addr}\n", "LISTENING");
    // axum::Server::bind(&addr)
    //     .serve(routes_all.into_make_service())
    //     .await
    //     .unwrap();
    // Ok(())
}

// region:      --- Hello
async fn root_home() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
// endregion:   --- Hello

// region:      --- Login page
async fn login_page() -> impl IntoResponse {
    let template = LoginPageTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/login/login-page.html")]
struct LoginPageTemplate;
// endregion:   --- Login page
