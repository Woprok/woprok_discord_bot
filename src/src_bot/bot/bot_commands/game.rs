//Usings, Mods, Crates, Macros...
use_expansion_serenity!();
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use unicode_segmentation::UnicodeSegmentation;


#[group("game")]
#[commands(set_activity, eight_ball, clap, shuffle)]
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

#[command]
#[description(":clap: Spice :clap: up :clap: your :clap: text! :clap:")]
#[min_args(1)]
#[usage("<word>...")]
fn clap(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut result = String::from(":clap: ");
    for word in args.iter::<String>().flat_map(|w| w) {
        result.push_str(&word);
        result.push_str(" :clap: ");
    }

    msg.channel_id.say(&ctx.http, result)?;

    Ok(())
}

#[command("8ball")]
#[description("Ask the Magic 8-Ball a question.")]
#[min_args(1)]
#[usage("<word>...")]
fn eight_ball(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    const ANSWERS: [&str; 20] = [
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes\u{2014}definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];
    let answer = ANSWERS.choose(&mut SmallRng::from_entropy()).unwrap();
    msg.channel_id.say(&ctx.http, answer)?;
    Ok(())
}

#[command]
#[aliases(scramble)]
#[description("Randomly scramble words.")]
#[min_args(1)]
#[usage("<word>...")]
fn shuffle(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult 
{
    // Break up each Unicode word into individual graphemes and shuffle.
    let words: String = args
        .message()
        .split_word_bounds()
        .collect::<Vec<_>>()
        .iter()
        .map(|word| {
            let mut graphemes: Vec<_> = word.graphemes(true).collect();
            graphemes.shuffle(&mut SmallRng::from_entropy());
            graphemes.concat()
        })
        .collect();

    msg.channel_id.say(&ctx.http, &words)?;

    Ok(())
}