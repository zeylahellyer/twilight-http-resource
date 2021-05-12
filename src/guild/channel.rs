use twilight_http::{
    client::Client,
    request::guild::{
        create_guild_channel::{CreateGuildChannel, CreateGuildChannelError},
        GetGuildChannels, UpdateGuildChannelPositions,
    },
};
use twilight_model::id::{ChannelId, GuildId};

/// Work with a guild's channels.
#[derive(Clone, Debug)]
pub struct GuildChannelResource<'a>(&'a Client, GuildId);

impl<'a> GuildChannelResource<'a> {
    /// Create a resource instance to work with a guild's channels.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// List a guild's channels.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetGuildChannels<'a> {
        self.0.guild_channels(self.1)
    }

    /// Update all of a guild's channels.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch_list(
        &self,
        channels: impl Iterator<Item = (ChannelId, u64)>,
    ) -> UpdateGuildChannelPositions<'a> {
        self.0.update_guild_channel_positions(self.1, channels)
    }

    /// Create a guild channel.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild_channel`] for error information.
    ///
    /// [`Client::create_guild_channel`]: twilight_http::Client::create_guild_channel
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(
        &self,
        name: impl Into<String>,
    ) -> Result<CreateGuildChannel<'a>, CreateGuildChannelError> {
        self.0.create_guild_channel(self.1, name)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildChannelResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildChannelResource<'_>: Clone, Debug, Send, Sync);
}
