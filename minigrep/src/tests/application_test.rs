#[cfg(test)]
mod tests {
    use crate::models::application::Application;
    use crate::models::config::Config;

    #[test]
    fn application_returns_true() {
        let config = Config {
            query: String::from("duct"),
            file_path: String::from("tests/poem.txt"),
        };

        let result = Application::default().run(&config);

        assert_eq!(result.unwrap(), true);
    }
}
