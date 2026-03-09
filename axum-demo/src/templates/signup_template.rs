use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "signup.html")]
pub struct SignupTemplate {}

impl SignupTemplate {
    pub fn new() -> Self {
        Self {}
    }

    pub fn signup_form(&self) -> SignupForm {
        SignupForm::new()
    }
}

#[derive(Template, Default, WebTemplate)]
#[template(path = "partials/signup_form.html")]
pub struct SignupForm {}

impl SignupForm {
    pub fn new() -> Self {
        Self {}
    }
}
