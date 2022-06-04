//! Work with the current user and its relationships.

mod connection;
mod guild;

pub use self::{connection::UserMeConnectionResource, guild::UserMeGuildResource};

use twilight_http::{
    client::Client,
    request::user::{GetCurrentUser, UpdateCurrentUser},
};

/// Work with the current user.
#[derive(Clone, Debug)]
pub struct UserMeResource<'a>(&'a Client);

impl<'a> UserMeResource<'a> {
    /// Create a resource instance to work with the current user.
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Get the current user.
    pub const fn get(&self) -> GetCurrentUser<'a> {
        self.0.current_user()
    }

    /// Update the current user.
    pub const fn patch(&self) -> UpdateCurrentUser<'a> {
        self.0.update_current_user()
    }
}

/// 1:M user guild relationships.
impl<'a> UserMeResource<'a> {
    /// Work with the current user's guilds.
    pub const fn guilds(&self) -> UserMeGuildResource<'a> {
        UserMeGuildResource::new(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::UserMeResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(UserMeResource<'_>: Clone, Debug, Send, Sync);
}
