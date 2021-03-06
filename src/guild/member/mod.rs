//! Work with guild members and their relationships.

mod role;

pub use self::role::GuildMemberRoleResource;

use twilight_http::{
    client::Client,
    request::guild::member::{
        AddGuildMember, GetGuildMembers, GetMember, RemoveMember, SearchGuildMembers,
        UpdateGuildMember,
    },
};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

/// Work with a guild's members.
#[derive(Clone, Debug)]
pub struct GuildMemberResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildMemberResource<'a> {
    /// Create a resource instance to work with a guild's member.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Remove a member from a guild.
    pub const fn delete(&self, user_id: Id<UserMarker>) -> RemoveMember<'a> {
        self.0.remove_guild_member(self.1, user_id)
    }

    /// Get a guild member.
    pub const fn get(&self, user_id: Id<UserMarker>) -> GetMember<'a> {
        self.0.guild_member(self.1, user_id)
    }

    /// List a guild's members.
    pub const fn list(&self) -> GetGuildMembers<'a> {
        self.0.guild_members(self.1)
    }

    /// Update a guild member.
    pub const fn patch(&self, user_id: Id<UserMarker>) -> UpdateGuildMember<'a> {
        self.0.update_guild_member(self.1, user_id)
    }

    /// Add a member to a guild.
    pub const fn post(&self, user_id: Id<UserMarker>, access_token: &'a str) -> AddGuildMember<'a> {
        self.0.add_guild_member(self.1, user_id, access_token)
    }
}

/// RPC calls.
impl<'a> GuildMemberResource<'a> {
    /// Search a guild's members.
    pub const fn search(
        &self,
        guild_id: Id<GuildMarker>,
        query: &'a str,
    ) -> SearchGuildMembers<'a> {
        self.0.search_guild_members(guild_id, query)
    }
}

/// 1:M guild member relationships.
impl<'a> GuildMemberResource<'a> {
    /// Work with a guild member's roles.
    pub const fn roles(&self, user_id: Id<UserMarker>) -> GuildMemberRoleResource<'a> {
        GuildMemberRoleResource::new(self.0, self.1, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildMemberResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildMemberResource<'_>: Clone, Debug, Send, Sync);
}
