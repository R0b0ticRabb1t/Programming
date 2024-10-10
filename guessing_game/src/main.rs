use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number!");

        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line for some reason");
        println!("You guessed:{}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("INVALID:: ENTER A VALID NUMBER NEXT TIME >:");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("CORRECTO!!"); 
                break;
            }
        }
    }
}