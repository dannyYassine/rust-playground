use askama::Template;
use askama_web::WebTemplate;

use crate::{
    models::User,
    templates::widgets::{
        ChartCardTemplate, OrdersTableTemplate, SidebarTemplate, StatCardTemplate, TopbarTemplate,
        UserActivityTemplate,
    },
};

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub user: User,
    pub topbar: TopbarTemplate,
    pub stat_card_progress: StatCardTemplate,
    pub stat_card_delivery: StatCardTemplate,
    pub stat_card_success: StatCardTemplate,
    pub orders_table: OrdersTableTemplate,
    pub chart_card: ChartCardTemplate,
    pub user_activity: UserActivityTemplate,
}

impl DashboardTemplate {
    pub fn sidebar(&self) -> SidebarTemplate<'_> {
        let sidebar = SidebarTemplate {
            user_name: self.user.name.as_str(),
            user_email: self.user.email.as_str(),
            active_nav: "dashboard".to_string(),
        };

        return sidebar;
    }
}
