//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use rand::Rng;

#[group]
#[prefix="math"] //This is added after prefix along with a space.
#[commands(multiply, addition, substract, divide, modulo, random)]
pub struct Math;

//Methods
#[command]
#[aliases("*")]
fn multiply(ctx:&mut Context, msg:&Message, mut args:Args) -> CommandResult 
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = first * second;
    msg.channel_id.say(&ctx.http, format!("{}*{}={}", first.to_string(), second.to_string(), result.to_string()))?;
    Ok(())
}
#[command]
#[aliases("+")]
fn addition(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = first + second;
    msg.channel_id.say(&ctx.http, format!("{}+{}={}", first, second, result))?;
    Ok(())
}
#[command]
#[aliases("-")]
fn substract(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = first - second;
    msg.channel_id.say(&ctx.http, format!("{}-{}={}", first, second, result))?;
    Ok(())
}
#[command]
#[aliases("/")]
fn divide(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = first / second;
    msg.channel_id.say(&ctx.http, format!("{}/{}={}", first, second, result))?;
    Ok(())
}
#[command]
#[aliases("%")]
fn modulo(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = first % second;
    msg.channel_id.say(&ctx.http, format!("{}%{}={}", first, second, result))?;
    Ok(())
}
#[command]
#[aliases("r")]
fn random(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult
{
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;
    let result = rand::thread_rng().gen_range(first, second);
    msg.channel_id.say(&ctx.http, format!("RNG({},{})={}", first, second, result))?;
    Ok(())
}