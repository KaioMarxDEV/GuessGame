use std::cmp::Ordering;
use::std::io;
use::rand::Rng;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);
    
    loop {
        println!("Guess the number!");
        println!("Type a number: ");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input need to be type number");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    println!("program finished!")
}