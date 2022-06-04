use twilight_http::{
    client::Client,
    request::channel::{DeleteChannelPermission, UpdateChannelPermission},
};
use twilight_model::{
    http::permission_overwrite::PermissionOverwrite,
    id::{marker::ChannelMarker, Id},
};

/// Work with a channel's permission overwrites.
#[derive(Clone, Debug)]
pub struct ChannelPermissionOverwriteResource<'a>(&'a Client, Id<ChannelMarker>);

impl<'a> ChannelPermissionOverwriteResource<'a> {
    /// Create a resource instance to work with a channel's permission
    /// overwrites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self(client, channel_id)
    }

    /// Get a channel's permission overwrites.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self) -> DeleteChannelPermission<'a> {
        self.0.delete_channel_permission(self.1)
    }

    /// Put a channel permission overwrite.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn put(
        &self,
        permission_overwrite: &PermissionOverwrite,
    ) -> UpdateChannelPermission<'a> {
        self.0
            .update_channel_permission(self.1, permission_overwrite)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelPermissionOverwriteResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ChannelPermissionOverwriteResource<'_>: Clone, Debug, Send, Sync);
}
