use twilight_http::{client::Client, request::guild::GetGuildPreview};
use twilight_model::id::GuildId;

/// Work with a guild's widget.
#[derive(Clone, Debug)]
pub struct GuildPreviewResource<'a>(&'a Client, GuildId);

impl<'a> GuildPreviewResource<'a> {
    /// Create a resource instance to work with a guild's preview.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's preview.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self) -> GetGuildPreview<'a> {
        self.0.guild_preview(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildPreviewResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildPreviewResource<'_>: Clone, Debug, Send, Sync);
}