use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    #[allow(dead_code)]
    pub email: Option<String>,
    #[allow(dead_code)]
    pub password: Option<String>,
}
impl LoginTemplate {
    pub fn new() -> Self {
        Self {
            email: None,
            password: None,
        }
    }
}

#[derive(Template, Default, WebTemplate)]
#[template(path = "partials/login_form.html")]
pub struct LoginForm {}
impl LoginForm {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Template, Default, WebTemplate)]
#[template(path = "partials/login_form_footer.html")]
pub struct LoginFormFooter {}
impl LoginFormFooter {
    pub fn new() -> Self {
        Self {}
    }
}
