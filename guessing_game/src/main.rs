use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u32 = 0;
    // println!("The secret number is {}", secret_number); // debug

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                attempts = attempts + 1;
                num
            },
            Err(_) => continue,
        };

        // println!("You guessed: {guess}");
        // println!("You have had {attempts} attempts");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win in {attempts} attempts!");
                break;
            }
        }
        
    }
}
