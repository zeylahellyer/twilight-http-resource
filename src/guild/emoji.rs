use twilight_http::{
    client::Client,
    request::guild::emoji::{CreateEmoji, DeleteEmoji, GetEmoji, GetEmojis, UpdateEmoji},
};
use twilight_model::id::{EmojiId, GuildId};

/// Work with a guild's emojis.
#[derive(Clone, Debug)]
pub struct GuildEmojiResource<'a>(&'a Client, GuildId);

impl<'a> GuildEmojiResource<'a> {
    /// Create a resource instance to work with a guild's emojis.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn new(client: &'a Client, guild_id: GuildId) -> Self {
        Self(client, guild_id)
    }

    /// Delete a guild emoji.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn delete(&self, emoji_id: EmojiId) -> DeleteEmoji<'a> {
        self.0.delete_emoji(self.1, emoji_id)
    }

    /// Get a guild emoji.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn get(&self, emoji_id: EmojiId) -> GetEmoji<'a> {
        self.0.emoji(self.1, emoji_id)
    }

    /// List a guild's emojis.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn list(&self) -> GetEmojis<'a> {
        self.0.emojis(self.1)
    }

    /// Update a guild emoji.
    #[must_use = "this is a builder and does nothing on its own"]
    pub const fn patch(&self, emoji_id: EmojiId) -> UpdateEmoji<'a> {
        self.0.update_emoji(self.1, emoji_id)
    }

    /// Create a guild emoji.
    pub const fn post(&self, name: &'a str, image: &'a str) -> CreateEmoji<'a> {
        self.0.create_emoji(self.1, name, image)
    }
}

#[cfg(test)]
mod tests {
    use super::GuildEmojiResource;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(GuildEmojiResource<'_>: Clone, Debug, Send, Sync);
}
