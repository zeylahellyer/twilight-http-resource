//! Work with guild members and their relationships.

mod nickname;
mod role;

pub use self::{nickname::GuildMemberNicknameResource, role::GuildMemberRoleResource};

use twilight_http::{
    client::Client,
    request::guild::member::{
        AddGuildMember, GetGuildMembers, GetMember, RemoveMember, SearchGuildMembers,
        UpdateGuildMember,
    },
};
use twilight_model::id::{GuildId, UserId};

/// Work with a guild's members.
#[derive(Clone, Debug)]
pub struct GuildMemberResource<'a>(&'a Client, GuildId);

impl<'a> GuildMemberResource<'a> {
    /// Create a resource instance to work with a guild's member.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Remove a member from a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, user_id: UserId) -> RemoveMember<'a> {
        self.0.remove_guild_member(self.1, user_id)
    }

    /// Get a guild member.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self, user_id: UserId) -> GetMember<'a> {
        self.0.guild_member(self.1, user_id)
    }

    /// List a guild's members.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetGuildMembers<'a> {
        self.0.guild_members(self.1)
    }

    /// Update a guild member.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(&self, user_id: UserId) -> UpdateGuildMember<'a> {
        self.0.update_guild_member(self.1, user_id)
    }

    /// Add a member to a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(&self, user_id: UserId, access_token: impl Into<String>) -> AddGuildMember<'a> {
        self.0.add_guild_member(self.1, user_id, access_token)
    }
}

/// RPC calls.
impl<'a> GuildMemberResource<'a> {
    /// Search a guild's members.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn search(&self, guild_id: GuildId, query: impl Into<String>) -> SearchGuildMembers<'a> {
        self.0.search_guild_members(guild_id, query)
    }
}

/// 1:1 guild member relationships.
impl<'a> GuildMemberResource<'a> {
    /// Work with the current user's nickname.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn current_user_nickname(&self) -> GuildMemberNicknameResource<'a> {
        GuildMemberNicknameResource::new(self.0, self.1)
    }
}

/// 1:M guild member relationships.
impl<'a> GuildMemberResource<'a> {
    /// Work with a guild member's roles.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn roles(&self, user_id: UserId) -> GuildMemberRoleResource<'a> {
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
