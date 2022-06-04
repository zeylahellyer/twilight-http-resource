use twilight_http::{
    client::Client,
    request::guild::integration::{DeleteGuildIntegration, GetGuildIntegrations},
};
use twilight_model::id::{
    marker::{GuildMarker, IntegrationMarker},
    Id,
};

/// Work with a guild's integrations.
#[derive(Clone, Debug)]
pub struct GuildIntegrationResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildIntegrationResource<'a> {
    /// Create a resource instance to work with a guild's integrations.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild's integration.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(
        &self,
        integration_id: Id<IntegrationMarker>,
    ) -> DeleteGuildIntegration<'a> {
        self.0.delete_guild_integration(self.1, integration_id)
    }

    /// List a guild's integrations.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetGuildIntegrations<'a> {
        self.0.guild_integrations(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildIntegrationResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildIntegrationResource<'_>: Clone, Debug, Send, Sync);
}
