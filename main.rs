use std::io::{self, Write}; // Import Write for flushing
use argon2::{Argon2, PasswordHasher, Params, password_hash::{SaltString, rand_core::OsRng}}; // import argon2 for hashing, verification and salting

fn main() {
    // Store the returned password
    let master_password = get_pass(); 

    let hashed_brown = hashing_password(&master_password);
    println!("Your hash brown is ready!: {}", hashed_brown)
}

// Defining function to get the master password
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

//defining function that will hash and return the master password
fn hashing_password(master_password: &str) -> String {
    //generating salt using OS random generator
    let salt = SaltString::generate(&mut OsRng);

    // Setting parameters for hashing: 64mb memory, 2 core and 4 threads used
    let params = Params::new(65536, 6, 2, None).expect("Invalid Argon2 params"); 
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let hashed_password = argon2.hash_password(master_password.as_bytes(), &salt)
    .expect("Failed to hash password")
    .to_string();

    //return hashed password
    hashed_password
}
