//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use crate::src_bot::bot;
use crate::src_bot::bot::bot_utils::bot_helpers;
use crate::src_bot::bot::bot_core::bot_framework;
use std::process;

#[group("owners")]
#[owners_only]
#[only_in(guilds)] // Limit all commands to be guild-restricted.
#[checks(Admin)] // Adds checks that need to be passed.
#[commands(quit, servers, nickname, prefix, invite, avatar)]
pub struct Owners;

//Methods
#[command]
#[owners_only]
#[required_permissions("ADMINISTRATOR")] // Allow only administrators to call this:
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let data = ctx.data.read();
    if let Some(manager) = data.get::<bot::bot_core::bot_main::ShardManagerContainer>() 
    {
        msg.reply(&ctx, "Shutting down! Nooooooooooooo")?;
        manager.lock().shutdown_all();
    } 
    else 
    {
        msg.reply(&ctx, "There was a problem getting the shard manager")?;
        return Ok(());
    }
    Ok(())
}


// A function which acts as a "check", to determine whether to call a command.
//
// In this case, this command checks to ensure you are the owner of the message
// in order for the command to be executed. If the check fails, the command is
// not called.
//#[check]
//#[name="owner"]
//fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult 
//{
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
//    (msg.author.id == 7).into()
//}



// A function which acts as a "check", to determine whether to call a command.
//
// This check analyses whether a guild member permissions has
// administrator-permissions.
#[check]
#[name="admin"]
#[check_in_help(true)] // Whether the check shall be tested in the help-system.
#[display_in_help(true)] // Whether the check shall be displayed in the help-system.
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) 
    {
        if let Ok(permissions) = member.permissions(&ctx.cache) 
        {
            return permissions.administrator().into();
        }
    }
    false.into()
}

#[command]
#[aliases(guilds)]
#[description("List all the servers the bot is currently in.")]
fn servers(ctx: &mut Context, msg: &Message) -> CommandResult {
    let cache = ctx.cache.read();

    // Get a vector of server names.
    let mut names: Vec<_> = cache
        .guilds
        .values()
        .map(|guild| guild.read().name.clone())
        .collect();
    names.sort_unstable();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO).title("Servers").description({
                let mut content = MessageBuilder::new();
                for name in &names {
                    content.push_line(name);
                }
                content.build()
            })
        })
    })?;

    Ok(())
}

#[command]
#[description("Edit the bot's nickname on a server. Resets the nickname if no arguments are provided.")]
#[only_in(guilds)]
#[usage("[name]")]
fn nickname(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult 
{
    // Reset the nickname if no args were provided.
    let name = if args.is_empty() 
    {
        None
    } 
    else 
    {
        Some(args.message())
    };
    let guild = msg.guild_id.unwrap();
    guild.edit_nickname(&ctx.http, name)?;
    Ok(())
}

#[command]
#[description("Change the bot's prefix on the current server. Resets if no arguments are provided.")]
#[only_in(guilds)]
#[usage("[prefix]")]
fn prefix(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    match data.get::<bot_framework::Prefixes>() {
        Some(prefixes) => {
            let mut prefixes = prefixes.write();
            if args.is_empty() {
                msg.channel_id.say(
                    &ctx.http,
                    format!(
                        "Reset prefix to `{}`.",
                        data.get::<bot_framework::DefaultPrefix>().unwrap_or_else(|| {
                            error!("Expected a default bot prefix in the environment");
                            process::exit(1);
                        })
                    ),
                )?;
                prefixes.remove(&msg.guild_id.unwrap());
            } else {
                msg.channel_id.say(
                    &ctx.http,
                    format!("Changed prefix to `{}`.", args.message()),
                )?;
                prefixes.insert(msg.guild_id.unwrap(), args.message().to_string());
            }
        }
        None => {
            error!("Problem accessing prefixes");
            process::exit(1);
        }
    }
    Ok(())
}

#[command]
#[description("Get the invite link for the bot.")]
fn invite(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, format!("<{}>", bot_helpers::invite_url(ctx)?))?;
    Ok(())
}

#[command]
#[description("Get a user's avatar. Gets your own avatar if no user is provided.")]
#[usage("[user]")]
fn avatar(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult 
{
    let user = if args.is_empty() 
    {
        Some(msg.author.clone())
    } 
    else if msg.mentions.is_empty() 
    {
        // If arguments are provided, try to find a matching user.
        msg.guild_id
            .and_then(|id| bot_helpers::find_user_in_guild(&ctx.cache.read(), id, args.message()))
            .and_then(|id| id.to_user(&ctx.http).ok())
    } 
    else 
    {
        // Get first mentioned user.
        msg.mentions.first().cloned()
    };

    let user = match &user 
    {
        Some(user) => user,
        None => 
        {
            msg.channel_id.say(&ctx.http, "User not found.")?;
            return Ok(());
        }
    };

    // Get the URL for the user's avatar.
    let mut url = user.face();
    let idx = url.find('?').unwrap_or(url.len());
    url.truncate(idx);

    // Get the nickname of the user if in a guild.
    let name = msg.guild_id.and_then(|id| user.nick_in(&ctx.http, id));

    // Send an embed containing the user's name and the avatar.
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO)
                .image(url)
                .title(name.as_ref().unwrap_or_else(|| &user.name))
        })
    })?;

    Ok(())
}