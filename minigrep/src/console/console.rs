use crate::models::{Application, Config};
use std::env;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Console {
    args: Option<Vec<String>>,
}

impl Console {
    pub fn new(args: Vec<String>) -> Self {
        Console { args: Some(args) }
    }
    pub fn run(&self) -> Result<bool, &'static str> {
        let args: Vec<String> = self.args.clone().unwrap_or_else(|| env::args().collect());

        let ignore_case: String = env::var("IGNORE_CASE").ok().unwrap_or("false".into());
        if let Ok(value) = ignore_case.parse::<bool>() {
            println!("value: {}", value);
        }

        let config: Config = Config::new(&args)?;

        Application::default().run(&config)
    }
}
