//Usings
use std::env;

//Constants
pub const DISCORD_TOKEN_KEY:&str = "DISCORD_TOKEN";
pub const DISCORD_RUST_LOG_KEY:&str = "RUST_LOG";

//Methods
// This will load the environment variables located at `./.env`, relative to the CWD. See `./.env.example` for an example on how to structure this.
pub fn load_environment_variables()
{    
    kankyo::load(false)
            .expect("Failed to load .env file");
}

// Initialize the logger to use environment variables.
pub fn initialize_logger()
{
    println!("Initializing logger: {}", get_variable(&DISCORD_RUST_LOG_KEY));
    env_logger::init();
}

// Get specific variable
pub fn get_variable(desired_variable:&str) -> String
{
    env::var(&desired_variable)
         .expect("Expected a token in the environment")
}