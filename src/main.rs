//Mods
mod bot;
mod other;

//Usings

const BOT_NAME:&str = "Woprok";
const BOT_LUCKY_NUMBER:u32 = 6_7_1996;

fn main()
{
    println!("I am Rust Bot a.k.a {}. Smart and Edgy! My lucky number is: {} :)", BOT_NAME, BOT_LUCKY_NUMBER);
    if true 
    {
        println!("Execution: Setting up environment.");
        bot::environment_methods::load_environment_variables();
        println!("Execution: Starting bot.");
        bot::example_bot_1_1::example_bot_1_1_main();
        println!("Execution: Bot is dead!");
    }
    else 
    {
        slay_johnny_announcer();
        rise_of_weebs();
        other::guess_game::guess_game();
        bot::example_bot_1::example_bot_1_main();
    }
}

fn slay_johnny_announcer()
{
    println!("I have come to slay Johnny Pythagoras and claim his place!");
}

fn rise_of_weebs()
{
    println!("Join us, you can't run away!");
}