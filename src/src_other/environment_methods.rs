//Usings
use std::env;
use std::io::stdout;
use chrono::offset::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::{log_file, Dispatch, InitError};
use log::LevelFilter;
use log::info;
use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

//Constants
pub const DISCORD_TOKEN_KEY:&str = "DISCORD_TOKEN";
#[derive(Deserialize, Debug)]
pub struct Config {
    pub logging_level: u64,
    pub prefix: String,
}

//Methods
// This will load the environment variables located at `./.env`, relative to the CWD. See `./.env.example` for an example on how to structure this.
pub fn load_environment_variables()
{    
    kankyo::load(false).expect("Failed to load .env file");
}

pub fn load_config_variables<T: AsRef<Path>>(path: T) -> Result<Config, Box<dyn Error>> 
{
    let file = File::open(path)?;
    let out = from_reader(file)?;
    Ok(out)
}

// Get specific variable
pub fn get_variable(desired_variable:&str) -> String
{
    env::var(&desired_variable)
         .expect("Expected a token in the environment")
}

// Method for setting up a logger system using fern.
pub fn setup_log_system(verbosity: u64) -> Result<(), InitError> 
{
    let mut base_config = Dispatch::new(); // Create the base configuration for the system.
    let colors = ColoredLevelConfig::new() // Set all the colors for the different levels.
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue);   
    base_config = match verbosity // Change the output level from the variable.
    { 
        0 => base_config.level(LevelFilter::Info),
        1 => base_config.level(LevelFilter::Warn),
        2 => base_config.level(LevelFilter::Debug),
        3 => base_config.level(LevelFilter::Error),
        _ => base_config.level(LevelFilter::Info),
    };
    let logging_file = Dispatch::new() // Log the console output to a file called "log.log".
        .format(|out, message, record| 
            {
                out.finish(format_args!("{}[{}][{}] {}", Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(), record.level(), message))
            })
        .chain(log_file("./resources/log.log")?);
    let stdout_config = Dispatch::new() // Setup console output for logging.
        .format(move |out, message, record| 
            {
                out.finish(format_args!("{}[{}][{}] {}", Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(), colors.color(record.level()), message))
            })
        .chain(stdout());
    base_config // Chain everything together and build.
        .chain(logging_file)
        .chain(stdout_config)
        .apply()?;
    // Finish by displaying that the logging system finished.
    info!("Logging system finished setting up");
    Ok(())
}