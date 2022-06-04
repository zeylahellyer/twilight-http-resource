use twilight_http::{
    client::Client,
    request::guild::role::{CreateRole, DeleteRole, GetGuildRoles, UpdateRole},
};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

/// Work with a guild's roles.
#[derive(Clone, Debug)]
pub struct GuildRoleResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildRoleResource<'a> {
    /// Create a resource instance to work with a guild's roles.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild's role.
    pub const fn delete(&self, role_id: Id<RoleMarker>) -> DeleteRole<'a> {
        self.0.delete_role(self.1, role_id)
    }

    /// List a guild's roles.
    pub const fn list(&self) -> GetGuildRoles<'a> {
        self.0.roles(self.1)
    }

    /// Update a guild's role.
    pub const fn patch(&self, role_id: Id<RoleMarker>) -> UpdateRole<'a> {
        self.0.update_role(self.1, role_id)
    }

    /// Create a guild role.
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
