//Usings, Mods, Crates, Macros
use_expansion_serenity!();

//Mods
use crate::src_bot::
{
    bot::
    {
        bot_commands::
        {
            math::*,
            meta::*,
            owner::*,
            admin::*,
            normal::*,
            bot_message_handlers,
        },
        bot_core::
        {
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

pub struct CommandCounter;
impl TypeMapKey for CommandCounter
{
    type Value = HashMap<String, u64>;
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

#[group("general")]
#[commands(ping, pong, cg_birth, ask_for_game, shard, quit)]
struct General;

#[group]
#[commands(about, say, commands, some_long_command)]
struct Common;

#[group]
// Sets multiple prefixes for a group.
// This requires us to call commands in this group
// via `~emoji` (or `~em`) instead of just `~`.
#[prefixes("emoji", "em")]
// Set a description to appear if a user wants to display a single group
// e.g. via help using the group-name or one of its prefixes.
#[description = "A group with commands providing an emoji as response."]
// Sets a command that will be executed if only a group-prefix was passed.
#[default_command(bird)]
#[commands(cat, dog)]
struct Emoji;

#[group]
// Sets a single prefix for this group.
// So one has to call commands in this group
// via `~math` instead of just `~`.
#[prefix = "math"]
#[commands(multiply)]
struct Math;

#[group]
#[owners_only]
#[only_in(guilds)] // Limit all commands to be guild-restricted.
#[checks(Admin)] // Adds checks that need to be passed.
#[commands(am_i_admin, slow_mode)]
struct Owner;

// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip =
"Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
// If the `String` is not empty, your given `String` will be used instead.
// If you pass in a `None`, no hint will be displayed at all.
fn my_help(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult 
{
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
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

    client.with_framework(StandardFramework::new()
        .configure(|c| c //create config.
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("!w_") //Prefix for all commands.
            .delimiters(vec![", ", ","]) //Delimeters see doc.
            .owners(owners)) //Set bots owners.
        .before(|ctx, msg, command_name| //Functions executed before command execution.
        {
            info!("Got command '{}' by user '{}'", command_name, msg.author.name);
            //Increment the number of times this command has been run once.
            let mut data = ctx.data.write();
            let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");
            let entry = counter.entry(command_name.to_string()).or_insert(0);
            *entry += 1;
            true //If `before` returns false, command processing doesn't happen.
        })
        .after(|_, _, command_name, error| //Functions executed before command execution. 
        {
            match error 
            {
                Ok(()) => info!("Processed command '{}'", command_name),
                Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        .unrecognised_command(|_, _, unknown_command_name|  //Set a function that's called whenever an attempted command-call's command could not be found.
        {
            println!("Could not find command named '{}'", unknown_command_name);
        })
        .normal_message(|_, message|  //Set a function that's called whenever a message is not a command.
        {
            println!("Message is not a command '{}'", message.content);
        })
        .on_dispatch_error(|ctx, msg, error| //Set a function that's called whenever a command's execution didn't complete for any reason.
        {
            if let DispatchError::Ratelimited(seconds) = error 
            {
                let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", seconds));
            }
        })
        .help(&MY_HELP) //Set a help function.
        .bucket("emoji", |b| b.delay(5)) //Functions that can't be used more then once per 5 seconds.
        .bucket("complicated", |b| b.delay(5).time_span(30).limit(2)) //Max 2 times per 30 seconds, with a 5 second delay.
        .group(&GENERAL_GROUP)
        .group(&EMOJI_GROUP)
        .group(&MATH_GROUP)
        .group(&OWNER_GROUP)
        .group(&COMMON_GROUP));

    // Start listening for events by starting a single shard
    if let Err(why) = client.start()
    {
        error!("Client error: {:?}", why);
    }
}

fn shard(client:&serenity::client::Client)
{
    let mut data = client.data.write();
    data.insert::<CommandCounter>(HashMap::default());
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