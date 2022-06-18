
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess a number");
    
    loop
    {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        }
    }

}