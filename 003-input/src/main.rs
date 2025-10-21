use std::io; // Import the standard input/output library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // Create a mutable String variable to hold the user input

    io::stdin() // Get standard input handle. That function returns an instance of Stdin
        .read_line(&mut guess) // Read a line from standard input and append it to 'guess'. The passed reference must be mutable
        .expect("Failed to read line"); // Handle potential errors

    println!("You guessed: {guess}");
}
