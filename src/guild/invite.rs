use twilight_http::{client::Client, request::guild::GetGuildInvites};
use twilight_model::id::{marker::GuildMarker, Id};

/// Work with a guild's invites.
#[derive(Clone, Debug)]
pub struct GuildInviteResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildInviteResource<'a> {
    /// Create a resource instance to work with a guild's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// List a guild's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetGuildInvites<'a> {
        self.0.guild_invites(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildInviteResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildInviteResource<'_>: Clone, Debug, Send, Sync);
}
