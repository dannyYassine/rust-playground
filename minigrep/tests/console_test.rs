#[cfg(test)]
mod tests {
    use minigrep::console::Console;

    #[test]
    fn console_returns_true() {
        let command = "cargo run --";
        let query = "duct";
        let file_path = "tests/poem.txt";
        let args: Vec<String> = vec![
            String::from(command),
            String::from(query),
            String::from(file_path),
        ];

        let result = Console::new(args).run();

        assert_eq!(result.unwrap(), true);
    }
}
