use askama::Template;

#[derive(Template)]
#[template(path = "partials/sidebar.html")]
pub struct SidebarTemplate<'a> {
    pub user_name: &'a str,
    pub user_email: &'a str,
    pub active_nav: String,
}
