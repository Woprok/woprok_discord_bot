//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use crate::src_bot::bot::bot_core::bot_framework;
use crate::src_bot::bot::bot_core::bot_main;
use std::fmt::Write;

#[group("normal")]
#[commands(latency, about_role, some_long_command, say, commands)]
pub struct Normal;

#[command]
fn latency(ctx: &mut Context, msg: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.read();

    let shard_manager = match data.get::<bot_main::ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(&ctx,  "No shard found");

            return Ok(());
        },
    };

    let _ = msg.reply(&ctx, &format!("The shard latency is {:?}", runner.latency));

    Ok(())
}


#[command]
// Limits the usage of this command to roles named:
#[allowed_roles("mods", "ultimate neko")]
fn about_role(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let potential_role_name = args.rest();

    if let Some(guild) = msg.guild(&ctx.cache) {
        // `role_by_name()` allows us to attempt attaining a reference to a role
        // via its name.
        if let Some(role) = guild.read().role_by_name(&potential_role_name) {
            if let Err(why) = msg.channel_id.say(&ctx.http, &format!("Role-ID: {}", role.id)) {
                println!("Error sending message: {:?}", why);
            }

            return Ok(());
        }
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Could not find role named: {:?}", potential_role_name)) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn some_long_command(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, &format!("Arguments: {:?}", args.rest())) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
// Repeats what the user passed as argument but ensures that user and role
// mentions are replaced with a safe textual alternative.
// In this example channel mentions are excluded via the `ContentSafeOptions`.
#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
       // By default roles, users, and channel mentions are cleaned.
       ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings);

    if let Err(why) = msg.channel_id.say(&ctx.http, &content) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
// Commands can be created via the attribute `#[command]` macro.
#[command]
// Options are passed via subsequent attributes.
// Make this command use the "complicated" bucket.
#[bucket = "complicated"]
fn commands(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read();
    let counter = data.get::<bot_framework::CommandCounter>().expect("Expected CommandCounter in ShareMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &contents) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}



