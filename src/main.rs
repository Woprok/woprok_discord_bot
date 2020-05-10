//Mods
//mod bot;
//mod bot_commands;
//use crate::bot_mod;
mod src_bot;
mod src_other;

//Usings

fn main()
{
    println!("Execution: Setting up environment.");
    src_other::environment_methods::load_environment_variables();
    src_other::environment_methods::initialize_logger();

    println!("Execution: Creating bot.");
    src_bot::bot::bot_core::bot_main::create_bot(&src_other::environment_methods::get_variable(src_other::environment_methods::DISCORD_TOKEN_KEY));

    if true 
    {
        println!("Execution: Starting bot.");
        
        println!("Execution: Bot is dead!");
    }
    else 
    {
        slay_johnny_announcer();
        src_other::guess_game::guess_game();
        
    }
}

fn slay_johnny_announcer()
{
    println!("I have come to slay Johnny Pythagoras and claim his place!");
    println!("Join us, you can't run away!");
}