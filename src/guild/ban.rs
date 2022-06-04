use twilight_http::{
    client::Client,
    request::guild::ban::{CreateBan, DeleteBan, GetBan, GetBans},
};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

/// Work with a guild's bans.
#[derive(Clone, Debug)]
pub struct GuildBanResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildBanResource<'a> {
    /// Create a resource instance to work with a guild's bans.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild ban.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self, user_id: Id<UserMarker>) -> DeleteBan<'a> {
        self.0.delete_ban(self.1, user_id)
    }

    /// Get a guild ban.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn get(&self, user_id: Id<UserMarker>) -> GetBan<'a> {
        self.0.ban(self.1, user_id)
    }

    /// List a guild's bans.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetBans<'a> {
        self.0.bans(self.1)
    }

    /// Create a guild ban.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(&self, user_id: Id<UserMarker>) -> CreateBan<'a> {
        self.0.create_ban(self.1, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildBanResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildBanResource<'_>: Clone, Debug, Send, Sync);
}
