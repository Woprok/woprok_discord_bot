//Usings
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Methods
// Implementation of simple guess_game.
pub fn guess_game()
{
    let secret_number = rand::thread_rng().gen_range(i32::min_value(), i32::max_value());
    println!("Guess the number!");
    loop
    {
        println!("Please input your guess: ");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("How dare you!?");

        let user_guess:i32 = match user_guess
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_) => 
                {
                    if guess_game_process_wrong_answer(user_guess, secret_number) == 1 
                    { 
                        break; 
                    } 
                    else 
                    {
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

// Process a non numerical answer.
fn guess_game_process_wrong_answer(guess:String, secret:i32) -> i32
{
    let trim_guess = guess.trim();
    if trim_guess == "quit" 
    {
        println!("Weakling!");
        1
    }
    else if trim_guess == "hint"
    {
        println!("I am nice, here is your number: {}", secret);
        0
    }
    else
    {
        println!("Please don't try to break me! I can do that myself!");
        0
    }
}