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
fn pong(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "Ping!")?;
    Ok(())
}

#[command]
fn cg_birth(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "Happy Birthday :cake:!")?;
    Ok(())
}

#[command]
fn ask_for_game(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "@everyone Ideme dnes niečo?")?;
    Ok(())
}

#[command]
fn shard(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, format!("My shard is: {}", ctx.shard_id))?;
    Ok(())
}