use twilight_http::{client::Client, request::guild::GetGuildWebhooks};
use twilight_model::id::{marker::GuildMarker, Id};

/// Work with a guild's webhooks.
#[derive(Clone, Debug)]
pub struct GuildWebhookResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildWebhookResource<'a> {
    /// Create a resource instance to work with a guild's webhooks.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// List a guild's webhooks.
    pub const fn list(&self) -> GetGuildWebhooks<'a> {
        self.0.guild_webhooks(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildWebhookResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildWebhookResource<'_>: Clone, Debug, Send, Sync);
}
