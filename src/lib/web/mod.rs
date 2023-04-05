pub mod ctx;
pub mod form;
pub mod hitcounter;
pub mod http;
pub mod renderer;

pub const PASSWORD_COOKIE: &str = "password";

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    Serialization(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String),
}

impl From<handlebars::RenderError> for PageError {
    fn from(value: handlebars::RenderError) -> Self {
        PageError::Render(format!("{}", value))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(value: serde_json::Error) -> Self {
        PageError::Serialization(format!("{}", value))
    }
}
