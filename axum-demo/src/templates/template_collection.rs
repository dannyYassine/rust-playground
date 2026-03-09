use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub struct TemplateCollection {
    parts: Vec<String>,
    status: StatusCode,
}

impl TemplateCollection {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            status: StatusCode::OK,
        }
    }

    /// Render and collect a template. Chainable.
    pub fn add<T: Template>(mut self, template: T) -> Self {
        match template.render() {
            Ok(html) => self.parts.push(html),
            Err(e) => tracing::error!("Failed to render template: {}", e),
        }
        self
    }

    #[allow(dead_code)]
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl IntoResponse for TemplateCollection {
    fn into_response(self) -> Response {
        let body = self.parts.join("\n");
        (self.status, Html(body)).into_response()
    }
}
