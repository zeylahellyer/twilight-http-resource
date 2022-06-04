use twilight_http::{
    client::Client,
    request::guild::{
        update_guild_channel_positions::Position, CreateGuildChannel, GetGuildChannels,
        UpdateGuildChannelPositions,
    },
};
use twilight_model::id::{marker::GuildMarker, Id};
use twilight_validate::channel::ChannelValidationError;

/// Work with a guild's channels.
#[derive(Clone, Debug)]
pub struct GuildChannelResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildChannelResource<'a> {
    /// Create a resource instance to work with a guild's channels.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// List a guild's channels.
    pub const fn list(&self) -> GetGuildChannels<'a> {
        self.0.guild_channels(self.1)
    }

    /// Update all of a guild's channels.
    pub const fn patch_list(&self, channels: &'a [Position]) -> UpdateGuildChannelPositions<'a> {
        self.0.update_guild_channel_positions(self.1, channels)
    }

    /// Create a guild channel.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild_channel`] for error information.
    ///
    /// [`Client::create_guild_channel`]: twilight_http::Client::create_guild_channel
    pub fn post(&self, name: &'a str) -> Result<CreateGuildChannel<'a>, ChannelValidationError> {
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
