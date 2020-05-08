//this one should be refactored here
use crate::bot::bot_token_constants::DISCORD_TOKEN_KEY;
//Obtain access to environment variables for bot auth.
use crate::bot::bot_token_constants::DISCORD_TOKEN_VALUE;

use std::env;

pub fn set_environment_variables()
{
    env::set_var(DISCORD_TOKEN_KEY, DISCORD_TOKEN_VALUE);
    assert_eq!(env::var(DISCORD_TOKEN_KEY), Ok(DISCORD_TOKEN_VALUE.to_string()));    
}