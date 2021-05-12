use twilight_http::{
    client::Client,
    request::guild::member::{AddRoleToMember, RemoveRoleFromMember},
};
use twilight_model::id::{GuildId, RoleId, UserId};

/// Work with a guild member's roles.
#[derive(Clone, Debug)]
pub struct GuildMemberRoleResource<'a>(&'a Client, GuildId, UserId);

impl<'a> GuildMemberRoleResource<'a> {
    /// Create a resource instance to work with a guild member's roles.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self(client, guild_id, user_id)
    }

    /// Remove a role from a guild member.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, role_id: RoleId) -> RemoveRoleFromMember<'a> {
        self.0.remove_guild_member_role(self.1, self.2, role_id)
    }

    /// Add a role to a guild member.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn put(&self, role_id: RoleId) -> AddRoleToMember<'a> {
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
