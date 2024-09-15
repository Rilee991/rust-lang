use std::io::{self};

use rand::Rng;

// Building a guessing game in rust
pub fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut moves: i32 = 0;

    // println!("Secret: {}", secret_number);

    loop {
        println!("Enter your number");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Not able to take input");
        let user_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Invalid input... {}", err);
                continue;
            }
        };

        println!("You gueessed {}", user_num);
        moves += 1;

        match user_num.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Guessed too low");
            },
            std::cmp::Ordering::Greater => {
                println!("Guessed too high!");
            },
            std::cmp::Ordering::Equal => {
                println!("You win in {} moves. What a loser... lmao", moves);
                break;
            }
        }
    }
}
