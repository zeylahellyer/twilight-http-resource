use twilight_http::{
    client::Client,
    request::channel::webhook::{CreateWebhook, GetChannelWebhooks},
};
use twilight_model::id::ChannelId;

/// Work with a channel's webhooks.
#[derive(Clone, Debug)]
pub struct ChannelWebhookResource<'a>(&'a Client, ChannelId);

impl<'a> ChannelWebhookResource<'a> {
    /// Create a resource instance to work with a channel's webhooks.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, channel_id: ChannelId) -> Self {
        Self(client, channel_id)
    }

    /// List a channel's webhooks.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetChannelWebhooks<'a> {
        self.0.channel_webhooks(self.1)
    }

    /// Create a channel webhook.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(&self, name: &'a str) -> CreateWebhook<'a> {
        self.0.create_webhook(self.1, name)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelWebhookResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelWebhookResource<'_>: Clone, Debug, Send, Sync);
}
