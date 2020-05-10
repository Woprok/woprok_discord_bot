extern crate chrono;
extern crate fern;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod src_bot;
mod src_other;

//Usings

fn main()
{
    src_other::environment_methods::load_environment_variables();
    let config = src_other::environment_methods::load_config_variables("resources/config.json").unwrap();
    //src_other::environment_methods::initialize_logger();
    src_other::environment_methods::setup_log_system(config.logging_level).unwrap();
    info!("Execution: Setting up configurable environment finished.");

    info!("Execution: Creating bot.");
    src_bot::bot::bot_core::bot_main::create_bot(&src_other::environment_methods::get_variable(src_other::environment_methods::DISCORD_TOKEN_KEY));

    if true 
    {        
        info!("Execution: Bot is dead!");
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