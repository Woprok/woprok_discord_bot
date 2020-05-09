//Mods
use crate::bot_commands;
use crate::bot;

//Usings
use std::
{
    collections::HashSet,
    sync::Arc,
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
use log::{error, info};

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
        if msg.content == "!w_pong"
        {
            println!("Shard {}", ctx.shard_id);
            if let Err(why) = msg.channel_id.say(&ctx.http, "Ping From Primary Location!") 
            {
                println!("Error sending message: {:?}", why);                
            }           
        }
    }

    fn ready(&self, _: Context, ready: Ready) 
    {
        info!("Connected as {}", ready.user.name);
        bot::bot_personality::present_yourself();
    }

    fn resume(&self, _: Context, _: ResumedEvent) 
    {
        info!("Resumed");
    }
}
#[group("general")]
#[commands(multiply, ping, pong, quit)]
struct General;

//Methods

pub fn create_bot(token:&str)
{    
    // Login with a bot token from the environment
    let mut client = Client::new(&token, Handler)
                             .expect("Error creating client");
                            {
                             let mut data = client.data.write();
                             data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
                            }
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