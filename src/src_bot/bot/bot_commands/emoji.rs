
//Usings, Mods, Crates, Macros
use_expansion_serenity!();

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
pub struct Emoji;

#[command]
fn bird(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult 
{
    let say_content = if args.is_empty() 
    {
        ":bird: can find animals for you.".to_string()
    } 
    else 
    {
        format!(":bird: could not find animal named: `{}`.", args.rest())
    };
    if let Err(why) = msg.channel_id.say(&ctx.http, say_content) 
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
#[description = "Sends an emoji with a dog."]
#[bucket = "emoji"]
fn dog(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, ":dog:")?;
    Ok(())
}

#[command]
#[aliases("kitty", "neko")] // Adds multiple aliases
#[bucket = "emoji"] // Make this command use the "emoji" bucket.
#[required_permissions("ADMINISTRATOR")] // Allow only administrators to call this:
fn cat(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, ":cat:")?; 
    Ok(())
}