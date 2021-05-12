use twilight_http::{
    client::Client,
    request::user::{GetCurrentUserGuilds, LeaveGuild},
};
use twilight_model::id::GuildId;

/// Work with a current user's guilds.
#[derive(Clone, Debug)]
pub struct UserMeGuildResource<'a>(&'a Client);

impl<'a> UserMeGuildResource<'a> {
    /// Create a resource instance to work with the current user's guilds.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Leave one of the current user's guilds.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, guild_id: GuildId) -> LeaveGuild<'a> {
        self.0.leave_guild(guild_id)
    }

    /// List the current user's guilds.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetCurrentUserGuilds<'a> {
        self.0.current_user_guilds()
    }
}

#[cfg(test)]
mod tests {
    use super::UserMeGuildResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(UserMeGuildResource<'_>: Clone, Debug, Send, Sync);
}
