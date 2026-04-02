use std::io::{self, Write};

pub fn clear_console() {
    // Clear screeen and set cursor to home position (0, 0)
    print!("\x1B[2J\x1B[H"); 
    io::stdout().flush().unwrap();
}

