//Usings, Mods, Crates, Macros
use_expansion_serenity!();

#[group("meta")]
#[commands(cg_birth, ask_for_game, ask_for_chat)]
pub struct Meta;

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
fn ask_for_chat(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    msg.channel_id.say(&ctx.http, "@everyone Ideme niekto kecat do voice?")?;
    Ok(())
}