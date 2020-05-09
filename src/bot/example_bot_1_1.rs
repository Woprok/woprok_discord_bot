use std::env;

use serenity::
{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler 
{
    // Set a handler for the `message` event - so that whenever a new message is received - the closure (or function) passed will be called.
    // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) 
    {
        if msg.content == "!w_ping" 
        {
            println!("Shard {}", ctx.shard_id);
            // Sending a message can fail, due to a network error, an authentication error, or lack of permissions to post in the channel, so log to stdout when some error happens, with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") 
            {
                println!("Error sending message: {:?}", why);
            }
        }
        // Serenity implements transparent sharding in a way that you do not need to manually handle separate processes or connections manually.
        // Transparent sharding is useful for a shared cache. Instead of having caches with duplicated data, a shared cache means all your data can be easily accessible across all shards.
        // If your bot is on many guilds - or over the maximum of 2500 - then you should/must use guild sharding.
        // This is an example file showing how guild sharding works. For this to properly be able to be seen in effect, your bot should be in at least 2 guilds.
        // Taking a scenario of 2 guilds, try saying "!ping" in one guild. It should print either "0" or "1" in the console. Saying "!ping" in the other guild, it should cache the other number in the console. This confirms that guild sharding works.
        if msg.content == "!w_pong"
        {
            println!("Shard {}", ctx.shard_id);
            if let Err(why) = msg.channel_id.say(&ctx.http, "Ping!") 
            {
                println!("Error sending message: {:?}", why);                
            }           
        }
    }

    // Set a handler to be called on the `ready` event. 
    // This is called when a shard is booted, and a READY payload is sent by Discord. 
    // This payload contains data like the current user's guild Ids, current user data, private channels, and more.
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) 
    {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn example_bot_1_1_main() 
{
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
                    .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. 
    // This will automatically prepend your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::new(&token, Handler)
                            .expect("Err creating client");

    // Shards will automatically attempt to reconnect, and will perform exponential backoff until it reconnects.
    // The total number of shards to use. The "current shard number" of a shard - that is, the shard it is assigned to - is indexed at 0, while the total shard count is indexed at 1.
    // This means if you have 5 shards, your total shard count will be 5, while each shard will be assigned numbers 0 through 4.
    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start() //client.start_shards(x) 
    {
        println!("Client error: {:?}", why);
    }
}