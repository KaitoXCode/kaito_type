use crate::{crypt, model, web};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- rpc
    RpcMethodUnknown(String),
    RpcMissingParams { rpc_method: String },
    RpcFailJsonParams { rpc_method: String },
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd { user_id: i64 },
    LoginFailPwdNotMatching { user_id: i64 },
    // -- CtxExtError
    CtxExt(web::mw_auth::CtxExtError),
    // modules
    Model(model::Error),
    Crypt(crypt::Error),
    // external modules
    SerdeJson(String),
}
// region:      --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::Crypt(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val.to_string())
    }
}

// endregion:   --- Froms

// region:      ---Error boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion:   ---Error boilerplate

// region:      --- Axum IntoResponse
// TODO: improve this maybe too vague / loss of info
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // log the internal error before making it vague for clien
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // response
        // -> define body
        let body = match self {
            _ => "something went wrong",
        };
        // -> return full response to client
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// endregion:   --- Axum IntoResponse

// region:      --- Client Error
/// Convert internal error into http status code and ClientError for external
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // -- login
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // model
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),
            // -- fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    SERVICE_ERROR,
}
// endregion:       --- Client Error
