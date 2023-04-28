use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("GUESS THE NUMBER GAME");
    loop{
        let mut guess: String = String::new();
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                io::stdout().flush().unwrap();
                continue;
            },
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                                println!("You win!");
                                break;
                            },
            Ordering::Greater => println!("Too Big"),
        }
    }
}
