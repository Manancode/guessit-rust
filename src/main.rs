use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
    println!("Guess the number : ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read the line");

    let guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue ,
    };

    println!("you guessed : {}" , guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("too small"),
        Ordering::Equal => {println!("you won") ; break ;},
        Ordering::Greater => println!("too big"),
    }
    }
}
