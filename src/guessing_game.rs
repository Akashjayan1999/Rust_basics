use std::io;
use Rand::rand;
use colored::*;

fn main(){
    loop {
    println!("Guess the number");
    println!("Please input your guess");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}","Too big!".red()),
        Ordering::Equal => {
            println!("{}","You win!".to_uppercase().green());
            break;
        },
    }
    }
    
}