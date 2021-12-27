use twilight_http::{
    client::Client,
    request::guild::role::{CreateRole, DeleteRole, GetGuildRoles, UpdateRole},
};
use twilight_model::id::{GuildId, RoleId};

/// Work with a guild's roles.
#[derive(Clone, Debug)]
pub struct GuildRoleResource<'a>(&'a Client, GuildId);

impl<'a> GuildRoleResource<'a> {
    /// Create a resource instance to work with a guild's roles.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild's role.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self, role_id: RoleId) -> DeleteRole<'a> {
        self.0.delete_role(self.1, role_id)
    }

    /// List a guild's roles.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetGuildRoles<'a> {
        self.0.roles(self.1)
    }

    /// Update a guild's role.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(&self, role_id: RoleId) -> UpdateRole<'a> {
        self.0.update_role(self.1, role_id)
    }

    /// Create a guild role.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(&self) -> CreateRole<'a> {
        self.0.create_role(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildRoleResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildRoleResource<'_>: Clone, Debug, Send, Sync);
}
