// region:      --- Modules
mod script;

use crate::{
    ctx::Ctx,
    model::ModelManager,
    web::html::script::{calculate_accuracy, calculate_speed, get_script_rand},
};

use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::fmt::Display;
use tracing::debug;
// endregion:   --- Modules

// region:      --- HTML Routes
pub fn routes_pages() -> Router {
    Router::new()
        // html pages
        .route("/", get(root_home))
        .route("/login", get(login_page))
}

pub fn routes_components(mm: ModelManager) -> Router {
    Router::new()
        // htmx componens
        .route("/html/spawn_script", get(spawn_script))
        .route("/html/check_script", post(check_script))
        .with_state(mm)
}

// region:      --- Static Pages
// home
async fn root_home() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootTemplate;

// login
async fn login_page() -> impl IntoResponse {
    let template = LoginPageTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/login/login-page.html")]
struct LoginPageTemplate;

// endregion:   --- Static Pages

// region:      --- Dynamic Components
// check_script
async fn check_script(
    _ctx: Ctx,
    State(_mm): State<ModelManager>,
    Form(payload): Form<CheckPayload>,
) -> impl IntoResponse {
    // fetch inputs
    let CheckPayload {
        user_input,
        script,
        elapsed_time,
    } = payload;
    // check
    let accuracy = calculate_accuracy(&user_input, &script).await;
    // convert {00:00:00} to seconds
    let elapsed_time = elapsed_time
        .split(":")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let elapsed_time = elapsed_time[0] * 3600 + elapsed_time[1] * 60 + elapsed_time[2];
    let speed = calculate_speed(&user_input, &elapsed_time).await;
    // err handling and resp gen
    let template = CheckScriptTemplate { accuracy, speed };
    HtmlTemplate(template)
}

#[derive(Debug, Deserialize)]
struct CheckPayload {
    user_input: String,
    script: String,
    elapsed_time: String,
}

impl Display for CheckPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            &self.user_input, &self.script, &self.elapsed_time,
        )
    }
}

#[derive(Template)]
#[template(path = "components/check_script.html")]
struct CheckScriptTemplate {
    accuracy: f64,
    speed: f64,
}

// spawn_script
async fn spawn_script(ctx: Ctx, State(mm): State<ModelManager>) -> impl IntoResponse {
    let script = get_script_rand(ctx, mm).await;
    let template = match script {
        Ok(script) => SpawnScriptTemplate {
            script: script.text,
        },
        Err(err) => {
            debug!("{:<12} - spawn_script - error: {err}", "TEMPLATING");
            SpawnScriptTemplate {
                script: format!("Failed to fetch script").to_string(),
            }
        }
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "components/spawn_script.html")]
struct SpawnScriptTemplate {
    script: String,
}

// endregion:   --- Dynamic Components

// region:      --- Templating Boilerplate
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
// endregion:   --- Templating Boilerplate
