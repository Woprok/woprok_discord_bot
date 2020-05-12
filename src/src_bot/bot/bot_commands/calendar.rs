//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use std::
{
    collections::HashSet, 
    env, 
    hash::{Hash, Hasher},
    sync::Arc,
};
use hey_listen::sync:: // We will use this crate as event dispatcher.
{
    ParallelDispatcher as Dispatcher,
    ParallelDispatcherRequest as DispatcherRequest
};
use white_rabbit:: // And this crate to schedule our tasks.
{
    Utc, 
    Scheduler, 
    DateResult, 
    Duration
};

// Custom event enum
#[derive(Clone)]
pub enum DispatchEvent 
{
    ReactEvent(MessageId, UserId),
}
// We need to implement equality for our enum.
// One could test variants only. In this case, we want to know who reacted on which message.
impl PartialEq for DispatchEvent 
{
    fn eq(&self, other: &DispatchEvent) -> bool 
    {
        match (self, other) 
        {
            (DispatchEvent::ReactEvent(self_message_id, self_user_id),
            DispatchEvent::ReactEvent(other_message_id, other_user_id)) => 
            {
                self_message_id == other_message_id &&
                self_user_id == other_user_id
            }
        }
    }
}

impl Eq for DispatchEvent {}
// See following Clippy-lint: https://rust-lang.github.io/rust-clippy/master/index.html#derive_hash_xor_eq
impl Hash for DispatchEvent 
{
    fn hash<H: Hasher>(&self, state: &mut H) 
    {
        match self 
        {
            DispatchEvent::ReactEvent(msg_id, user_id) => 
            {
                msg_id.hash(state);
                user_id.hash(state);
            }
        }
    }
}

#[group("calendar")]
#[prefixes("rm", "reminder")]
#[commands(set_reminder)]
pub struct Calendar;

pub struct DispatcherKey;
impl TypeMapKey for DispatcherKey 
{
    type Value = Arc<RwLock<Dispatcher<DispatchEvent>>>;
}

pub struct SchedulerKey;
impl TypeMapKey for SchedulerKey 
{
    type Value = Arc<RwLock<Scheduler>>;
}

pub fn get_dispatcher(cpu_count:usize) -> Dispatcher<DispatchEvent>
{
    let mut dispatcher: Dispatcher<DispatchEvent> = Dispatcher::default();
    dispatcher.num_threads(cpu_count).expect("Could not construct threadpool");
    dispatcher
}

pub fn get_scheduler(cpu_count:usize) -> white_rabbit::Scheduler
{
    let scheduler = Scheduler::new(cpu_count);
    scheduler
}

#[command]
#[aliases("add")]
fn set_reminder(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult 
{
    // It might be smart to set a moderately high minimum value for `time`
    // to avoid abuse like tasks that repeat every 100ms, especially since
    // channels have send-message rate limits.
    let time: u64 = args.single()?;
    let repeat: bool = args.single()?;
    let args = args.rest().to_string();

    let scheduler = 
    {
        let mut context = context.data.write();
        context.get_mut::<SchedulerKey>().expect("Expected Scheduler.").clone()
    };

    let dispatcher = 
    {
        let mut context = context.data.write();
        context.get_mut::<DispatcherKey>().expect("Expected Dispatcher.").clone()
    };

    let http = context.http.clone();
    let msg = msg.clone();

    let mut scheduler = scheduler.write();

    // First, we check if the user wants a repeated task or not.
    if repeat 
    {
        // Chrono's duration can also be negative
        // and therefore we cast to `i64`.
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| 
        {
            let bot_msg = match msg.channel_id.say(&http, &args) 
            {
                Ok(msg) => msg,
                // We could not send the message, thus we will try sending it
                // again in five seconds.
                // It might be wise to keep a counter for maximum tries.
                // If the channel got deleted, trying to send a message will
                // always fail.
                Err(why) => 
                {
                    println!("Error sending message: {:?}.", why);
                    return DateResult::Repeat(Utc::now() + Duration::milliseconds(5000))
                },
            };

            let http = http.clone();

            // We add a function to dispatch for a certain event.
            dispatcher.write()
                .add_fn(DispatchEvent::ReactEvent(bot_msg.id, msg.author.id),
                    // The `thanks_for_reacting`-function creates a function
                    // to schedule.
                    thanks_for_reacting(http, bot_msg.channel_id));

            // We return that our date shall happen again, therefore we need
            // to tell when this shall be.
            DateResult::Repeat(Utc::now() + Duration::milliseconds(time as i64))
        });
    } 
    else 
    {
        // Pretty much identical with the `true`-case except for the returned
        // variant.
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| 
        {
            let bot_msg = match msg.channel_id.say(&http, &args) 
            {
                Ok(msg) => msg,
                Err(why) => 
                {
                    println!("Error sending message: {:?}.", why);
                    return DateResult::Repeat(Utc::now() + Duration::milliseconds(5000))
                },
            };
            let http = http.clone();

            dispatcher.write()
                .add_fn(DispatchEvent::ReactEvent(bot_msg.id, msg.author.id),
                    thanks_for_reacting(http, bot_msg.channel_id));

            // The task is done and that's it, we don't need to repeat it.
            DateResult::Done
        });
    };
    Ok(())
}

// Just a helper-function for creating the closure we want to use as listener.
// It saves us from writing the same trigger twice for repeated and non-repeated
// tasks (see remind-me command below).
fn thanks_for_reacting(http: Arc<Http>, channel: ChannelId) -> Box<dyn Fn(&DispatchEvent) -> Option<DispatcherRequest> + Send + Sync> 
{
    Box::new(move |_| 
    {
        if let Err(why) = channel.say(&http, "Thanks for reacting!") 
        {
            println!("Could not send message: {:?}", why);
        }
        Some(DispatcherRequest::StopListening)
    })
}