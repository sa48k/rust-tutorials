use std::io;

fn main() {
    println!("Convert Celsius to Fahrenheit");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // let mut attempts: u32 = 0;
    // println!("The secret number is {}", secret_number); // debug

    loop {
        println!("\nEnter a temperature:");

        let mut temp = String::new();
        
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius: f32 = (temp - 32.0) / 1.8;
        let fahrenheit: f32 = temp * 1.8 + 32.0; 
        
        println!("{} Celsius is {} Fahrenheit", temp, fahrenheit.round());
        println!("{} Fahrenheit is {} Celsius", temp, celsius.round());
    }
}
