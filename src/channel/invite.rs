use twilight_http::{
    client::Client,
    request::channel::invite::{CreateInvite, GetChannelInvites},
};
use twilight_model::id::{marker::ChannelMarker, Id};

/// Work with a channel's invites.
#[derive(Clone, Debug)]
pub struct ChannelInviteResource<'a>(&'a Client, Id<ChannelMarker>);

impl<'a> ChannelInviteResource<'a> {
    /// Create a resource instance to work with a channel's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self(client, channel_id)
    }

    /// List a channel's invites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetChannelInvites<'a> {
        self.0.channel_invites(self.1)
    }

    /// Create a channel invite.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn post(&self) -> CreateInvite<'a> {
        self.0.create_invite(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelInviteResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelInviteResource<'_>: Clone, Debug, Send, Sync);
}
