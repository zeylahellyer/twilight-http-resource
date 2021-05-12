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
use twilight_model::id::{ChannelId, MessageId};

/// Work with a channel's messages.
#[derive(Clone, Debug)]
pub struct ChannelMessageResource<'a>(&'a Client, ChannelId);

impl<'a> ChannelMessageResource<'a> {
    /// Create a resource instance to work with a channel's messages.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: ChannelId) -> Self {
        Self(client, guild_id)
    }

    /// Delete multiple channel messages.
    pub fn delete_list(&self, message_ids: impl Into<Vec<MessageId>>) -> DeleteMessages<'a> {
        self.0.delete_messages(self.1, message_ids)
    }

    /// Delete a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, message_id: MessageId) -> DeleteMessage<'a> {
        self.0.delete_message(self.1, message_id)
    }

    /// Get a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self, message_id: MessageId) -> GetMessage<'a> {
        self.0.message(self.1, message_id)
    }

    /// List a channel's messages.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetChannelMessages<'a> {
        self.0.channel_messages(self.1)
    }

    /// Update a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(&self, message_id: MessageId) -> UpdateMessage<'a> {
        self.0.update_message(self.1, message_id)
    }

    /// Create a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(&self) -> CreateMessage<'a> {
        self.0.create_message(self.1)
    }
}

/// RPC calls.
impl<'a> ChannelMessageResource<'a> {
    /// Crosspost a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn crosspost(&self, message_id: MessageId) -> CrosspostMessage<'a> {
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
