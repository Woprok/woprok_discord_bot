//Usings, Mods, Crates, Macros
use_expansion_serenity!();

#[group("dice")]
pub struct Dice;

#[command("4")]
pub fn four(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  return Ok(());
}

#[command("6")]
pub fn six(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  return Ok(());
}

#[command("8")]
pub fn eight(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  return Ok(());
}

#[command("12")]
pub fn twelve(ctx: &mut Context, msg: &Message) -> CommandResult 
{ 
    return Ok(()); 
}

#[command("20")]
pub fn twenty(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  return Ok(());
}

#[command("100")]
pub fn hundred(ctx: &mut Context, msg: &Message) -> CommandResult 
{
  return Ok(());
}