
// To obtain user input and then print the result as output, 
// we need to bring the io input/output library into scope. 
// The io library 
// comes from the standard library, known as std:

use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    // get input guess
    // We use the let statement to create the variable
    // let apples = 5;
    // This line creates a new variable named apples and binds it to the value 5
    // o make a variable mutable, we add mut before the variable name:
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
}
