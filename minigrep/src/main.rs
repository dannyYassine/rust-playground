use std::process;

use minigrep::console::Console;

fn main() -> Result<(), &'static str> {
    if let Err(message) = Console::default().run() {
        println!("{}", message);
        process::exit(1);
    }
    Ok(())
}
