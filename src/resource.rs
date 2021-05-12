use crate::VoiceRegionResource;

use super::{
    ChannelResource, GatewayResource, GuildResource, InviteResource, TemplateResource,
    UserResource, WebhookResource,
};
use twilight_http::Client;

/// Create a resource tree based on an HTTP client.
pub trait Resource {
    /// Work with channels.
    fn channels(&self) -> ChannelResource<'_>;

    /// Work with gateways.
    fn gateways(&self) -> GatewayResource<'_>;

    /// Work with guilds.
    fn guilds(&self) -> GuildResource<'_>;

    /// Work with invites.
    fn invites(&self) -> InviteResource<'_>;

    /// Work with templates.
    fn templates(&self) -> TemplateResource<'_>;

    /// Work with users.
    fn users(&self) -> UserResource<'_>;

    /// Work with voice regions.
    fn voice_regions(&self) -> VoiceRegionResource<'_>;

    /// Work with webhooks.
    fn webhooks(&self) -> WebhookResource<'_>;
}

impl Resource for Client {
    /// Work with channels.
    fn channels(&self) -> ChannelResource<'_> {
        ChannelResource::new(self)
    }

    /// Work with gateways.
    fn gateways(&self) -> GatewayResource<'_> {
        GatewayResource::new(self)
    }

    /// Work with guilds.
    fn guilds(&self) -> GuildResource<'_> {
        GuildResource::new(self)
    }

    /// Work with invites.
    fn invites(&self) -> InviteResource<'_> {
        InviteResource::new(self)
    }

    /// Work with templates.
    fn templates(&self) -> TemplateResource<'_> {
        TemplateResource::new(self)
    }

    /// Work with users.
    fn users(&self) -> UserResource<'_> {
        UserResource::new(self)
    }

    /// Work with voice regions.
    fn voice_regions(&self) -> VoiceRegionResource<'_> {
        VoiceRegionResource::new(self)
    }

    /// Work with webhooks.
    fn webhooks(&self) -> WebhookResource<'_> {
        WebhookResource::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Resource;
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use twilight_http::Client;

    assert_impl_all!(Client: Resource);
    assert_obj_safe!(Client);
}
