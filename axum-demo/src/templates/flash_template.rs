use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "partials/flash.html")]
pub struct FlashTemplate {
    pub error: Option<String>,
    pub success: Option<String>,
}
