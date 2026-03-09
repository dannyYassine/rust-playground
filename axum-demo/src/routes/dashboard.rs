use std::sync::Arc;

use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;
use crate::models::User;
use crate::templates::DashboardTemplate;
use crate::templates::widgets::{
    ChartCardTemplate, Order, OrderStatus, OrdersTableTemplate, StatCardTemplate, TopbarTemplate,
    UserActivityTemplate,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/dashboard", get(dashboard_page_handler))
}

async fn dashboard_page_handler(user: User) -> Result<DashboardTemplate, StatusCode> {
    let topbar = TopbarTemplate;

    let stat_card_progress = StatCardTemplate {
        label: "Order on progress".to_string(),
        value: "3,279".to_string(),
        icon_color: "orange".to_string(),
        icon_svg: r#"
            <line x1="18" y1="20" x2="18" y2="10" />
            <line x1="12" y1="20" x2="12" y2="4" />
            <line x1="6" y1="20" x2="6" y2="14" />
        "#
        .to_string(),
        bar_pct: 73,
        range_min: "2548 pcs".to_string(),
        range_max: "3500 pcs".to_string(),
    };

    let stat_card_delivery = StatCardTemplate {
        label: "On delivery order".to_string(),
        value: "3,279".to_string(),
        icon_color: "purple".to_string(),
        icon_svg: r#"
            <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
            <polyline points="16.5 9.4 7.55 4.24" />
            <polyline points="3.29 7 12 12 20.71 7" />
            <line x1="12" y1="22" x2="12" y2="12" />
            <path d="m18 21 3-3-3-3" />
            <path d="m15 18h6" />
        "#
        .to_string(),
        bar_pct: 73,
        range_min: "2548 pcs".to_string(),
        range_max: "3500 pcs".to_string(),
    };

    let stat_card_success = StatCardTemplate {
        label: "Success order".to_string(),
        value: "3,279".to_string(),
        icon_color: "green".to_string(),
        icon_svg: r#"
            <polyline points="20 6 9 17 4 12" />
        "#
        .to_string(),
        bar_pct: 73,
        range_min: "2548 pcs".to_string(),
        range_max: "3500 pcs".to_string(),
    };

    let orders_table = OrdersTableTemplate {
        orders: vec![
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#4f46e5".to_string(),
                status: OrderStatus::Success,
            },
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#22c55e".to_string(),
                status: OrderStatus::Success,
            },
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#f97316".to_string(),
                status: OrderStatus::Cancelled,
            },
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#4f46e5".to_string(),
                status: OrderStatus::Success,
            },
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#22c55e".to_string(),
                status: OrderStatus::Success,
            },
            Order {
                name: "Iphone 12 Pro Max".to_string(),
                date: "01-03-2019".to_string(),
                color: "#6366f1".to_string(),
                status: OrderStatus::Cancelled,
            },
        ],
    };

    let chart_card = ChartCardTemplate {
        label: "Average daily respond".to_string(),
        value: "8.25h".to_string(),
        year: "2021".to_string(),
    };

    let user_activity = UserActivityTemplate {
        active_pct: 92,
        inactive_pct: 8,
    };

    Ok(DashboardTemplate {
        user,
        topbar,
        stat_card_progress,
        stat_card_delivery,
        stat_card_success,
        orders_table,
        chart_card,
        user_activity,
    })
}
