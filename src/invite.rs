use twilight_http::{client::Client, request::channel::invite::GetInvite};

/// Work with invites.
#[derive(Clone, Debug)]
pub struct InviteResource<'a>(&'a Client);

impl<'a> InviteResource<'a> {
    /// Create a resource instance to work with invites.
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Get an invite.
    pub const fn get(&self, code: &'a str) -> GetInvite<'a> {
        self.0.invite(code)
    }
}

#[cfg(test)]
mod tests {
    use super::InviteResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InviteResource<'_>: Clone, Debug, Send, Sync);
}
