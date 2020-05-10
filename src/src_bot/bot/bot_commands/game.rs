//Usings, Mods, Crates, Macros...
use_expansion_serenity!();

#[group("game")]
#[commands(set_activity)]
pub struct Game;

//Methods...
#[command]
fn set_activity(ctx:&mut Context, msg:&Message, args:Args) -> CommandResult 
{
    let name = args.rest();
    ctx.set_presence(Some(Activity::playing(&name)), OnlineStatus::Idle);
    msg.channel_id.say(&ctx.http, format!("Set current playing message to: {}", &name))?;
    Ok(())
}