use twilight_http::{
    client::Client,
    request::channel::webhook::{DeleteWebhookMessage, ExecuteWebhook, UpdateWebhookMessage},
};
use twilight_model::id::{
    marker::{MessageMarker, WebhookMarker},
    Id,
};

/// Work with a webhook's messages.
#[derive(Clone, Debug)]
pub struct WebhookMessageResource<'a>(&'a Client, Id<WebhookMarker>, &'a str);

impl<'a> WebhookMessageResource<'a> {
    /// Create a resource instance to work with a webhook's messages.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, webhook_id: Id<WebhookMarker>, token: &'a str) -> Self {
        Self(client, webhook_id, token)
    }

    /// Delete a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(self, message_id: Id<MessageMarker>) -> DeleteWebhookMessage<'a> {
        self.0.delete_webhook_message(self.1, self.2, message_id)
    }

    /// Update a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(self, message_id: Id<MessageMarker>) -> UpdateWebhookMessage<'a> {
        self.0.update_webhook_message(self.1, self.2, message_id)
    }

    /// Create a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(self, id: Id<WebhookMarker>) -> ExecuteWebhook<'a> {
        self.0.execute_webhook(id, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::WebhookMessageResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(WebhookMessageResource<'_>: Clone, Debug, Send, Sync);
}
