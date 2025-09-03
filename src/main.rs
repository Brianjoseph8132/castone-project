// Import the standard input/output library from Rust's standard library
use std::io;

fn main() {
    // Create a mutable (changeable) String to hold user input
    let mut name = String::new();

    // Print a prompt message for the user
    println!("Enter your name:");

    // Read a line from standard input (keyboard) into the `name` variable
    // `expect` will stop the program if an error happens while reading
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Print a greeting. `trim()` removes the newline character from input
    println!("Hello, {}!", name.trim());
}
