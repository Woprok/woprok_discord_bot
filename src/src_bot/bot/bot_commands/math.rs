//Usings, Mods, Crates, Macros
use_expansion_serenity!();

//Methods
#[command]
pub fn multiply(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();
    let product = one * two;
    let _ = msg.channel_id.say(&ctx.http, product);
    Ok(())
}

#[command]
// Lets us also call `~math *` instead of just `~math multiply`.
#[aliases("*")]
fn multiply_extra(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first * second;

    if let Err(why) = msg.channel_id.say(&ctx.http, &res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, why);
    }

    Ok(())
}