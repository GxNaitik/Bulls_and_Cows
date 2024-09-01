use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Bull and Cows");

    let secret_number = rand::thread_rng().gen_range(1..11);

    let mut attempts = 0;
60

    loop {
        println!("Please input a number:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Ops! Something goes wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };

        attempts += 1;

        if guess < 1 || guess > 10 {
            println!("Please input a number between 1 and 10");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small!");
                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }
            Ordering::Greater => {
                println!("too big!");
                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }
            Ordering::Equal => {
                println!("Congratulation you're right!");
                println!("tips: you have tried {} times", attempts);
                break;
            }
        }

        if attempts == 10 {
            println!("You have tried 10 times, game over");
            break;
        }
    }
}