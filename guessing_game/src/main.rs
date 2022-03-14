use rand::Rng;
use std::cmp::Ordering;
use std::io;

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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&rand_num) {
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
    }
}
