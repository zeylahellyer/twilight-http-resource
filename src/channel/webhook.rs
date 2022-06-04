use twilight_http::{
    client::Client,
    request::channel::webhook::{CreateWebhook, GetChannelWebhooks},
};
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_validate::request::ValidationError;

/// Work with a channel's webhooks.
#[derive(Clone, Debug)]
pub struct ChannelWebhookResource<'a>(&'a Client, Id<ChannelMarker>);

impl<'a> ChannelWebhookResource<'a> {
    /// Create a resource instance to work with a channel's webhooks.
    pub const fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self(client, channel_id)
    }

    /// List a channel's webhooks.
    pub const fn list(&self) -> GetChannelWebhooks<'a> {
        self.0.channel_webhooks(self.1)
    }

    /// Create a channel webhook.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_webhook`] for error information.
    ///
    /// [`Client::create_webhook`]: twilight_http::Client::create_webhook
    pub fn post(&self, name: &'a str) -> Result<CreateWebhook<'a>, ValidationError> {
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
