use twilight_http::{
    client::Client,
    request::channel::{CreatePin, DeletePin, GetPins},
};
use twilight_model::id::{ChannelId, MessageId};

/// Work with a channel's pins.
#[derive(Clone, Debug)]
pub struct ChannelPinResource<'a>(&'a Client, ChannelId);

impl<'a> ChannelPinResource<'a> {
    /// Create a resource instance to work with a channel's pins.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, channel_id: ChannelId) -> Self {
        Self(client, channel_id)
    }

    /// Delete a channel pin.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self, message_id: MessageId) -> DeletePin<'a> {
        self.0.delete_pin(self.1, message_id)
    }

    /// List a channel's pins.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetPins<'a> {
        self.0.pins(self.1)
    }

    /// Create a channel pin.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(&self, message_id: MessageId) -> CreatePin<'a> {
        self.0.create_pin(self.1, message_id)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelPinResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelPinResource<'_>: Clone, Debug, Send, Sync);
}
