use twilight_http::{
    client::Client,
    request::guild::{GetGuildWidget, UpdateGuildWidget},
};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

/// Work with a guild's widget.
#[derive(Clone, Debug)]
pub struct GuildWidgetResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildWidgetResource<'a> {
    /// Create a resource instance to work with a guild's widget.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's widget.
    pub const fn get(&self) -> GetGuildWidget<'a> {
        self.0.guild_widget(self.1)
    }

    /// Update a guild's widget.
    pub const fn patch(&self, role_id: Id<RoleMarker>) -> UpdateGuildWidget<'a> {
        self.0.update_guild_widget(self.1, role_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildWidgetResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildWidgetResource<'_>: Clone, Debug, Send, Sync);
}
