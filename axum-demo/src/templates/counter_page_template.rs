use askama::Template;
use askama_web::WebTemplate;
use component::Component;
use component::filters;

use super::CalendarPopoverTemplate;

#[derive(Template, WebTemplate, Component)]
#[template(path = "counter.html")]
#[component(css_path = "static/counter.css", js_path = "static/counter.js")]
pub struct CounterPageTemplate {
    pub count: isize,
}

impl CounterPageTemplate {
    pub fn new(count: isize) -> Self {
        Self { count }
    }

    pub fn calendar_popover(&self) -> CalendarPopoverTemplate {
        CalendarPopoverTemplate::new()
    }
}
