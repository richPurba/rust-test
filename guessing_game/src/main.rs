use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);


    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // new() is an associated function, much like static method
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Please input your guess.");
        // shadowing.
        let guess: u32 = guess.trim().parse().expect("Please type a number");
        println!("You guessed: {}", guess);

        ///
        /// it seems match is a construct where you can put enum and make your code exhaustively robust.
        /// Match consists of arms. An arm consists of a pattern that you need to match.
        /// In the below code, each arm has a pattern and that pattern should match to the argument given
        ///
        ///
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    // typing `quit` will end this infinite loop
    }



}
