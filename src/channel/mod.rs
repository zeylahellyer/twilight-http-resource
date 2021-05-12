//! Work with guilds and their relationships.

pub mod message;

mod invite;
mod permission_overwrite;
mod pin;
mod webhook;

pub use self::{
    invite::ChannelInviteResource, message::ChannelMessageResource,
    permission_overwrite::ChannelPermissionOverwriteResource, pin::ChannelPinResource,
    webhook::ChannelWebhookResource,
};

use twilight_http::{
    client::Client,
    request::channel::{
        CreateTypingTrigger, DeleteChannel, FollowNewsChannel, GetChannel, UpdateChannel,
    },
};
use twilight_model::id::ChannelId;

/// Work with channels.
#[derive(Clone, Debug)]
pub struct ChannelResource<'a>(&'a Client);

impl<'a> ChannelResource<'a> {
    /// Create a resource instance to work with channels.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Delete a channel.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, id: ChannelId) -> DeleteChannel<'a> {
        self.0.delete_channel(id)
    }

    /// Get a channel.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self, id: ChannelId) -> GetChannel<'a> {
        self.0.channel(id)
    }

    /// Update a channel.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(&self, id: ChannelId) -> UpdateChannel<'a> {
        self.0.update_channel(id)
    }
}

/// RPC calls.
impl<'a> ChannelResource<'a> {
    /// Follow a channel.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn follow(&self, id: ChannelId, webhook_channel_id: ChannelId) -> FollowNewsChannel<'a> {
        self.0.follow_news_channel(id, webhook_channel_id)
    }

    /// Trigger a typing indicator in a channel.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn typing(&self, id: ChannelId) -> CreateTypingTrigger<'a> {
        self.0.create_typing_trigger(id)
    }
}

/// 1:M channel relationships.
impl<'a> ChannelResource<'a> {
    /// Work with a channel's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn invites(&self, channel_id: ChannelId) -> ChannelInviteResource<'a> {
        ChannelInviteResource::new(self.0, channel_id)
    }

    /// Work with a channel's messages.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn messages(&self, channel_id: ChannelId) -> ChannelMessageResource<'a> {
        ChannelMessageResource::new(self.0, channel_id)
    }

    /// Work with a channel's permission overwrites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn permission_overwrites(
        &self,
        channel_id: ChannelId,
    ) -> ChannelPermissionOverwriteResource<'a> {
        ChannelPermissionOverwriteResource::new(self.0, channel_id)
    }

    /// Work with a channel's pins.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn pins(&self, channel_id: ChannelId) -> ChannelPinResource<'a> {
        ChannelPinResource::new(self.0, channel_id)
    }

    /// Work with a channel's webhooks.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn webhooks(&self, channel_id: ChannelId) -> ChannelWebhookResource<'a> {
        ChannelWebhookResource::new(self.0, channel_id)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelResource<'_>: Clone, Debug, Send, Sync);
}
