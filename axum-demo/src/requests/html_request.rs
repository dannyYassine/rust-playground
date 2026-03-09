use serde::Deserialize;

#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct HtmlRequest {
    pub name: Option<String>,
}
