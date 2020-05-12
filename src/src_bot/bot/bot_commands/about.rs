//Usings, Mods, Crates, Macros...
use_expansion_serenity!();
use chrono::Local;
use crate::src_bot::bot::bot_core::bot_framework;
use crate::src_bot::bot::bot_utils::bot_helpers;

#[group("about")]
#[commands(ping, pong, shard, about, latency)]
pub struct About;

//Methods...
#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}
#[command]
fn pong(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "Ping!")?;
    Ok(())
}
#[command]
fn shard(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, format!("My shard is: {}", ctx.shard_id))?;
    Ok(())
}
#[command]
#[description("Get information about the bot.")]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, 
        format!("I am incarnation of Chaos!\nMy version is: {}",
        env!("CARGO_PKG_VERSION")))?; //This returns cargo version.
        
    let cache = ctx.cache.read();
    let data = ctx.data.read();

    // Get the number of users in all guilds.
    let users = cache.guilds.values().fold(0, |acc, guild| {
        let guild = guild.read();
        acc + guild.member_count
    });

    let uptime = data
        .get::<bot_framework::StartTime>()
        .map(|t| t.elapsed().as_secs().to_string())
        .unwrap_or("N/A".to_string());

    let invite_link = bot_helpers::invite_url(ctx)
        .map(|url| format!("\n[Invite me!]({})", url))
        .unwrap_or_else(|_| "".to_string());

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO)
                .title(&cache.user.name)
                .thumbnail(cache.user.face())
                .description(format!("I am incarnation of Chaos!\nMy version is: {}", env!("CARGO_PKG_VERSION")))
                .field(
                    "Info",
                    format!(
                        "I am currently on {} servers, serving {} users in total.\nI have been online for {} seconds.",
                        cache.guilds.len(),
                        users,
                        uptime
                    ),
                    false,
                )
                .field("Links",
                    format!("[GitHub](https://github.com/Woprok/woprok_discord_bot){}", invite_link),
                    false,
                )
        })
    })?;

    Ok(())
}
#[command]
fn latency(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let start = Local::now();
    let msg = msg.reply(&ctx.http,"0");
    let end = Local::now();
    if let Ok(mut m) = msg {
        let ms = end.signed_duration_since(start).num_milliseconds();
        let _ = m.edit(&ctx.http, |m| m.content(&format!("Detected latency: {} milliseconds", ms)));
    }
    Ok(())
}