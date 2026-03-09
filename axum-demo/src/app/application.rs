use std::sync::Arc;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_service_provider(&mut self) -> &mut Self {
        return self;
    }

    pub async fn serve(self) -> () {}
}
