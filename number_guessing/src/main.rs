use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Hello 👋");
    println!("Enter the number: ");
   loop {
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Invalid Input");
    let name: u32 = match name.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    match name.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Your number is smaller than the secret number!"),
        std::cmp::Ordering::Greater => println!("Your number is greater than the secret number!"),
        std::cmp::Ordering::Equal => {
            println!("You won the prize 🎉");
            break;
        }
    }
   }
}
