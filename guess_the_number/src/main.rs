use std::io;
use rand::prelude::*;

fn main() {
    guess_number();
}


fn guess_number(){
    println!("Guess the number");
    
    let mut rng = rand::rng();
    let secret_number: i32 = rng.random_range(1..10);


    println!("Please input your guess.");
    //println!("Debug: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
    let guess = guess.trim();
    match guess.parse::<i32>(){
        Ok(guess)=> {
            if guess == secret_number {
                println!("YOU GUESSED RIGHT!!!")
            } else {
                println!("You guessed wrong, the number is {}", secret_number)
            };
        },
        Err(_) => println!("Type a number!")
    }
}