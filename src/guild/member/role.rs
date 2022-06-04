use twilight_http::{
    client::Client,
    request::guild::member::{AddRoleToMember, RemoveRoleFromMember},
};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

/// Work with a guild member's roles.
#[derive(Clone, Debug)]
pub struct GuildMemberRoleResource<'a>(&'a Client, Id<GuildMarker>, Id<UserMarker>);

impl<'a> GuildMemberRoleResource<'a> {
    /// Create a resource instance to work with a guild member's roles.
    pub const fn new(
        client: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self(client, guild_id, user_id)
    }

    /// Remove a role from a guild member.
    pub const fn delete(&self, role_id: Id<RoleMarker>) -> RemoveRoleFromMember<'a> {
        self.0.remove_guild_member_role(self.1, self.2, role_id)
    }

    /// Add a role to a guild member.
    pub const fn put(&self, role_id: Id<RoleMarker>) -> AddRoleToMember<'a> {
        self.0.add_guild_member_role(self.1, self.2, role_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildMemberRoleResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildMemberRoleResource<'_>: Clone, Debug, Send, Sync);
}
