use std::env;

random

pub fn set_environment_variables()
{
    env::set_var(DISCORD_TOKEN_KEY, DISCORD_TOKEN_VALUE);
    assert_eq!(env::var(DISCORD_TOKEN_KEY), Ok(DISCORD_TOKEN_VALUE.to_string()));    
}