use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() 
{
    let mut lifes = 3;

    let secret_number :u32 = rand::thread_rng().gen_range(1..=10);
    println!("The number is {}", secret_number);

    loop
    {
        if lifes == 0 
        {
            print!("{}", "Looser".red());
            break;
        }
        println!("Guess a number between 1 and 10.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess :u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed : {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less =>
            {
                println!("{}", "Too small!".red());
                lifes -= 1;
                println!("{} lifes left", lifes);
            } 
            Ordering::Greater =>
            {
                println!("{}", "Too big!".red());
                lifes -= 1;
                println!("{} lifes left", lifes);
            } 
            Ordering::Equal =>
            {
                println!("{}", "You Win!".green());
                break;
            } 
        }

    }
    

}

