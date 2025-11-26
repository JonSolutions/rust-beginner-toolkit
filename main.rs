// Import the io module from the standard library for input/output operations  
use std::io;

// The main function - entry point of every Rust program  
fn main() {  
    // Print a welcome message  
    println!("=== Welcome to Rust! ===");  
    println!("Let's create a personalized greeting.\n");  
      
    // Get user's name  
    println!("What's your name?");  
    let name = get_user_input();  
      
    // Get user's age  
    println!("\nHow old are you?");  
    let age_input = get_user_input();  
      
    // Parse age from string to number, with error handling  
    let age: u32 = match age_input.trim().parse() {  
        Ok(num) => num,  
        Err(_) => {  
            println!("That doesn't look like a valid age. Using 0 instead.");  
            0  
        }  
    };  
      
    // Generate and display personalized greeting  
    let greeting = create_greeting(&name, age);  
    println!("\n{}", greeting);  
      
    // Display some fun facts about the user's age  
    display_age_category(age);  
}

// Function to read user input from the terminal  
// Returns a String containing the user's input  
fn get_user_input() -> String {  
    let mut input = String::new(); // Create a mutable String to store input  
      
    // Read a line from stdin into our string  
    io::stdin()  
        .read_line(&mut input)  
        .expect("Failed to read input"); // Handle potential errors  
      
    input.trim().to_string() // Remove whitespace and return  
}

// Function to create a personalized greeting  
// Takes a string reference (borrowed) and age as parameters  
fn create_greeting(name: &str, age: u32) -> String {  
    if name.is_empty() {  
        return String::from("Hello, mysterious stranger! 👋");  
    }  
      
    // Format a greeting message  
    format!("Hello, {}! At {} years old, you're doing great! 🎉", name, age)  
}

// Function to display age-related information  
fn display_age_category(age: u32) {  
    println!("\n--- Age Insights ---");  
      
    // Use match expression for pattern matching  
    match age {  
        0 => println!("Age not specified or invalid."),  
        1..=12 => println!("You're in the childhood phase! 🧒"),  
        13..=19 => println!("Teenage years - enjoy them! 🎸"),  
        20..=35 => println!("Young adult - prime time! 💪"),  
        36..=60 => println!("Experienced and wise! 🎓"),  
        _ => println!("Age is just a number! ✨"),  
    }  
      
    // Calculate years until next milestone  
    if age < 100 {  
        let years_to_100 = 100 - age;  
        println!("You have {} years until your 100th birthday!", years_to_100);  
    } else {  
        println!("Congratulations on reaching 100+! 🎂");  
    }  
}
