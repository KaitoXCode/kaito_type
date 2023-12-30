// region:      --- Modules
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
// endregion:   --- Modules

// region:      --- HTML Routes
pub fn routes_pages() -> Router {
    Router::new()
        // html pages
        .route("/", get(root_home))
        .route("/login", get(login_page))
}

pub fn routes_components() -> Router {
    Router::new()
        // htmx componens
        .route("/html/spawn_script", get(spawn_script))
}

// region:      --- Static Pages
async fn root_home() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootTemplate;

async fn login_page() -> impl IntoResponse {
    let template = LoginPageTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/login/login-page.html")]
struct LoginPageTemplate;

// endregion:   --- Static Pages

// region:      --- Dynamic Components
async fn spawn_script() -> impl IntoResponse {
    // FIXME: get the script from db
    let script_text = String::from("spawned scipt #1");
    let template = SpawnScriptTemplate {
        script: script_text,
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
