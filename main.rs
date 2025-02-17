use std::io::{self, Write}; // Import Write for flushing
use argon2::{Argon2, PasswordHasher, PasswordVerifier, Params, password_hash::{SaltString, rand_core::OsRng, PasswordHash}}; // Import Argon2 for hashing, verification, and salting

fn main() {
    // Store the returned password
    let master_password = get_pass(); 
    
    // Store the hashed password
    let hashed_brown = hashing_password(&master_password);

    // Verify password
    if check_password(&hashed_brown) {
        println!("Password is correct!");
    } else {
        println!("Incorrect password!");
    }
}

// Function to get the master password
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

// Function to hash the master password
fn hashing_password(master_password: &str) -> String {
    // Generate salt using OS random generator
    let salt = SaltString::generate(&mut OsRng);

    // Setting parameters for hashing: 64MB memory, 6 iterations, 2 parallelism
    let params = Params::new(65536, 6, 2, None).expect("Invalid Argon2 params"); 
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    //hashign password
    let hashed_password = argon2.hash_password(master_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    // Return hashed password
    hashed_password 
}

// Function to verify password
fn check_password(hashed_brown: &str) -> bool {
    // Ask the user to enter the password again
    let entered_password = get_pass(); // Get password input again

    // Parse the stored hash
    let parsed_hash = PasswordHash::new(hashed_brown).expect("Invalid hash format");

    // Use Argon2 to verify the password
    let argon2 = Argon2::default();

    argon2.verify_password(entered_password.as_bytes(), &parsed_hash).is_ok()
}
