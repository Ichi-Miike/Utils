use std::io;
use arboard::Clipboard;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    copy_to_clipboard(input);
}

fn copy_to_clipboard(input: String) {

    let mut clipboard = Clipboard::new().unwrap(); 

    let result = clipboard.set_text(&input);
    
    match result {
        Ok(_value) => println!("Value '{}' written to clipboard", input.trim()),
        Err(e) => eprintln!("Error: {}", e)
    }
}
