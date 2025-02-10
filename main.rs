use std::io::{self, Write}; // Import Write for flushing

fn main() {
    let master_password = get_pass(); // Store the returned password
    println!("You entered: {}", master_password);
}

fn get_pass() -> String {
    let mut password = String::new();

    // Print the prompt and flush to ensure it appears before input
    print!("Enter Your password: ");
    io::stdout().flush().unwrap();

    // Read user input
    io::stdin().read_line(&mut password).expect("Failed to read input");

    // Trim and return the password
    password.trim().to_string()
}
