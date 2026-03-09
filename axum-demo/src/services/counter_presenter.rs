use axum::response::{IntoResponse, Response};

use crate::templates::{CounterTemplate, TemplateCollection};

pub struct CounterPresenter(pub isize);

impl IntoResponse for CounterPresenter {
    fn into_response(self) -> Response {
        TemplateCollection::new()
            .add(CounterTemplate { count: self.0 })
            .into_response()
    }
}
