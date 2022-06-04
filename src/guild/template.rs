use twilight_http::{
    client::Client,
    request::template::{
        CreateTemplate, DeleteTemplate, GetTemplates, SyncTemplate, UpdateTemplate,
    },
};
use twilight_model::id::{marker::GuildMarker, Id};
use twilight_validate::request::ValidationError;

/// Work with a guild's templates.
#[derive(Clone, Debug)]
pub struct GuildTemplateResource<'a>(&'a Client, Id<GuildMarker>);

impl<'a> GuildTemplateResource<'a> {
    /// Create a resource instance to work with a guild's templates.
    pub const fn new(client: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild's template.
    pub fn delete(&self, code: &'a str) -> DeleteTemplate<'a> {
        self.0.delete_template(self.1, code)
    }

    /// List a guild's templates.
    pub fn list(&self) -> GetTemplates<'a> {
        self.0.get_templates(self.1)
    }

    /// Update a guild template.
    pub fn patch(&self, template_code: &'a str) -> UpdateTemplate<'a> {
        self.0.update_template(self.1, template_code)
    }

    /// Create a guild template.
    ///
    /// # Errors
    ///
    /// Refer to [`Client::create_template`] for error information.
    ///
    /// [`Client::create_template`]: twilight_http::Client::create_template
    pub fn post(&self, name: &'a str) -> Result<CreateTemplate<'a>, ValidationError> {
        self.0.create_template(self.1, name)
    }
}

/// RPC calls.
impl<'a> GuildTemplateResource<'a> {
    /// Sync a guild's template.
    pub fn sync(&self, code: &'a str) -> SyncTemplate<'a> {
        self.0.sync_template(self.1, code)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildTemplateResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildTemplateResource<'_>: Clone, Debug, Send, Sync);
}
