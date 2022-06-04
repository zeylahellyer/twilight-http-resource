//! Work with user private channels.

use twilight_http::{client::Client, request::user::CreatePrivateChannel};
use twilight_model::id::{marker::UserMarker, Id};

/// Work with users.
#[derive(Clone, Debug)]
pub struct UserPrivateChannelResource<'a>(&'a Client, Id<UserMarker>);

impl<'a> UserPrivateChannelResource<'a> {
    /// Create a resource instance to work with a user's private channels.
    pub const fn new(client: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self(client, user_id)
    }

    /// Create a private channel with a user.
    pub const fn post(&self) -> CreatePrivateChannel<'a> {
        self.0.create_private_channel(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::UserPrivateChannelResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(UserPrivateChannelResource<'_>: Clone, Debug, Send, Sync);
}
