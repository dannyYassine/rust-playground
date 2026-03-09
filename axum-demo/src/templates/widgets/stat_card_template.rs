use askama::Template;

#[derive(Template)]
#[template(path = "partials/stat_card.html")]
pub struct StatCardTemplate {
    pub label: String,
    pub value: String,
    pub icon_color: String,
    pub icon_svg: String,
    pub bar_pct: u8,
    pub range_min: String,
    pub range_max: String,
}
