use twilight_http::{
    client::Client,
    request::guild::user::{UpdateCurrentUserVoiceState, UpdateUserVoiceState},
};
use twilight_model::id::{ChannelId, GuildId, UserId};

/// Work with a guild member's voice states.
#[derive(Clone, Debug)]
pub struct GuildVoiceStateResource<'a>(&'a Client, GuildId, UserId);

impl<'a> GuildVoiceStateResource<'a> {
    /// Create a resource instance to work with a guild's voice states.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self(client, guild_id, user_id)
    }

    /// Update a guild's voice state.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(&self, channel_id: ChannelId) -> UpdateUserVoiceState<'a> {
        self.0.update_user_voice_state(self.1, self.2, channel_id)
    }
}

/// Special calls.
impl<'a> GuildVoiceStateResource<'a> {
    /// Update the current user's voice state in a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch_current_user(
        &self,
        channel_id: ChannelId,
    ) -> UpdateCurrentUserVoiceState<'a> {
        self.0.update_current_user_voice_state(self.1, channel_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildVoiceStateResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildVoiceStateResource<'_>: Clone, Debug, Send, Sync);
}
