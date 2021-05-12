//! Work with users and their relationships.

pub mod me;

mod private_channel;

pub use self::{me::UserMeResource, private_channel::UserPrivateChannelResource};

use twilight_http::{client::Client, request::user::GetUser};
use twilight_model::id::UserId;

/// Work with users.
#[derive(Clone, Debug)]
pub struct UserResource<'a>(&'a Client);

impl<'a> UserResource<'a> {
    /// Create a resource instance to work with users.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Get a user.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn get(&self, id: UserId) -> GetUser<'a> {
        self.0.user(id)
    }
}

/// 1:1 user relationships.
impl<'a> UserResource<'a> {
    /// Work with the current user.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn me(&self) -> UserMeResource<'a> {
        UserMeResource::new(self.0)
    }

    /// Work with a user.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn private_channel(&self, user_id: UserId) -> UserPrivateChannelResource<'a> {
        UserPrivateChannelResource::new(self.0, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::UserResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(UserResource<'_>: Clone, Debug, Send, Sync);
}
