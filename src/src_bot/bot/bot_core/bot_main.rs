//Usings, Mods, Crates, Macros
use_expansion_serenity!();

//Mods
use crate::src_bot::
{
    bot::
    {
        bot_commands::
        {
            bot_message_handlers,
        },
        bot_core::
        {
            bot_framework,
        },
        bot_utils,
    }
};

//Usings
use std::
{
    collections::
    {
        HashMap,
        HashSet,
    },
    sync::Arc,
    thread,
    time::Duration,
    env,
    fmt::Write,
};
use log::
{
    debug, 
    error, 
    info
};

//Fields
// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
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
        bot_message_handlers::process_raw_message(&ctx, &msg);
        debug!("We avoided special commands crash.");
    }

    fn ready(&self, _: Context, ready: Ready) 
    {
        info!("Connected as {}", ready.user.name);
        bot_utils::bot_personality::present_yourself();
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

//Methods

pub fn create_bot(token:&str)
{    
    // Login with a bot token from the environment
    let mut client = Client::new(&token, Handler)
                             .expect("Error creating client");
    shard(&client);
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() 
    {
        Ok(info) => 
        {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(bot_framework::construct_framework(owners, bot_id));

    // Start listening for events by starting a single shard
    if let Err(why) = client.start()
    {
        error!("Client error: {:?}", why);
    }
}

fn shard(client:&serenity::client::Client)
{
    let mut data = client.data.write();
    data.insert::<bot_framework::CommandCounter>(HashMap::default());
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