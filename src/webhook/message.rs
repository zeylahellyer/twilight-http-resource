use twilight_http::{
    client::Client,
    request::channel::webhook::{DeleteWebhookMessage, ExecuteWebhook, UpdateWebhookMessage},
};
use twilight_model::id::{MessageId, WebhookId};

/// Work with a webhook's messages.
#[derive(Clone, Debug)]
pub struct WebhookMessageResource<'a>(&'a Client, WebhookId, String);

impl<'a> WebhookMessageResource<'a> {
    /// Create a resource instance to work with a webhook's messages.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn new(client: &'a Client, webhook_id: WebhookId, token: impl Into<String>) -> Self {
        Self(client, webhook_id, token.into())
    }

    /// Delete a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(self, message_id: MessageId) -> DeleteWebhookMessage<'a> {
        self.0.delete_webhook_message(self.1, self.2, message_id)
    }

    /// Update a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(self, message_id: MessageId) -> UpdateWebhookMessage<'a> {
        self.0.update_webhook_message(self.1, self.2, message_id)
    }

    /// Create a webhook message.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn post(self, id: WebhookId) -> ExecuteWebhook<'a> {
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
