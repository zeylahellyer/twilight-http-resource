use twilight_http::{
    client::Client,
    request::guild::{CreateGuildPrune, GetGuildPruneCount},
};
use twilight_model::id::GuildId;

/// Work with a guild's RPC pruning.
#[derive(Clone, Debug)]
pub struct GuildPruneRpc<'a>(&'a Client, GuildId);

impl<'a> GuildPruneRpc<'a> {
    /// Create a resource instance to work with a guild's pruning.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's prune information.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn get(&self) -> GetGuildPruneCount<'a> {
        self.0.guild_prune_count(self.1)
    }

    /// Begin a guild prune.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(&self) -> CreateGuildPrune<'a> {
        self.0.create_guild_prune(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildPruneRpc;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildPruneRpc<'_>: Clone, Debug, Send, Sync);
}
