pub const DISCORD_TOKEN_KEY:&str = "DISCORD_TOKEN";
pub const DISCORD_RUST_LOG_KEY:&str = "RUST_LOG";

// This will load the environment variables located at `./.env`, relative to the CWD. See `./.env.example` for an example on how to structure this.
pub fn load_environment_variables()
{    
    kankyo::load()
            .expect("Failed to load .env file");
}

// Initialize the logger to use environment variables.
pub fn initialize_logger()
{
    env_logger::init();
}