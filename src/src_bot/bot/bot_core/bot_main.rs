//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use std::time::Instant;
use serde::de;

//Mods
use crate::src_bot::
{
    bot::
    {
        bot_commands::
        {
            bot_message_handlers,
            voice as CommandVoice,
            calendar,
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
    process,
    fs,
    fmt::Write,
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

    // We want to dispatch an event whenever a new reaction has been added.
    fn reaction_add(&self, context: Context, reaction: Reaction) 
    {
        let dispatcher = 
        {
            let mut context = context.data.write();
            context.get_mut::<calendar::DispatcherKey>().expect("Expected Dispatcher.").clone()
        };
        dispatcher.write().dispatch_event(&calendar::DispatchEvent::ReactEvent(reaction.message_id, reaction.user_id));
    }
}

//Methods

pub fn create_bot(token:&str, shard_count:u64)
{    
    // Login with a bot token from the environment
    let mut client = Client::new(&token, Handler)
                             .expect("Error creating client");
    shard(&client);
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() 
    {
        Ok(info) => //Fetch bot id and owner
        {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(bot_framework::construct_framework(owners, bot_id));

    info_shard_state(&client);

    if shard_count == 1 // Start listening for events by starting a single shard
    {
        if let Err(why) = client.start()
        {
            error!("Client error: {:?}", why);
        }
    }
    else // Start two shards. Note that there is an ~5 second ratelimit period between when one shard can start after another.
    {
        if let Err(why) = client.start_shards(shard_count) 
        {
            error!("Client error: {:?}", why);
        }
    }

    // Write server-local prefixes to a file.
    {
        let data = client.data.read();
        if let Some(prefixes) = data.get::<bot_framework::Prefixes>() {
            match kankyo::key("PREFIX_FILE") {
                Some(file) => {
                    match fs::write(file, serde_json::to_string(&*prefixes.read()).unwrap()) {
                        Ok(_) => info!("Prefix file successfully written"),
                        Err(e) => error!("Problem writing prefix file: {:?}", e),
                    }
                }
                None => error!("Expected a prefix file in the environment"),
            }
        }
    }
}

fn shard(client:&serenity::client::Client)
{
    let mut data = client.data.write();
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<bot_framework::CommandCounter>(HashMap::default());
    data.insert::<CommandVoice::VoiceManager>(Arc::clone(&client.voice_manager));
    data.insert::<calendar::DispatcherKey>(Arc::new(RwLock::new(calendar::get_dispatcher(4))));
    data.insert::<calendar::SchedulerKey>( Arc::new(RwLock::new(calendar::get_scheduler(4))));
    data.insert::<bot_framework::DefaultPrefix>(match kankyo::key("PREFIX") 
    {
        Some(prefix) => prefix,
        None => 
        {
            error!("Expected a default bot prefix in the environment");
            process::exit(1);
        }
    });
    data.insert::<bot_framework::PermissionsContainer>(
        match kankyo::key("PERMS").and_then(|p| p.parse().ok()) 
        {
            Some(p) => Permissions::from_bits_truncate(p),
            None => Permissions::empty(),
        },
    );
    data.insert::<bot_framework::Prefixes>(Arc::new(RwLock::new(
        {
            match kankyo::key("PREFIX_FILE") 
            {
                Some(file) => fs::read_to_string(file)
                    .map_err(de::Error::custom)
                    .and_then(|contents| serde_json::from_str(&contents))
                    .unwrap_or_else(|e| {
                        warn!("Problem reading prefix file: {:?}", e);
                        HashMap::new()
                    }),
                None => 
                {
                    error!("Expected a prefix file in the environment");
                    process::exit(1);
                }
            }
        })));
    data.insert::<bot_framework::StartTime>(Instant::now());
}

fn info_shard_state(client:&serenity::client::Client)
{
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
}