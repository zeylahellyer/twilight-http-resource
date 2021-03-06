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
use twilight_model::id::{marker::ChannelMarker, Id};

/// Work with channels.
#[derive(Clone, Debug)]
pub struct ChannelResource<'a>(&'a Client);

impl<'a> ChannelResource<'a> {
    /// Create a resource instance to work with channels.
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Delete a channel.
    pub const fn delete(&self, id: Id<ChannelMarker>) -> DeleteChannel<'a> {
        self.0.delete_channel(id)
    }

    /// Get a channel.
    pub const fn get(&self, id: Id<ChannelMarker>) -> GetChannel<'a> {
        self.0.channel(id)
    }

    /// Update a channel.
    pub const fn patch(&self, id: Id<ChannelMarker>) -> UpdateChannel<'a> {
        self.0.update_channel(id)
    }
}

/// RPC calls.
impl<'a> ChannelResource<'a> {
    /// Follow a channel.
    pub const fn follow(
        &self,
        id: Id<ChannelMarker>,
        webhook_channel_id: Id<ChannelMarker>,
    ) -> FollowNewsChannel<'a> {
        self.0.follow_news_channel(id, webhook_channel_id)
    }

    /// Trigger a typing indicator in a channel.
    pub const fn typing(&self, id: Id<ChannelMarker>) -> CreateTypingTrigger<'a> {
        self.0.create_typing_trigger(id)
    }
}

/// 1:M channel relationships.
impl<'a> ChannelResource<'a> {
    /// Work with a channel's invites.
    pub const fn invites(&self, channel_id: Id<ChannelMarker>) -> ChannelInviteResource<'a> {
        ChannelInviteResource::new(self.0, channel_id)
    }

    /// Work with a channel's messages.
    pub const fn messages(&self, channel_id: Id<ChannelMarker>) -> ChannelMessageResource<'a> {
        ChannelMessageResource::new(self.0, channel_id)
    }

    /// Work with a channel's permission overwrites.
    pub const fn permission_overwrites(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> ChannelPermissionOverwriteResource<'a> {
        ChannelPermissionOverwriteResource::new(self.0, channel_id)
    }

    /// Work with a channel's pins.
    pub const fn pins(&self, channel_id: Id<ChannelMarker>) -> ChannelPinResource<'a> {
        ChannelPinResource::new(self.0, channel_id)
    }

    /// Work with a channel's webhooks.
    pub const fn webhooks(&self, channel_id: Id<ChannelMarker>) -> ChannelWebhookResource<'a> {
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
