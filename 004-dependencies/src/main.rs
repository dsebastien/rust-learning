use std::io; // Import the standard input/output library
use rand::Rng; // Import the random number generation library


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // Create a mutable String variable to hold the user input

    io::stdin() // Get standard input handle. That function returns an instance of Stdin
        .read_line(&mut guess) // Read a line from standard input and append it to 'guess'. The passed reference must be mutable
        // read_line returns a Result type that indicates whether the operation was successful or not
        // it's an enum (https://doc.rust-lang.org/book/ch06-00-enums.html) with two variants: Ok and Err
        .expect("Failed to read line"); // The message to display if an error occurs
        // if the Result is an Err variant, the expect method will cause the program to crash and display the provided message
        // if it's an Ok variant, the expect method will return the value inside the Ok variant
        // in Java this would be similar to throwing an exception

    println!("You guessed: {guess}");
}
