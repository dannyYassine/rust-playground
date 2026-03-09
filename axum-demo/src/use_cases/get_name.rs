use component::Injectable;

#[derive(Injectable)]
pub struct GetNameUseCase {}

impl GetNameUseCase {
    pub async fn execute(&self, name: Option<String>) -> String {
        name.unwrap_or("Your name will appear here.".to_string())
    }
}
