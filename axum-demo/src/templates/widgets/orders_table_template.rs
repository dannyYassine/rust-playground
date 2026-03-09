use askama::Template;

pub enum OrderStatus {
    Success,
    Cancelled,
}

pub struct Order {
    pub name: String,
    pub date: String,
    pub color: String,
    pub status: OrderStatus,
}

impl Order {
    pub fn status_class(&self) -> &'static str {
        match self.status {
            OrderStatus::Success => "success",
            OrderStatus::Cancelled => "cancelled",
        }
    }

    pub fn status_label(&self) -> &'static str {
        match self.status {
            OrderStatus::Success => "Success",
            OrderStatus::Cancelled => "Cancelled",
        }
    }
}

#[derive(Template)]
// #[template(path = "partials/orders_table.html")]
#[template(
    source = r#"
    <div class="orders-card">
        <div class="orders-header">
            <span class="orders-title">Recent Order</span>
            <button class="btn-see-more">See More</button>
        </div>
        <table class="orders-table">
            <thead>
                <tr>
                    <th>Order Name <span class="sort-arrow">▼</span></th>
                    <th>Date <span class="sort-arrow">▼</span></th>
                    <th>Status <span class="sort-arrow">▼</span></th>
                </tr>
            </thead>
            <tbody>
                {% for order in orders %}
                <tr>
                    <td>
                        <div class="order-name-cell">
                            <div class="order-thumb" style="background: {{ order.color }}">
                                <svg
                                    width="16"
                                    height="16"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                >
                                    <rect x="5" y="2" width="14" height="20" rx="2" />
                                    <line x1="9" y1="7" x2="15" y2="7" />
                                    <line x1="9" y1="11" x2="15" y2="11" />
                                </svg>
                            </div>
                            {{ order.name }}
                        </div>
                    </td>
                    <td>{{ order.date }}</td>
                    <td>
                        {{ order_tag_cell(order)|safe }}
                    </td>
                </tr>
                {% endfor %}
            </tbody>
        </table>
    </div>
"#,
    ext = "html"
)]
pub struct OrdersTableTemplate {
    pub orders: Vec<Order>,
}
impl OrdersTableTemplate {
    fn order_tag_cell<'a>(&self, order: &'a Order) -> OrderTagCell<'a> {
        OrderTagCell { order: order }
    }
}

#[derive(Template)]
#[template(
    source = r#"
    <span class="badge-status {{ order.status_class() }}">
        {{ order.status_label() }}
    </span>
"#,
    ext = "html"
)]
pub struct OrderTagCell<'a> {
    pub order: &'a Order,
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    fn make_order(name: &str, date: &str, color: &str, status: OrderStatus) -> Order {
        Order {
            name: name.to_string(),
            date: date.to_string(),
            color: color.to_string(),
            status,
        }
    }

    // --- Order helpers ---

    #[test]
    fn test_status_class_success() {
        let order = make_order("A", "2024-01-01", "#fff", OrderStatus::Success);
        assert_eq!(order.status_class(), "success");
    }

    #[test]
    fn test_status_class_cancelled() {
        let order = make_order("A", "2024-01-01", "#fff", OrderStatus::Cancelled);
        assert_eq!(order.status_class(), "cancelled");
    }

    #[test]
    fn test_status_label_success() {
        let order = make_order("A", "2024-01-01", "#fff", OrderStatus::Success);
        assert_eq!(order.status_label(), "Success");
    }

    #[test]
    fn test_status_label_cancelled() {
        let order = make_order("A", "2024-01-01", "#fff", OrderStatus::Cancelled);
        assert_eq!(order.status_label(), "Cancelled");
    }

    // --- OrdersTableTemplate rendering ---

    #[test]
    fn test_renders_empty_orders() {
        let tmpl = OrdersTableTemplate { orders: vec![] };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("orders-table"));
        assert!(html.contains("Recent Order"));
        assert!(!html.contains("<td>"));
    }

    #[test]
    fn test_renders_order_name_and_date() {
        let tmpl = OrdersTableTemplate {
            orders: vec![make_order(
                "Widget Pro",
                "2024-06-01",
                "#ff0000",
                OrderStatus::Success,
            )],
        };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("Widget Pro"));
        assert!(html.contains("2024-06-01"));
    }

    #[test]
    fn test_renders_order_color_in_style() {
        let tmpl = OrdersTableTemplate {
            orders: vec![make_order(
                "Item",
                "2024-06-01",
                "#abcdef",
                OrderStatus::Success,
            )],
        };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("background: #abcdef"));
    }

    #[test]
    fn test_renders_success_badge() {
        let tmpl = OrdersTableTemplate {
            orders: vec![make_order(
                "Item",
                "2024-06-01",
                "#fff",
                OrderStatus::Success,
            )],
        };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("badge-status success"));
        assert!(html.contains("Success"));
    }

    #[test]
    fn test_renders_cancelled_badge() {
        let tmpl = OrdersTableTemplate {
            orders: vec![make_order(
                "Item",
                "2024-06-01",
                "#fff",
                OrderStatus::Cancelled,
            )],
        };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("badge-status cancelled"));
        assert!(html.contains("Cancelled"));
    }

    #[test]
    fn test_renders_multiple_orders() {
        let tmpl = OrdersTableTemplate {
            orders: vec![
                make_order("Order A", "2024-01-01", "#111", OrderStatus::Success),
                make_order("Order B", "2024-02-01", "#222", OrderStatus::Cancelled),
                make_order("Order C", "2024-03-01", "#333", OrderStatus::Success),
            ],
        };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("Order A"));
        assert!(html.contains("Order B"));
        assert!(html.contains("Order C"));
    }

    // --- OrderTagCell rendering ---

    #[test]
    fn test_order_tag_cell_renders_success() {
        let order = make_order("Item", "2024-06-01", "#fff", OrderStatus::Success);
        let tmpl = OrderTagCell { order: &order };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("badge-status success"));
        assert!(html.contains("Success"));
    }

    #[test]
    fn test_order_tag_cell_renders_cancelled() {
        let order = make_order("Item", "2024-06-01", "#fff", OrderStatus::Cancelled);
        let tmpl = OrderTagCell { order: &order };
        let html = tmpl.render().expect("template should render");
        assert!(html.contains("badge-status cancelled"));
        assert!(html.contains("Cancelled"));
    }
}
