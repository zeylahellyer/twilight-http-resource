use twilight_http::{client::Client, request::template::GetTemplate};

/// Work with templates.
#[derive(Clone, Debug)]
pub struct TemplateResource<'a>(&'a Client);

impl<'a> TemplateResource<'a> {
    /// Create a resource instance to work with templates.
    pub const fn new(client: &'a Client) -> Self {
        Self(client)
    }

    /// Get a template.
    pub const fn get(&self, template_code: &'a str) -> GetTemplate<'a> {
        self.0.get_template(template_code)
    }
}

#[cfg(test)]
mod tests {
    use super::TemplateResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TemplateResource<'_>: Clone, Debug, Send, Sync);
}
