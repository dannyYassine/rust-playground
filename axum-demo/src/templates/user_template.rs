use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub name: String,
    pub email: String,
}
