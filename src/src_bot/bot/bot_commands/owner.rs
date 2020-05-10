//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use crate::src_bot::bot;

#[group("owners")]
#[owners_only]
#[only_in(guilds)] // Limit all commands to be guild-restricted.
#[checks(Admin)] // Adds checks that need to be passed.
#[commands(quit)]
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
        manager.lock().shutdown_all();
    } 
    else 
    {
        msg.reply(&ctx, "There was a problem getting the shard manager")?;
        return Ok(());
    }
    msg.reply(&ctx, "Shutting down!")?;
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