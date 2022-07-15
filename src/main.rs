use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the human binary search game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 1;
    loop {
        println!("Enter number between 1-100");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => { tries += 1; println!("Too small!");},
            Ordering::Greater => { tries +=1; println!("Too big!");},
            Ordering::Equal => { println!("You win! Took {tries} tries"); break;},
        };
    }
}
