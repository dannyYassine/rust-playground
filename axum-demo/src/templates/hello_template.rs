use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "root.html")]
pub struct HelloTemplate {
    pub name: String,
    pub count: isize,
}
