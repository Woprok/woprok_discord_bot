//Usings, Mods, Crates, Macros
use_expansion_serenity!();

#[group("admin")]
#[commands(am_i_admin, slow_mode, ping_extra)]
pub struct Admin;

#[command]
fn am_i_admin(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    if let Err(why) = msg.channel_id.say(&ctx.http, "Yes you are.") 
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
fn slow_mode(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() 
    {
        if let Err(why) = msg.channel_id.edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds)) 
        {
            println!("Error setting channel's slow mode rate: {:?}", why);

            format!("Failed to set slow mode to `{}` seconds.", slow_mode_rate_seconds)
        } else 
        {
            format!("Successfully set slow mode rate to `{}` seconds.", slow_mode_rate_seconds)
        }
    } 
    else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache) 
    {
        format!("Current slow mode rate is `{}` seconds.", channel.read().slow_mode_rate.unwrap_or(0))
    } 
    else 
    {
        "Failed to find channel in cache.".to_string()
    };
    if let Err(why) = msg.channel_id.say(&ctx.http, say_content) 
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
// Limit command usage to guilds.
//#[only_in(guilds)]
//#[checks(Owner)]
fn ping_extra(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "Pong! : )")?;
    Ok(())
}