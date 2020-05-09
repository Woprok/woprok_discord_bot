//Mods
mod bot;
mod bot_commands;
mod other;

//Usings

fn main()
{
    println!("Execution: Setting up environment.");
    other::environment_methods::load_environment_variables();
    other::environment_methods::initialize_logger();

    println!("Execution: Creating bot.");
    bot::bot_main::create_bot(&other::environment_methods::get_variable(other::environment_methods::DISCORD_TOKEN_KEY));

    if true 
    {
        println!("Execution: Starting bot.");
        
        println!("Execution: Bot is dead!");
    }
    else 
    {
        slay_johnny_announcer();
        rise_of_weebs();
        other::guess_game::guess_game();
        
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