use askama::Template;
use askama_web::WebTemplate;
use component::Component;

#[derive(Template, WebTemplate, Component)]
#[template(
    source = "
    <div id=\"counter\" class=\"text-6xl font-bold min-w-20 text-center\" hx-swap-oob=\"true\">{{ count }}</div>
    ",
    ext = "html"
)]
pub struct CounterTemplate {
    pub count: isize,
}
