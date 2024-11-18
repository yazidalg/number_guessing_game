use rand::Rng;
use std::io;

const MAX_ATTEMPTS: i32 = 5;
fn main() {
    println!("Number Guessing Game");
    let random_number = generate_random_number();
    let mut attempts = 0;

    loop {
        let user_input = read_input();
        let text = compare_numbers(user_input, random_number);
        println!("{}", text);
        attempts += 1;

        if attempts == MAX_ATTEMPTS {
            println!("You lose! the number was: {}", random_number);
            break;
        }

        if user_input == random_number {
            break;
        }
    }
}

fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=10)
}

fn read_input() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please type a number")
}

fn compare_numbers(user_input: u32, random_number: u32) -> String {
    if user_input > random_number {
        return String::from("Too High!");
    } else if user_input < random_number {
        return String::from("Too Low!");
    } else {
        return String::from("You Win!");
    }
}
