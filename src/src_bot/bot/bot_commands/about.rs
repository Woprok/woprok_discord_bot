//Usings, Mods, Crates, Macros...
use_expansion_serenity!();
use chrono::Local;

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
fn about(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, 
        format!("I am incarnation of Chaos!\nMy version is: {}",
        env!("CARGO_PKG_VERSION")))?; //This returns cargo version.
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