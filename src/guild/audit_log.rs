use twilight_http::{client::Client, request::guild::GetAuditLog};
use twilight_model::id::GuildId;

/// Work with a guild's audit log entries.
#[derive(Clone, Debug)]
pub struct GuildAuditLogResource<'a>(&'a Client, GuildId);

impl<'a> GuildAuditLogResource<'a> {
    /// Create a resource instance to work with a guild's audit log entries.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Get a guild's audit log entries.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetAuditLog<'a> {
        self.0.audit_log(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildAuditLogResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildAuditLogResource<'_>: Clone, Debug, Send, Sync);
}
