use twilight_http::{
    client::Client,
    request::channel::reaction::{DeleteAllReactions, GetReactions, RequestReactionType},
};
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

/// Work with a guild member's roles.
#[derive(Clone, Debug)]
pub struct ChannelMessageReactionResource<'a>(&'a Client, Id<ChannelMarker>, Id<MessageMarker>);

impl<'a> ChannelMessageReactionResource<'a> {
    /// Create a resource instance to work with a channel message's reactions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(
        client: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<MessageMarker>,
    ) -> Self {
        Self(client, channel_id, user_id)
    }

    /// Delete all reactions on a channel message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete_list(&self) -> DeleteAllReactions<'a> {
        self.0.delete_all_reactions(self.1, self.2)
    }

    /// List a channel message's reactions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self, emoji: &'a RequestReactionType<'a>) -> GetReactions<'a> {
        self.0.reactions(self.1, self.2, emoji)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelMessageReactionResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelMessageReactionResource<'_>: Clone, Debug, Send, Sync);
}
