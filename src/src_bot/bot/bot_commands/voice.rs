//Usings, Mods, Crates, Macros...
use_expansion_serenity!();
use std::
{
    env, 
    sync::Arc
};

#[group("voice")]
#[commands(deafen, join, join_args, leave, mute, play, undeafen, unmute)]
pub struct Voice;

pub struct VoiceManager;
impl TypeMapKey for VoiceManager 
{
    type Value = Arc<Mutex<ClientVoiceManager>>;
}
pub struct Receiver;
impl Receiver 
{
    pub fn new() -> Self 
    {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self { }
    }
}

impl AudioReceiver for Receiver 
{
    fn speaking_update(&mut self, _ssrc: u32, _user_id: u64, _speaking: bool) 
    {
        // You can implement logic here so that you can differentiate users'
        // SSRCs and map the SSRC to the User ID and maintain a state in
        // `Receiver`. Using this map, you can map the `ssrc` in `voice_packet`
        // to the user ID and handle their audio packets separately.
    }

    fn voice_packet(&mut self, ssrc: u32,sequence: u16,_timestamp: u32, _stereo: bool, data: &[i16],compressed_size: usize) 
    {
        println!("Audio packet's first 5 bytes: {:?}", data.get(..5));
        println!("Audio packet sequence {:05} has {:04} bytes (decompressed from {}), SSRC {}",
            sequence, data.len(), compressed_size, ssrc);
    }

    fn client_connect(&mut self, _ssrc: u32, _user_id: u64) 
    {
        // You can implement your own logic here to handle a user who has joined the
        // voice channel e.g., allocate structures, map their SSRC to User ID.
    }

    fn client_disconnect(&mut self, _user_id: u64) 
    {
        // You can implement your own logic here to handle a user who has left the
        // voice channel e.g., finalise processing of statistics etc.
        // You will typically need to map the User ID to their SSRC; observed when
        // speaking or connecting.
    }
}

#[command]
fn deafen(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => 
        {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));
            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) 
    {
        Some(handler) => handler,
        None => 
        {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));
            return Ok(());
        },
    };

    if handler.self_deaf 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened"));
    } 
    else 
    {
        handler.deafen(true);
        check_msg(msg.channel_id.say(&ctx.http, "Deafened"));
    }

    Ok(())
}

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let guild = match msg.guild(&ctx.cache) 
    {
        Some(guild) => guild,
        None =>
        {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));
            return Ok(());
        }
    };

    let guild_id = guild.read().id;
    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id 
    {
        Some(channel) => channel,
        None => 
        {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));
            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() 
    {
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));
    } 
    else 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }
    Ok(())
}

#[command]
fn join_args(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let connect_to = match args.single::<u64>() 
    {
        Ok(id) => ChannelId(id),
        Err(_) => {
            check_msg(msg.reply(&ctx, "Requires a valid voice channel ID be given"));

            return Ok(());
        },
    };

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        },
    };
    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    if let Some(handler) = manager.join(guild_id, connect_to) 
    {
        handler.listen(Some(Box::new(Receiver::new())));
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));
    } 
    else 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }
    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => 
        {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));
            return Ok(());
        },
    };
    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();
    if has_handler 
    {
        manager.remove(guild_id);
        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel"));
    } 
    else 
    {
        check_msg(msg.reply(&ctx, "Not in a voice channel"));
    }
    Ok(())
}

#[command]
fn mute(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => 
        {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));
            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) 
    {
        Some(handler) => handler,
        None => 
        {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));
            return Ok(());
        },
    };

    if handler.self_mute 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted"));
    } 
    else 
    {
        handler.mute(true);
        check_msg(msg.channel_id.say(&ctx.http, "Now muted"));
    }

    Ok(())
}

#[command]
fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    let url = match args.single::<String>() 
    {
        Ok(url) => url,
        Err(_) => 
        {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio"));
            return Ok(());
        },
    };

    if !url.starts_with("http") 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL"));
        return Ok(());
    }

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => 
        {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));
            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) 
    {
        info!("{}", &url);
        let source = match voice::ytdl(&url) 
        {
            Ok(source) => source,
            Err(why) => 
            {
                println!("Err starting source: {:?}", why);
                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg"));
                return Ok(());
            },
        };

        handler.play(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song"));
    } 
    else 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in"));
    }

    Ok(())
}

#[command]
fn undeafen(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) 
    {
        handler.deafen(false);

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened"));
    } 
    else 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to undeafen in"));
    }

    Ok(())
}

#[command]
fn unmute(ctx: &mut Context, msg: &Message) -> CommandResult 
{
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) 
    {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));

            return Ok(());
        },
    };
    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) 
    {
        handler.mute(false);

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted"));
    } 
    else 
    {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to unmute in"));
    }

    Ok(())
}

// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) 
{
    if let Err(why) = result 
    {
        println!("Error sending message: {:?}", why);
    }
}