//! Work with channel messages and their relationships.

mod reaction;

pub use reaction::ChannelMessageReactionResource;

use twilight_http::{
    client::Client,
    request::channel::message::{
        CreateMessage, CrosspostMessage, DeleteMessage, DeleteMessages, GetChannelMessages,
        GetMessage, UpdateMessage,
    },
};
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

/// Work with a channel's messages.
#[derive(Clone, Debug)]
pub struct ChannelMessageResource<'a>(&'a Client, Id<ChannelMarker>);

impl<'a> ChannelMessageResource<'a> {
    /// Create a resource instance to work with a channel's messages.
    pub const fn new(client: &'a Client, guild_id: Id<ChannelMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Delete multiple channel messages.
    pub const fn delete_list(&self, message_ids: &'a [Id<MessageMarker>]) -> DeleteMessages<'a> {
        self.0.delete_messages(self.1, message_ids)
    }

    /// Delete a channel message.
    pub const fn delete(&self, message_id: Id<MessageMarker>) -> DeleteMessage<'a> {
        self.0.delete_message(self.1, message_id)
    }

    /// Get a channel message.
    pub const fn get(&self, message_id: Id<MessageMarker>) -> GetMessage<'a> {
        self.0.message(self.1, message_id)
    }

    /// List a channel's messages.
    pub const fn list(&self) -> GetChannelMessages<'a> {
        self.0.channel_messages(self.1)
    }

    /// Update a channel message.
    pub const fn patch(&self, message_id: Id<MessageMarker>) -> UpdateMessage<'a> {
        self.0.update_message(self.1, message_id)
    }

    /// Create a channel message.
    pub const fn post(&self) -> CreateMessage<'a> {
        self.0.create_message(self.1)
    }
}

/// RPC calls.
impl<'a> ChannelMessageResource<'a> {
    /// Crosspost a channel message.
    pub fn crosspost(&self, message_id: Id<MessageMarker>) -> CrosspostMessage<'a> {
        self.0.crosspost_message(self.1, message_id)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelMessageResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelMessageResource<'_>: Clone, Debug, Send, Sync);
}
