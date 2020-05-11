use_expansion_serenity!();

//Mods
use crate::src_bot::
{
    bot::
    {
        bot_commands::
        {
            about::*,
            admin::*,
            bot_message_handlers,
            game::*,
            emoji::*,
            math::*,
            meta::*,
            normal::*,
            owner::*,
            voice::*,
        },
        bot_core::
        {
        },
        bot_utils,
    }
};
use std::
{
    collections::
    {
        HashMap,
        HashSet,
    },
    sync::Arc,
    thread,
    time::Duration,
    env,
    fmt::Write,
};
pub struct CommandCounter;
impl TypeMapKey for CommandCounter
{
    type Value = HashMap<String, u64>;
}

#[help]
#[individual_command_tip =
"Greetings my friend! Do you wish to see my wares?
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)] //How much it will try to fill to real command
#[indention_prefix = "+"]
#[lacking_permissions = "Strike"]
#[lacking_role = "Strike"]
#[wrong_channel = "Strike"]
fn my_help(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult 
{
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

// Construct the full framework for a client instance.
pub fn construct_framework(owners:std::collections::HashSet<serenity::model::id::UserId>, bot_id:serenity::model::id::UserId) -> StandardFramework 
{
    StandardFramework::new()
        .configure(|c| c //create config.
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("!w_") //Prefix for all commands.
            .delimiters(vec![" ",", ", ","]) //Delimeters see doc.
            .owners(owners)) //Set bots owners.
        .before(|ctx, msg, command_name| //Functions executed before command execution.
        {
            info!("Got command '{}' by user '{}'", command_name, msg.author.name);
            //Increment the number of times this command has been run once.
            let mut data = ctx.data.write();
            let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");
            let entry = counter.entry(command_name.to_string()).or_insert(0);
            *entry += 1;
            true //If `before` returns false, command processing doesn't happen.
        })
        .after(|_, _, command_name, error| //Functions executed before command execution. 
        {
            match error 
            {
                Ok(()) => info!("Processed command '{}'", command_name),
                Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        .unrecognised_command(|_, _, unknown_command_name|  //Set a function that's called whenever an attempted command-call's command could not be found.
        {
            info!("Could not find command named '{}'", unknown_command_name);
        })
        .normal_message(|_, message|  //Set a function that's called whenever a message is not a command.
        {
            info!("Message is not a command '{}'", message.content);
        })
        .on_dispatch_error(|ctx, msg, error| //Set a function that's called whenever a command's execution didn't complete for any reason.
        {
            if let DispatchError::Ratelimited(seconds) = error 
            {
                if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m
                    .embed(|e| e
                        .title(&format!("Command cooldown! Try again in {} seconds", seconds))
                        .color(Colour::RED)))
                        {
                            error!("Error sending message: {:?}", why);
                        }
            }
        })
        .help(&MY_HELP) //Set a help function.
        .bucket("emoji", |b| b.delay(5)) //Functions that can't be used more then once per 5 seconds.
        .bucket("complicated", |b| b.delay(5).time_span(30).limit(2)) //Max 2 times per 30 seconds, with a 5 second delay.
        .group(&ABOUT_GROUP)
        .group(&ADMIN_GROUP)
        .group(&EMOJI_GROUP)
        .group(&GAME_GROUP)
        .group(&MATH_GROUP)
        .group(&META_GROUP)
        .group(&NORMAL_GROUP)   
        .group(&OWNERS_GROUP)
        .group(&VOICE_GROUP)         
}