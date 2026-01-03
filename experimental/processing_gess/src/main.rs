// "use" represents the same as require or import on javascript
// In this case, we import the io module and this is usefull to get input from the user
// and output to the user
use std::io;

use rand::Rng;

fn main() {
    let min_number: i32 = 1;
    let max_number: i32 = 50;
    // generamos un numero aleatorio entre min = 1 hasta max = 50
    let secret_number: i32 = rand::rng().random_range(min_number..=max_number);

    // "mut" means that the variable is mutable and it can be changed
    // "mut" significa que es una variable dispuesta a cambiar en el tiempo
    let mut guess: String = String::new();
    println!("sectret number: {}", secret_number);

    loop {
        println!("Guess a number!");
        println!("Plese input your guess.");
        // "read_line" is a function from the io module
        io::stdin()
            // Assigns the input to the variable guess
            .read_line(&mut guess)
            // "expect" is a function from the io module
            // - Cases the error and returns the error message:
            //      - 1. The standard input is closed or innacessible
            //      - 2. Memory allocation fails
            //      - 3. IO permissions are denied
            //      - 4. Error in the peripherical device
            .expect("Failed to read line");

        // we can cast the number (i32) to a string with the format macro
        if guess.trim() == format!("{}", secret_number) {
            println!("You win!");
            break;
        }
        println!("You fail, try again or press CTRL+c to exit!");
        // we must clear the variable
        guess = String::new();
    }
}
