<!-- cargo-sync-readme start -->

# twilight-http-resource

Interface for `twilight-http` that attempts to conform to the mental models
of resources, entities, and relationships within REST APIs.

Instead of using methods on a client named `delete_ban`, this library allows
you to model structure based on the relationship of the ban to the guild.

Internally resource calls are simply calling their `twilight-http` Client
equivalents and return the builders that are in turn returned by the Client.

Supports Twilight 0.11.

# Examples

Delete a message in a channel:

```rust,no_run
use std::env;
use twilight_http_resource::Resource;
use twilight_http::Client;
use twilight_model::id::{marker::{ChannelMarker, MessageMarker}, Id};

let client = Client::new(env::var("DISCORD_TOKEN")?);
let channel_id = Id::new(1);
let message_id = Id::new(2);

client.channels().messages(channel_id).delete(message_id).exec().await?;
```

What's going on here is that `Resource::channels` is returning a
`ChannelResource` instance. This `ChannelResource` has methods like
`get``ChannelResource::get` for getting a channel by ID or
`ChannelResource::messages` to retrieve a resource for working
with the relationship of messages. `ChannelMessageResource` in turn
contains methods like `list``ChannelMessageResource::list` or
`ChannelMessageResource::post`.

# Installation

`twilight-http-resource` is not currently on `crates.io`; you can instead
install it in your `Cargo.toml` file like so:

```toml
[dependencies]
twilight-http-resource = { git = "https://github.com/zeylahellyer/twilight-http-resource" }
```

<!-- cargo-sync-readme end -->
