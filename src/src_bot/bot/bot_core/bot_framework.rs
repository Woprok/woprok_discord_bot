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



// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip =
"Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
// If the `String` is not empty, your given `String` will be used instead.
// If you pass in a `None`, no hint will be displayed at all.
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
            .delimiters(vec![", ", ","]) //Delimeters see doc.
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
        .group(&MATH_GROUP)
        .group(&META_GROUP)
        .group(&NORMAL_GROUP)   
        .group(&OWNERS_GROUP)         
}