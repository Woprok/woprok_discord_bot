//Usings
use std::io;
use std::cmp::Ordering;
use rand::Rng;

const BOT_NAME:&str = "Woprok";
const BOT_LUCKY_NUMBER:u32 = 6_7_1996;

fn main()
{
    println!("I am Rust Bot a.k.a {}. Smart and Edgy! My lucky number is: {} :)", BOT_NAME, BOT_LUCKY_NUMBER);
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

fn guess_game()
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
                Err(_) => {
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