use std::io; //Standard library for taking input from user
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Enter a number");
        let mut input_guess = String::new();
        io::stdin()
            .read_line(&mut input_guess)
            .expect("Failed to read input");

        println!("You guessed {}",input_guess);

        //This step is to convert the string input into an integer
        let input_guess : i32 = match input_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            } 
        };

        match input_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too big")
        };
    }
    
}
