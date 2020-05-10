//Usings, Mods, Crates, Macros
use_expansion_serenity!();

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
// Adds multiple aliases
#[aliases("kitty", "neko")]
// Make this command use the "emoji" bucket.
#[bucket = "emoji"]
// Allow only administrators to call this:
#[required_permissions("ADMINISTRATOR")]
fn cat(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    if let Err(why) = msg.channel_id.say(&ctx.http, ":cat:") 
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
#[checks(Owner)]
fn ping_extra(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! : )") {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
// A function which acts as a "check", to determine whether to call a command.
//
// In this case, this command checks to ensure you are the owner of the message
// in order for the command to be executed. If the check fails, the command is
// not called.
#[check]
#[name = "Owner"]
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    // Replace 7 with your ID to make this check pass.
    //
    // `true` will convert into `CheckResult::Success`,
    //
    // `false` will convert into `CheckResult::Failure(Reason::Unknown)`,
    //
    // and if you want to pass a reason alongside failure you can do:
    // `CheckResult::new_user("Lacked admin permission.")`,
    //
    // if you want to mark it as something you want to log only:
    // `CheckResult::new_log("User lacked admin permission.")`,
    //
    // and if the check's failure origin is unknown you can mark it as such (same as using `false.into`):
    // `CheckResult::new_unknown()`
    (msg.author.id == 7).into()
}

// A function which acts as a "check", to determine whether to call a command.
//
// This check analyses whether a guild member permissions has
// administrator-permissions.
#[check]
#[name = "Admin"]
// Whether the check shall be tested in the help-system.
#[check_in_help(true)]
// Whether the check shall be displayed in the help-system.
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {

        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}