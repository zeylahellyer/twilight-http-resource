use twilight_http::{
    client::Client,
    request::guild::{GetGuildWelcomeScreen, UpdateGuildWelcomeScreen},
};
use twilight_model::id::{marker::GuildMarker, Id};

/// Work with a guild's welcome screen.
#[derive(Clone, Debug)]
pub struct GuildWelcomeScreenResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildWelcomeScreenResource<'a> {
    /// Create a resource instance to work with a guild's welcome screen.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's welcome screen.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn get(&self) -> GetGuildWelcomeScreen<'a> {
        self.0.guild_welcome_screen(self.1)
    }

    /// Update a guild's welcome screen.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(&self) -> UpdateGuildWelcomeScreen<'a> {
        self.0.update_guild_welcome_screen(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildWelcomeScreenResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildWelcomeScreenResource<'_>: Clone, Debug, Send, Sync);
}
