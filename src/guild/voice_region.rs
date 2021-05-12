use twilight_http::{client::Client, request::guild::GetGuildVoiceRegions};
use twilight_model::id::GuildId;

/// Work with a guild's voice regions.
#[derive(Clone, Debug)]
pub struct GuildVoiceRegionResource<'a>(&'a Client, GuildId);

impl<'a> GuildVoiceRegionResource<'a> {
    /// Create a resource instance to work with a guild's voice regions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// List a guild's voice regions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetGuildVoiceRegions<'a> {
        self.0.guild_voice_regions(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildVoiceRegionResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildVoiceRegionResource<'_>: Clone, Debug, Send, Sync);
}
