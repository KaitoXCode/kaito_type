pub use self::error::{Error, Result};
use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    println!("Launching Backend...");
    // define routes:
    let routes_all = Router::new()
        .merge(routes_all())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());
    // region: --- Start Server
    let listner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listner);
    axum::serve(listner, routes_all.into_make_service())
        .await
        .unwrap();
    // // build our application with a single route
    // let app = Router::new().route("/hello", get(|| async { Html("Hello <strong>World!!!</strong>") }));
    //
    // // run our app with hyper, listening glabally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

// region:      --- Start Server
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:      --- Routes Hello
fn routes_all() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Kaito`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Kaito`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
