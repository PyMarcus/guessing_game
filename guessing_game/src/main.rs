use std::io;  // from standard library import io thats use output and iput
use rand::Rng;
use std::cmp::Ordering;


fn main()
{

    loop{
            println!("Guess the number");
            println!("Please, input your guess");
            let mut guess = String::new();  // for default, variables are immutables
            // the new means that guess is a associate function for the String structure
            // by this, String::new() create a new instance of String type


            // the read_line receive the memo address of guess to apply the value received
            // in stdin, so the function has 2 result, ok or err, if err, so expect receive the error and print
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            println!("You Guessed: {guess}");


            // randomic number
            let secret_number = rand::thread_rng().gen_range(1..=100);  // [1, 100]
            println!("The secret number was {secret_number}");

            //convert string into a integer
            let guess: i32 = match guess.trim().parse(){
                Ok(number)=> number,
                Err(_) => continue,   // _ is a catchall
            };

            std::println!("Comparing {guess} with {secret_number}");

            // compare
            match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Higher"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }


}