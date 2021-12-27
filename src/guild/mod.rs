//! Work with guilds and their relationships.

pub mod member;

mod audit_log;
mod ban;
mod channel;
mod emoji;
mod integration;
mod invite;
mod preview;
mod prune;
mod role;
mod template;
mod vanity_url;
mod voice_region;
mod voice_state;
mod webhook;
mod welcome_screen;

pub use self::{
    audit_log::GuildAuditLogResource, ban::GuildBanResource, channel::GuildChannelResource,
    emoji::GuildEmojiResource, integration::GuildIntegrationResource, invite::GuildInviteResource,
    member::GuildMemberResource, preview::GuildPreviewResource, prune::GuildPruneRpc,
    role::GuildRoleResource, template::GuildTemplateResource, vanity_url::GuildVanityUrlResource,
    voice_region::GuildVoiceRegionResource, voice_state::GuildVoiceStateResource,
    webhook::GuildWebhookResource, welcome_screen::GuildWelcomeScreenResource,
};

use twilight_http::{
    client::Client,
    request::{
        guild::{
            create_guild::{CreateGuild, CreateGuildError},
            DeleteGuild, GetGuild, UpdateGuild,
        },
        template::create_guild_from_template::{
            CreateGuildFromTemplate, CreateGuildFromTemplateError,
        },
    },
};
use twilight_model::id::{GuildId, UserId};

/// Work with guilds.
#[derive(Clone, Debug)]
pub struct GuildResource<'a>(&'a Client);

impl<'a> GuildResource<'a> {
    /// Create a resource instance to work with guilds.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Delete a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self, id: GuildId) -> DeleteGuild<'a> {
        self.0.delete_guild(id)
    }

    /// Get a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn get(&self, id: GuildId) -> GetGuild<'a> {
        self.0.guild(id)
    }

    /// Update a guild.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(&self, id: GuildId) -> UpdateGuild<'a> {
        self.0.update_guild(id)
    }

    /// Create a guild.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild`] for error information.
    ///
    /// [`Client::create_guild`]: twilight_http::Client::create_guild
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(&self, name: String) -> Result<CreateGuild<'a>, CreateGuildError> {
        self.0.create_guild(name)
    }
}

/// RPC calls.
impl<'a> GuildResource<'a> {
    /// Work with a guild's prune capability.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn prune(&self, guild_id: GuildId) -> GuildPruneRpc<'a> {
        GuildPruneRpc::new(self.0, guild_id)
    }

    /// Create a guild from a template.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild_from_template`] for error information.
    ///
    /// [`Client::create_guild_from_template`]: twilight_http::Client::create_guild_from_template
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post_from_template(
        &self,
        template_code: &'a str,
        name: &'a str,
    ) -> Result<CreateGuildFromTemplate<'_>, CreateGuildFromTemplateError> {
        self.0.create_guild_from_template(template_code, name)
    }
}

/// 1:1 guild relationships.
impl<'a> GuildResource<'a> {
    /// Work with a guild's preview.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn preview(&self, guild_id: GuildId) -> GuildPreviewResource<'a> {
        GuildPreviewResource::new(self.0, guild_id)
    }

    /// Work with a guild's vanity URL.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn vanity_url(&self, guild_id: GuildId) -> GuildVanityUrlResource<'a> {
        GuildVanityUrlResource::new(self.0, guild_id)
    }

    /// Work with a guild's welcome screen.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn welcome_screen(&self, guild_id: GuildId) -> GuildWelcomeScreenResource<'a> {
        GuildWelcomeScreenResource::new(self.0, guild_id)
    }
}

/// 1:M guild relationships.
impl<'a> GuildResource<'a> {
    /// Work with a guild's audit log entries.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn audit_logs(&self, guild_id: GuildId) -> GuildAuditLogResource<'a> {
        GuildAuditLogResource::new(self.0, guild_id)
    }

    /// Work with a guild's bans.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn bans(&self, guild_id: GuildId) -> GuildBanResource<'a> {
        GuildBanResource::new(self.0, guild_id)
    }

    /// Work with a guild's channels.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn channels(&self, guild_id: GuildId) -> GuildChannelResource<'a> {
        GuildChannelResource::new(self.0, guild_id)
    }

    /// Work with a guild's emojis.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn emojis(&self, guild_id: GuildId) -> GuildEmojiResource<'a> {
        GuildEmojiResource::new(self.0, guild_id)
    }

    /// Work with a guild's integrations.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn integrations(&self, guild_id: GuildId) -> GuildIntegrationResource<'a> {
        GuildIntegrationResource::new(self.0, guild_id)
    }

    /// Work with a guild's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn invites(&self, guild_id: GuildId) -> GuildInviteResource<'a> {
        GuildInviteResource::new(self.0, guild_id)
    }

    /// Work with a guild's members.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn members(&self, guild_id: GuildId) -> GuildMemberResource<'a> {
        GuildMemberResource::new(self.0, guild_id)
    }

    /// Work with a guild's roles.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn roles(&self, guild_id: GuildId) -> GuildRoleResource<'a> {
        GuildRoleResource::new(self.0, guild_id)
    }

    /// Work with a guild's templates.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn templates(&self, guild_id: GuildId) -> GuildTemplateResource<'a> {
        GuildTemplateResource::new(self.0, guild_id)
    }

    /// Work with a guild's voice states.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn voice_states(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> GuildVoiceStateResource<'a> {
        GuildVoiceStateResource::new(self.0, guild_id, user_id)
    }

    /// Work with a guild's webhooks.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn webhooks(&self, guild_id: GuildId) -> GuildWebhookResource<'a> {
        GuildWebhookResource::new(self.0, guild_id)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildResource<'_>: Clone, Debug, Send, Sync);
}
