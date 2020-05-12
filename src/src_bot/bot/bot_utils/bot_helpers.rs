//Usings, Mods, Crates, Macros
use_expansion_serenity!();
use crate::src_bot::bot::bot_core::bot_framework;

/// Finds the UserId of a member matching a particular name in a guild.
pub fn find_user_in_guild<G: Into<GuildId>>(cache: &Cache, id: G, name: &str) -> Option<UserId> 
{
    cache
        .guild(id)
        .and_then(|guild| guild.read().member_named(name).map(|m| m.user_id()))
}


/// Return an optional query with the spaces converted to underscores, or
/// otherwise use a default.
pub fn default_query(query: Option<&str>, default: &str) -> String {
    query
        .map(|q| q.replace(' ', "_"))
        .unwrap_or_else(|| default.to_string())
}

/// Return the bot's invite URL with the appropriate permissions.
pub fn invite_url(ctx: &Context) -> serenity::Result<String> {
    let data = ctx.data.read();
    let perms = data
        .get::<bot_framework::PermissionsContainer>()
        .copied()
        .unwrap_or_else(|| Permissions::empty());

    ctx.cache.read().user.invite_url(&ctx.http, perms)
}