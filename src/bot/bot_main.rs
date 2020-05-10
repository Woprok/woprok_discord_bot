//Mods
use crate::bot_commands;
use crate::bot;

//Usings
use std::
{
    collections::HashSet,
    sync::Arc,
    thread,
    time::Duration
};
use serenity::
{
    client::bridge::gateway::ShardManager,
    framework::
    {
        StandardFramework,
        standard::macros::group,
    },
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use log::{debug, error, info};

use bot_commands::
{
    math::*,
    meta::*,
    owner::*,
};

//Fields
pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer 
{
    type Value = Arc<Mutex<ShardManager>>;
}
struct Handler;
impl EventHandler for Handler 
{
    fn message(&self, ctx: Context, msg: Message) 
    {
        info!("Executing special commands.");
        bot::bot_message_handlers::process_raw_message(&ctx, &msg);
        debug!("We avoided special commands crash.");
    }

    fn ready(&self, _: Context, ready: Ready) 
    {
        info!("Connected as {}", ready.user.name);
        bot::bot_personality::present_yourself();
        if let Some(shard) = ready.shard 
        {
            println!("{} is connected on shard {}/{}!", ready.user.name, shard[0], shard[1]);
        }
    }

    fn resume(&self, _: Context, resume: ResumedEvent) 
    {
        info!("Resumed");
        debug!("Resumed trace: {:?}", resume.trace);
    }
}
#[group("general")]
#[commands(multiply, ping, pong, cg_birth, ask_for_game, shard, quit)]
struct General;

//Methods

pub fn create_bot(token:&str)
{    
    // Login with a bot token from the environment
    let mut client = Client::new(&token, Handler)
                             .expect("Error creating client");
    shard(&client);
    let owners = match client.cache_and_http.http.get_current_application_info() 
    {
        Ok(info) => 
        {
            let mut set = HashSet::new();
            set.insert(info.owner.id);
            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
                                             .configure(|c| c
                                                .owners(owners)
                                                .prefix("!w_"))
                                             .group(&GENERAL_GROUP));

    // Start listening for events by starting a single shard
    if let Err(why) = client.start()
    {
        error!("Client error: {:?}", why);
    }
}

fn shard(client:&serenity::client::Client)
{
    let mut data = client.data.write();
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
}

// Example function for creating multiple shard bot.
#[allow(dead_code)]
pub fn create_multishard_bot(token:&str, shard_count:u64)
{
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Here we clone a lock to the Shard Manager, and then move it into a new
    // thread. The thread will unlock the manager and print shards' status on a
    // loop.
    let manager = client.shard_manager.clone();

    thread::spawn(move || 
    {
        loop 
        {
            thread::sleep(Duration::from_secs(30));
            let lock = manager.lock();
            let shard_runners = lock.runners.lock();
            for (id, runner) in shard_runners.iter() 
            {
                println!("Shard ID {} is {} with a latency of {:?}", id, runner.stage, runner.latency);
            }
        }
    });

    // Start two shards. Note that there is an ~5 second ratelimit period
    // between when one shard can start after another.
    if let Err(why) = client.start_shards(shard_count) 
    {
        println!("Client error: {:?}", why);
    }
}