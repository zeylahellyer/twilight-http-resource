use twilight_http::{client::Client, request::guild::GetGuildVanityUrl};
use twilight_model::id::GuildId;

/// Work with a guild's vanity URL.
#[derive(Clone, Debug)]
pub struct GuildVanityUrlResource<'a>(&'a Client, GuildId);

impl<'a> GuildVanityUrlResource<'a> {
    /// Create a resource instance to work with a guild's vanity URL.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's vanity URL.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self) -> GetGuildVanityUrl<'a> {
        self.0.guild_vanity_url(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildVanityUrlResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildVanityUrlResource<'_>: Clone, Debug, Send, Sync);
}
