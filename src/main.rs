//Usings
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("I am Rust Bot. Smart and Edgy!");
    slay_johnny_announcer();
    rise_of_weebs();
    guess_game();
}

fn slay_johnny_announcer()
{
    println!("I have come to slay Johnny Pythagoras and claim his place!");
}

fn rise_of_weebs()
{
    println!("Join us, you can't run away!");
}

fn guess_game()
{
    let secret_number = rand::thread_rng().gen_range(0,101);
    println!("The secret number is: {}", secret_number);
    println!("Guess the number!");
    loop
    {
        println!("Please input your guess: ");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("How dare you!?");
        let user_guess:u32 = match user_guess
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_) => {
                    if user_guess.trim() == "quit" 
                    {
                        println!("Weakling!");
                        break;
                    }
                    else if user_guess.trim() == "hint"
                    {
                        println!("I am nice, here is your number: {}", secret_number);
                        continue;
                    }
                    else
                    {
                        println!("Please don't try to break me! I can do that myself!");
                        continue;
                    }
                }
            };
        println!("You guessed: {}", user_guess);
        match user_guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Nope, you undermined me!"),
            Ordering::Greater => println!("Nope, you expect too much from me!"),
            Ordering::Equal =>
            {
                println!("You did it!");
                break;
            }        
        }
    }
}