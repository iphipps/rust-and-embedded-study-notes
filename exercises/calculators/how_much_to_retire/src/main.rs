use std::cmp::Ordering;
use std::io;

fn main() {
    println!("What have you saved?");
    let mut saved_amount = String::new();
    io::stdin()
        .read_line(&mut saved_amount)
        .expect("Failed to read the line");

    let saved_number = string_to_number(saved_amount);

    println!("What are your yearly expenses?");
    let mut saved_expenses = String::new();
    io::stdin()
        .read_line(&mut saved_expenses)
        .expect("Failed to read the line");
    let save_target = string_to_number(saved_expenses) * 25;
    let delta = save_target - saved_number;
    match save_target.cmp(&saved_number) {
        Ordering::Less => println!("Enjoy retirement"),
        Ordering::Greater => println!("Keep saving: you still need {}", delta),
        Ordering::Equal => println!("You have enough but maybe could have more"),
    }
}

fn string_to_number(user_entered_string: String) -> u32 {
    match user_entered_string.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
