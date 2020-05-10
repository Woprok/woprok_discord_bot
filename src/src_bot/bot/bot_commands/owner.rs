//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use crate::src_bot::bot;

//Methods
#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let data = ctx.data.read();
    if let Some(manager) = data.get::<bot::bot_core::bot_main::ShardManagerContainer>() 
    {
        manager.lock().shutdown_all();
    } 
    else 
    {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }
    let _ = msg.reply(&ctx, "Shutting down!");
    Ok(())
}