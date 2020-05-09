//Usings
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::
{
    CommandResult,
    macros::command,
};

//Methods
#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let _ = msg.channel_id.say(&ctx.http, "Pong!");
    Ok(())
}

#[command]
fn pong(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Ping!")?;

    Ok(())
}