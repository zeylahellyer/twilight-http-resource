use twilight_http::{client::Client, request::guild::UpdateCurrentUserNick};
use twilight_model::id::GuildId;

/// Work with a guild member's nickname.
#[derive(Clone, Debug)]
pub struct GuildMemberNicknameResource<'a>(&'a Client, GuildId);

impl<'a> GuildMemberNicknameResource<'a> {
    /// Create a resource instance to work with a guild member's nickname.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Update the current user guild member's nickname.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(&self, nick: impl Into<String>) -> UpdateCurrentUserNick<'a> {
        self.0.update_current_user_nick(self.1, nick)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildMemberNicknameResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildMemberNicknameResource<'_>: Clone, Debug, Send, Sync);
}
