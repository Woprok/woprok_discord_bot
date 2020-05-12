//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use rand::Rng;

#[group("dice")]
pub struct Dice;

#[command("4")]
pub fn four(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  let first:u64 = 1;
  let second:u64 = 5;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d4)={}", result))?;
  return Ok(());
}

#[command("6")]
pub fn six(ctx: &mut Context, msg: &Message) -> CommandResult 
{  
  let first:u64 = 1;
  let second:u64 = 7;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d6)={}", result))?;
  return Ok(());
}

#[command("8")]
pub fn eight(ctx: &mut Context, msg: &Message) -> CommandResult 
{  
  let first:u64 = 1;
  let second:u64 = 9;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d8)={}", result))?;
  return Ok(());
}

#[command("12")]
pub fn twelve(ctx: &mut Context, msg: &Message) -> CommandResult 
{  
  let first:u64 = 1;
  let second:u64 = 13;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d12)={}", result))?;
  return Ok(());
}

#[command("20")]
pub fn twenty(ctx: &mut Context, msg: &Message) -> CommandResult 
{  
  let first:u64 = 1;
  let second:u64 = 21;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d20)={}", result))?;
  return Ok(());
}

#[command("100")]
pub fn hundred(ctx: &mut Context, msg: &Message) -> CommandResult 
{  
  let first:u64 = 1;
  let second:u64 = 101;
  rand::thread_rng().gen_range(first, second);
  msg.channel_id.say(&ctx.http, format!("ROLL(d100)={}", result))?;
  return Ok(());
}