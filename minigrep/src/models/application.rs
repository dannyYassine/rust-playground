use super::config::Config;
use super::helpers::search;
use std::fs;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Application {}

impl Application {
    pub fn run(&self, config: &Config) -> Result<bool, &'static str> {
        println!("Searching for {}", &config.query);
        println!("In file {}", &config.file_path);

        let contents = fs::read_to_string(&config.file_path).expect("Should be able to read");

        // println!("With text:\n{}", contents);

        for line in search(&config.query, &contents) {
            println!("{line}");
        }

        Ok(true)
    }
}
