use twilight_http::{client::Client, request::GetVoiceRegions};

/// Work with voice regions.
#[derive(Clone, Debug)]
pub struct VoiceRegionResource<'a>(&'a Client);

impl<'a> VoiceRegionResource<'a> {
    /// Create a resource instance to work with voice regions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// List the voice regions.
    #[must_use = "this is a builder and does nothing on its own"]
    pub fn list(&self) -> GetVoiceRegions<'a> {
        self.0.voice_regions()
    }
}

#[cfg(test)]
mod tests {
    use super::VoiceRegionResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(VoiceRegionResource<'_>: Clone, Debug, Send, Sync);
}
