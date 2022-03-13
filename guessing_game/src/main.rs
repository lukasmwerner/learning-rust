use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let rand_num = rand::thread_rng().gen_range(1..101);

    println!("Guess a number!");

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Guess: {}", guess);

        let guess: u32 = guess.trim().parse().expect("Not a number");
        
        match guess.cmp(&rand_num) {
            Ordering::Equal => break,
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
        
    }
    println!("You guessed right!");

    
}
