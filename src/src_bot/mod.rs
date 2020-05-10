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
                        EmbedAuthor,
                        Channel, 
                        Message,
                    }, 
                    event::ResumedEvent,
                    gateway::
                    {
                        Ready,
                        Activity,
                    }, 
                    id::UserId,
                    prelude::*,
                    user::OnlineStatus,
                },
                utils::
                {
                    MessageBuilder,
                    content_safe, 
                    ContentSafeOptions,
                    Colour,
                },
                http::AttachmentType,
                prelude::*,
            };
        }  
    }
    pub mod bot_commands
    {
        pub mod about; 
        pub mod admin; 
        pub mod bot_message_handlers;
        pub mod emoji;
        pub mod game; 
        pub mod math;
        pub mod meta;
        pub mod normal;
        pub mod owner;
    }
    pub mod bot_core
    {
        pub mod bot_framework;
        pub mod bot_main;
    }
    pub mod bot_utils
    {
        pub mod bot_personality;
    }
}