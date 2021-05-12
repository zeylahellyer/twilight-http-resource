use twilight_http::{client::Client, request::GetGateway};

/// Work with gateways.
#[derive(Clone, Debug)]
pub struct GatewayResource<'a>(&'a Client);

impl<'a> GatewayResource<'a> {
    /// Create a resource instance to work with gateways.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Get a gateway.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self) -> GetGateway<'a> {
        self.0.gateway()
    }
}

#[cfg(test)]
mod tests {
    use super::GatewayResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GatewayResource<'_>: Clone, Debug, Send, Sync);
}
