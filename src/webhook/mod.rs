//! Work with webhooks and their relationships.

mod message;

pub use self::message::WebhookMessageResource;

use twilight_http::{
    client::Client,
    request::channel::webhook::{DeleteWebhook, GetWebhook, UpdateWebhook, UpdateWebhookWithToken},
};
use twilight_model::id::WebhookId;

/// Work with webhooks.
#[derive(Clone, Debug)]
pub struct WebhookResource<'a>(&'a Client);

impl<'a> WebhookResource<'a> {
    /// Create a resource instance to work with webhooks.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Delete a webhook.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn delete(&self, id: WebhookId) -> DeleteWebhook<'a> {
        self.0.delete_webhook(id)
    }

    /// Get a webhook.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self, id: WebhookId) -> GetWebhook<'a> {
        self.0.webhook(id)
    }

    /// Update a webhook.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn patch(&self, id: WebhookId) -> UpdateWebhook<'a> {
        self.0.update_webhook(id)
    }
}

/// RPC calls.
impl<'a> WebhookResource<'a> {
    /// Update a webhook with a token.
    pub fn patch_with_token(
        &self,
        id: WebhookId,
        token: impl Into<String>,
    ) -> UpdateWebhookWithToken<'a> {
        self.0.update_webhook_with_token(id, token)
    }
}

/// 1:M webhook relationships.
impl<'a> WebhookResource<'a> {
    /// Work with a webhook's messages.
    pub fn messages(&self, id: WebhookId, token: impl Into<String>) -> WebhookMessageResource<'a> {
        WebhookMessageResource::new(self.0, id, token)
    }
}

#[cfg(test)]
mod tests {
    use super::WebhookResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(WebhookResource<'_>: Clone, Debug, Send, Sync);
}
