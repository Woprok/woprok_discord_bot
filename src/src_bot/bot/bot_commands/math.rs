//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use rand::Rng;

#[group]
#[prefix="math"] //This is added after prefix along with a space.
#[commands(multiply, addition, substract, divide, modulo, random, collatz)]
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

#[command]
#[description("Calculates the number of steps in the [Collatz sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) for a given positive integer.")]
#[num_args(1)]
#[usage("<number>")]
fn collatz(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult 
{
    let mut num = match args.parse::<u128>() 
    {
        Ok(num) => num,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Unable to parse number.")?;
            return Ok(());
        }
    };

    // Calculate number of steps from repeatedly applying Collatz process.
    let mut steps = 0;
    while num > 1 
    {
        steps += 1;
        num = if num % 2 == 0 
        {
            num / 2
        } 
        else 
        {
            match num.checked_mul(3).and_then(|num| num.checked_add(1)) 
            {
                Some(num) => num,
                None => 
                {
                    msg.channel_id.say(&ctx.http, format!("Overflow occurred after {} iterations.", steps))?;
                    return Ok(());
                }
            }
        }
    }
    msg.channel_id.say(&ctx.http, format!("Finished after {} iterations.", steps))?;
    Ok(())
}