use twilight_http::{client::Client, request::user::GetCurrentUserConnections};

/// Work with a current user's connections.
#[derive(Clone, Debug)]
pub struct UserMeConnectionResource<'a>(&'a Client);

impl<'a> UserMeConnectionResource<'a> {
    /// Create a resource instance to work with the current user's connections.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// List the current user's connections.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetCurrentUserConnections<'a> {
        self.0.current_user_connections()
    }
}

#[cfg(test)]
mod tests {
    use super::UserMeConnectionResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(UserMeConnectionResource<'_>: Clone, Debug, Send, Sync);
}
