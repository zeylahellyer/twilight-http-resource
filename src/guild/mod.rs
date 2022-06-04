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
        template::CreateGuildFromTemplate,
    },
};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};
use twilight_validate::request::ValidationError;

/// Work with guilds.
#[derive(Clone, Debug)]
pub struct GuildResource<'a>(&'a Client);

impl<'a> GuildResource<'a> {
    /// Create a resource instance to work with guilds.
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Delete a guild.
    pub const fn delete(&self, id: Id<GuildMarker>) -> DeleteGuild<'a> {
        self.0.delete_guild(id)
    }

    /// Get a guild.
    pub const fn get(&self, id: Id<GuildMarker>) -> GetGuild<'a> {
        self.0.guild(id)
    }

    /// Update a guild.
    pub const fn patch(&self, id: Id<GuildMarker>) -> UpdateGuild<'a> {
        self.0.update_guild(id)
    }

    /// Create a guild.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild`] for error information.
    ///
    /// [`Client::create_guild`]: twilight_http::Client::create_guild
    pub fn post(&self, name: String) -> Result<CreateGuild<'a>, CreateGuildError> {
        self.0.create_guild(name)
    }
}

/// RPC calls.
impl<'a> GuildResource<'a> {
    /// Work with a guild's prune capability.
    pub const fn prune(&self, guild_id: Id<GuildMarker>) -> GuildPruneRpc<'a> {
        GuildPruneRpc::new(self.0, guild_id)
    }

    /// Create a guild from a template.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_guild_from_template`] for error information.
    ///
    /// [`Client::create_guild_from_template`]: twilight_http::Client::create_guild_from_template
    pub fn post_from_template(
        &self,
        template_code: &'a str,
        name: &'a str,
    ) -> Result<CreateGuildFromTemplate<'_>, ValidationError> {
        self.0.create_guild_from_template(template_code, name)
    }
}

/// 1:1 guild relationships.
impl<'a> GuildResource<'a> {
    /// Work with a guild's preview.
    pub const fn preview(&self, guild_id: Id<GuildMarker>) -> GuildPreviewResource<'a> {
        GuildPreviewResource::new(self.0, guild_id)
    }

    /// Work with a guild's vanity URL.
    pub const fn vanity_url(&self, guild_id: Id<GuildMarker>) -> GuildVanityUrlResource<'a> {
        GuildVanityUrlResource::new(self.0, guild_id)
    }

    /// Work with a guild's welcome screen.
    pub const fn welcome_screen(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> GuildWelcomeScreenResource<'a> {
        GuildWelcomeScreenResource::new(self.0, guild_id)
    }
}

/// 1:M guild relationships.
impl<'a> GuildResource<'a> {
    /// Work with a guild's audit log entries.
    pub const fn audit_logs(&self, guild_id: Id<GuildMarker>) -> GuildAuditLogResource<'a> {
        GuildAuditLogResource::new(self.0, guild_id)
    }

    /// Work with a guild's bans.
    pub const fn bans(&self, guild_id: Id<GuildMarker>) -> GuildBanResource<'a> {
        GuildBanResource::new(self.0, guild_id)
    }

    /// Work with a guild's channels.
    pub const fn channels(&self, guild_id: Id<GuildMarker>) -> GuildChannelResource<'a> {
        GuildChannelResource::new(self.0, guild_id)
    }

    /// Work with a guild's emojis.
    pub const fn emojis(&self, guild_id: Id<GuildMarker>) -> GuildEmojiResource<'a> {
        GuildEmojiResource::new(self.0, guild_id)
    }

    /// Work with a guild's integrations.
    pub const fn integrations(&self, guild_id: Id<GuildMarker>) -> GuildIntegrationResource<'a> {
        GuildIntegrationResource::new(self.0, guild_id)
    }

    /// Work with a guild's invites.
    pub const fn invites(&self, guild_id: Id<GuildMarker>) -> GuildInviteResource<'a> {
        GuildInviteResource::new(self.0, guild_id)
    }

    /// Work with a guild's members.
    pub const fn members(&self, guild_id: Id<GuildMarker>) -> GuildMemberResource<'a> {
        GuildMemberResource::new(self.0, guild_id)
    }

    /// Work with a guild's roles.
    pub const fn roles(&self, guild_id: Id<GuildMarker>) -> GuildRoleResource<'a> {
        GuildRoleResource::new(self.0, guild_id)
    }

    /// Work with a guild's templates.
    pub const fn templates(&self, guild_id: Id<GuildMarker>) -> GuildTemplateResource<'a> {
        GuildTemplateResource::new(self.0, guild_id)
    }

    /// Work with a guild's voice states.
    pub const fn voice_states(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> GuildVoiceStateResource<'a> {
        GuildVoiceStateResource::new(self.0, guild_id, user_id)
    }

    /// Work with a guild's webhooks.
    pub const fn webhooks(&self, guild_id: Id<GuildMarker>) -> GuildWebhookResource<'a> {
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
