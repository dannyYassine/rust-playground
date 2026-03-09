use askama::Template;
use askama_web::WebTemplate;
use component::Component;

#[derive(Template, WebTemplate, Component)]
#[template(path = "calendar_popover.html")]
#[component(css = "
    #cally-popover1 {
        background-color: red;
        border: 1px solid #ccc;
        border-radius: 4px;
        padding: 10px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        position: absolute;
        z-index: 1000;
    }
    ")]
pub struct CalendarPopoverTemplate;

impl CalendarPopoverTemplate {
    pub fn new() -> Self {
        Self {}
    }
}
