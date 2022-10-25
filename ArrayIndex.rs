use std::io;

fn main() {
    let a = [1,2,3,4,5];
    
    loop {
        println!("Enter array index:");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Not a number");

        let element = a[index];

        println!("Value at index {} is {}", index, element)
    }
}