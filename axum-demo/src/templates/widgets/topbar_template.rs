use askama::Template;

#[derive(Template)]
#[template(path = "partials/topbar.html")]
pub struct TopbarTemplate;
