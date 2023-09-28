use prime_test::is_prime;
use std::io;

fn main() {
    println!("Test if a number is prime!");

    loop {
        println!("Please input your number.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", input);

        if is_prime(input) {
            println!("{} is a prime!\n", input);
        } else {
            println!("{} is not a prime\n", input);
        }
    }
}
