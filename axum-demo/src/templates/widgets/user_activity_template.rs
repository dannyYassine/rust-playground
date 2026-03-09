use askama::Template;

#[derive(Template)]
#[template(path = "partials/user_activity.html")]
pub struct UserActivityTemplate {
    pub active_pct: u8,
    pub inactive_pct: u8,
}
