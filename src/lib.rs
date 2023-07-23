use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn type_text(text: &str) {
    let delay = 25;
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay));
    }
}

fn main() {}