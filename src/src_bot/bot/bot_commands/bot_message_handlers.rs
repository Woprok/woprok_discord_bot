//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use std::{path::Path};

//Methods
// Wrapper around all message evaluations to remove clutter from bot_main handler.
pub fn process_raw_message(ctx: &Context, msg: &Message)
{
    special_pong(&ctx, &msg); 
    special_message(&ctx, &msg);
    special_safe_ping(&ctx, &msg);
    special_show_ferris(&ctx, &msg);
    special_atla(&ctx, &msg);
    special_save_anime(&ctx, &msg);
    special_abuse_johnny_cat(&ctx, &msg);
}

// Classic command used by parsing message context directly.
fn special_pong(ctx:&Context, msg:&Message)
{
    if msg.content == "!w_special_pong"
    {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Ping From Primary Location!") 
        {
            println!("Error sending message: {:?}", why);                
        }
    }
}

// Send a message directly to user.
fn special_message(ctx:&Context, msg:&Message)
{
    if msg.content == "!w_special_messageme" 
    {
        let dm = msg.author
                    .dm(ctx, |m|
                    {
                        m.content("Hello!");
                        m
                    });

        if let Err(why) = dm 
        {
            println!("Error when direct messaging user: {:?}", why);
        }
    }
}

// Message that uses safe and normalized content to prevent missuse (emoji, command text).
// Mention user and location dynamically.
fn special_safe_ping(ctx:&Context, msg:&Message)
{
    if msg.content == "!w_special_ping" 
    {
        let channel = match msg.channel_id.to_channel(ctx) 
        {
            Ok(channel) => channel,
            Err(why) => 
            {
                println!("Error getting channel: {:?}", why);
                return;
            },
        };

        let response = MessageBuilder::new()
                                            .push("User ")
                                            .push_bold_safe(&msg.author.name)
                                            .push(" used the 'ping' command in the ")
                                            .mention(&channel)
                                            .push(" channel")
                                            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response) 
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

// Creates embeded message with builder syntax that has a title, description, three fields, and a footer.
fn special_show_ferris(ctx:&Context, msg:&Message) 
{
    if msg.content == "!w_special_show_ferris" 
    {
        let msg = msg.channel_id.send_message(&ctx.http, |message_builder| 
        {
            message_builder.content("Look at my cute Ferris!");
            message_builder.embed(|embeded| 
            {
                embeded.title("Ferris"); //Title
                embeded.description("He is my trusty companion that helps me to live.");
                embeded.image("attachment://ferris_eyes.png");
                embeded.fields(vec![
                                    ("Likes", "People", true),
                                    ("Dislikes", "Bots named Johnny", true),
                                ]);
                embeded.field("Support us!", "MAKE RUSTY FERRIS GREAT AGAIN!", false);
                embeded.footer(|foot| 
                    {
                        foot.text("© Woprok Bot Charity");
                        foot
                    });
                embeded
            });
            message_builder.add_file(AttachmentType::Path(Path::new("./resources/ferris_eyes.png")));
            message_builder
        });

        if let Err(why) = msg 
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

// Save Atla.
fn special_atla(ctx:&Context, msg:&Message) 
{
    if msg.content == "!w_special_atla" || (msg.content.to_lowercase().contains("atla") && msg.author.id != ctx.cache.read().user.id)
    {
        let msg = msg.channel_id.send_message(&ctx.http, |message_builder| 
        {
            message_builder.content("ATLA IS ALIVE!");
            message_builder.embed(|embeded| 
            {
                embeded.title("JOIN US!"); //Title
                embeded.description("SAVE ATLA!");
                embeded.image("attachment://atla.jpg");
                embeded.field("Atla Captiosus", "Cute young lady.", false);
                embeded.field("Avoid propaganda of our enemies!", "She never died, do not let yourself to be fooled.", false);
                embeded.footer(|foot| 
                    {
                        foot.text("© Movement for Atla");
                        foot
                    });
                embeded
            });
            message_builder.add_file(AttachmentType::Path(Path::new("./resources/atla.jpg")));
            message_builder
        });

        if let Err(why) = msg 
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

// Save anime.
fn special_save_anime(ctx:&Context, msg:&Message)
{
    if msg.content.to_lowercase().contains("anime") && msg.author.id != ctx.cache.read().user.id
    {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Anime is best! Join the bright side and get your waifu pillow now!") 
        {
            println!("Error sending message: {:?}", why);                
        }
    }
}

// Ask other bot for cat.
fn special_abuse_johnny_cat(ctx:&Context, msg:&Message)
{
    if msg.content == "!w_special_cat" || msg.content == "!w_cat"
    {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Johnny, my good slavenemy: Please give me a cat for my friend here.") 
        {
            println!("Error sending message: {:?}", why);                
        }
    }
}