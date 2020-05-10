// Defines bot code and most of it serenity library usings.

#[allow(unused_imports)]
pub mod bot
{
    macro_rules! use_expansion_serenity
    {
        () =>
        {
            use serenity::
            {
                client::bridge::gateway::
                {
                    ShardId,
                    ShardManager,
                },
                framework::
                {
                    StandardFramework,
                    standard::
                    {
                        Args, 
                        CheckResult, 
                        CommandOptions, 
                        CommandResult, 
                        CommandGroup,
                        DispatchError, 
                        HelpOptions, 
                        help_commands, 
                        macros::
                        {
                            command,
                            group,
                            help,
                            check,
                        }
                    }
                },
                model::
                {
                    channel::
                    {
                        Channel, 
                        Message,
                    }, 
                    event::ResumedEvent,
                    gateway::Ready, 
                    id::UserId,
                    prelude::*,
                },
                utils::
                {
                    MessageBuilder,
                    content_safe, 
                    ContentSafeOptions,
                },
                http::AttachmentType,
                prelude::*,
            };
        }  
    }
    pub mod bot_commands
    {
        pub mod admin; 
        pub mod math;
        pub mod owner;
        pub mod normal;
        pub mod meta;
        pub mod bot_message_handlers;
    }
    pub mod bot_core
    {
        pub mod bot_main;
    }
    pub mod bot_utils
    {
        pub mod bot_personality;
    }
}