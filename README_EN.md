<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsuapi</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osuapi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osuapi"/></a>
  <a href="https://docs.rs/osynic_osuapi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osuapi/0.1.0"/></a>
  <a href="https://github.com/osynicite/osynic_osuapi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/JWyvc6M5" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>

</p>

<p align="center">
    Osu!Api wrapper for Osynic
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# 📄 OSU!API Official Documentation

- [V1 Documentation](https://github.com/ppy/osu-api/wiki)
- [V2 Documentation](https://osu.ppy.sh/docs/index.html)

# 🧻 V1(WASM) Experience Website (Currently Still Facing CORS Issues)

[LeptosOsuapiPlayground](https://github.com/islatri/leptos_osuapi_playground) is a website quickly built with the [leptos](https://www.leptos.dev/) framework to demonstrate `osynic_osuapi`, primarily using V1's WASM client support (based on [gloo-net](https://crates.io/crates/gloo-net)). However, obviously, due to CORS issues, using the V1 API directly in the browser without a proxy will encounter cross-origin problems (since the WASM part makes requests from the browser frontend). Therefore, a relay server [osynic-cors.deno.dev](https://osynic-cors.deno.dev) was set up using [Deno](https://deno.dev), working with the WASM client's `proxy_url` to implement proxy requests.

The website is currently deployed on [osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/) via [Deno](deno.dev), but at this point it's basically just for viewing.

# 📜 Features

- **Support for both old and new APIs**: Supports all V1 endpoints and most V2 endpoints (except for the undocumented modules)
- **WASM Compatibility**: Provides WebAssembly support for V1 interfaces, allowing direct access to the OSU API from web applications (though you may encounter CORS issues)
- **Well-structured project**: Based on a three-module division of `client`, `interface`, and `model`; the `client` part aggregates `interface` interfaces and supports various HTTP clients for easy extension
- **Very complete example support**: The `examples` directory contains very complete example code and return data, see the [API Checklist](#-api-checklist) section below for details
- **Learn by example**: The best way to learn how to use this library is to directly view the rich example code in `examples`, or run `cargo run --example example_name` to see the corresponding return data. Once you get used to the style of the examples, you'll quickly be able to use it

# 🚀 Quick Start

## 1. Applying for OSU! API OAuth (V2) or Legacy API (V1)

You can apply for API access through your [osu settings page](https://osu.ppy.sh/home/account/edit). Simply navigate to either the OAuth (V2) or Legacy API (V1) section to complete your application.

## 2. Setting Up Environment Variables

Create a `.env` file in your project's root directory with the following content:

```env
# V2 API
CLIENT_ID="your_client_id"
CLIENT_SECRET="your_client_secret"
REDIRECT_URI="your_redirect_uri"
CODE="your_code"  # Required for Authorization Code Grant authentication

# V1 API
API_KEY="your_api_key"
```

After setting up this file, you can use the `dotenvy` dependency to read these environment variables from the `.env` file in your project.

### 3. Installing and Using the Library

First, add the dependency to your `Cargo.toml`:

```toml
[dependencies]
osynic_osuapi = "0.1.0"
# The default features are ["v1", "v2", "not-wasm"]. If you need to use this in a WASM environment, you need to disable the `not-wasm` feature and add the `wasm` feature, for example:
# osynic_osuapi = { version = "0.1.0", default-features = false, features = ["v1", "v2", "wasm"] }
```

Then you can use it in your code~

### Example 1: Complete CCG certification with V2 and obtain Peppy's user information

The following code is from `examples/peppy.rs`, you can directly run `cargo run --example peppy` to see the effect

```rust
// Client Credentials Grant and Get Peppy's User Info
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

// You can also import all the client and interface modules by prelude
// use osynic_osuapi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let client = OsynicOsuApiV2Client::default();
    let token = client
        .oauth
        .get_token_without_code(client_id.parse()?, &client_secret)
        .await?;
    println!("{:?}", token);

    let peppy = client
        .users
        .get_user_by_username("peppy", None, None)
        .await?;
    println!("{:?}", peppy);

    Ok(())
}
```

### Example 2: Check beatmap information with V1

The following code is from `examples/gb.rs`, you can directly run `cargo run --example gb` to see the effect

```rust
// Get beatmap by hash
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

// You can also import all the client and interface modules by prelude
// use osynic_osuapi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set.");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetBeatmapsParams::default().hash("69f77b0dfe67d288c1bf748f91ceb133".to_string());

    let beatmaps = client.beatmap.get_beatmaps(params).await?;
    println!("{:?}", beatmaps);

    Ok(())
}
```

# 🍕 API Checklist

Examples can be run using `cargo run --example example_name`

## [V1](https://github.com/ppy/osu-api/wiki)

This section is categorized based on the API classes from the [V1 Official Documentation](https://github.com/ppy/osu-api/wiki).

Interface modules can be found in `src/v1/interface`, with corresponding implementations in `src/v1/client/request/api` or `src/v1/client/gloo/api`.

| API              | Support | Note              | Example Name | Module Name   |
| ---------------- | ------- | ----------------- | ------------ | ------------- |
| /get_beatmaps    | ✅       | Get beatmaps      | `gb`         | `beatmap`     |
| /get_user        | ✅       | Get user          | `gu`         | `user`        |
| /get_user_best   | ✅       | Get user's best   | `gub`        | `user`        |
| /get_user_recent | ✅       | Get user's recent | `gur`        | `user`        |
| /get_match       | ✅       | Get match         | `gm`         | `multiplayer` |
| /get_scores      | ✅       | Get scores        | `gss`        | `scores`      |
| /get_replay      | ✅       | Get replay        | `gr`         | `replay`      |

## [V2](https://osu.ppy.sh/docs/index.html)

This section is categorized based on the API classes from the [V2 Official Documentation](https://osu.ppy.sh/docs/index.html).

Interface modules can be found in `src/v2/interface`, with corresponding implementations in `src/v2/client/request/api`.

| Category       | Total APIs | Supported        | Notes             | Module Name     |
| -------------- | ---------- | ---------------- | ----------------- | --------------- |
| Authentication | 4          | 4 ✅              | OAuth & Auth      | `oauth`         |
| Beatmaps       | 10         | 10 ✅             | Beatmap API       | `beatmaps`      |
| Beatmapsets    | 7          | 2 ⚠️403 Forbidden | Beatmapset API    | `beatmapsets`   |
| Changelog      | 3          | 3 ✅              | Changelog API     | `changelog`     |
| Chat           | 11         | 0 ❌403 Forbidden | Chat API          | `chat`          |
| Comments       | 7          | 2 ⚠️403 Forbidden | Comments API      | `comments`      |
| Events         | 1          | 1 ✅              | Events API        | `events`        |
| Forum          | 8          | 4 ⚠️403 Forbidden | Forum API         | `forum`         |
| Home           | 1          | 1 ✅              | Home API          | `search`        |
| Matches        | 1          | 2 ✅              | Matches API       | `matches`       |
| Multiplayer    | 4          | 2 ⚠️403 Forbidden | Multiplayer API   | `multiplayer`   |
| News           | 2          | 2 ✅              | News API          | `news`          |
| Notifications  | 2          | 0 ❌403 Forbidden | Notifications API | `notifications` |
| Rankings       | 3          | 3 ✅              | Rankings API      | `rankings`      |
| Scores         | 1          | 1 ✅              | Scores API        | `scores`        |
| Users          | 7          | 7 ✅              | Users API         | `users`         |
| Wiki           | 1          | 1 ✅              | Wiki API          | `wiki`          |

### [Authentication](https://osu.ppy.sh/docs/index.html#authentication)

| API                     | Support | Note                                                                                                                                       | Example Name |
| ----------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ | ------------ |
| /get_token_with_code    | ✅       | Short for Authorization Code Grant. Requires user browser OAuth authorization to get code for token request. No client_secret needed       | `acg`        |
| /get_token_without_code | ✅       | Short for Client Credentials Grant. Directly requests token without user authorization. client_secret must be set in environment variables | `ccg`        |
| /refresh_token          | ✅       | Refresh token using refresh_token obtained with CCG authentication                                                                         | `refresh`    |
| /revoke_current_token   | ✅       | Revoke current token                                                                                                                       | `revoke`     |

### [Beatmaps](https://osu.ppy.sh/docs/index.html#beatmaps)

| API                     | Support | Note                        | Example Name |
| ----------------------- | ------- | --------------------------- | ------------ |
| /get_beatmap_packs      | ✅       | Get beatmap packs           | `bpsg`       |
| /get_beatmap_pack       | ✅       | Get beatmap pack            | `bpg`        |
| /lookup_beatmap         | ✅       | Look up beatmap             | `bl`         |
| /get_beatmap            | ✅       | Get beatmap                 | `bg`         |
| /get_beatmap_attributes | ✅       | Get beatmap attributes      | `bga`        |
| /get_beatmaps           | ✅       | Get multiple beatmaps       | `bgs`        |
| /get_scores             | ✅       | Get beatmap scores          | `bgss`       |
| /get_solo_scores        | ✅       | Get beatmap scores (Legacy) | `bgssn`      |
| /get_user_score         | ✅       | Get user score              | `bgus`       |
| /get_user_scores        | ✅       | Get multiple user scores    | `bguss`      |

### [Beatmapsets](https://osu.ppy.sh/docs/index.html#beatmapsets)

| API                               | Support               | Note                              | Example Name |
| --------------------------------- | --------------------- | --------------------------------- | ------------ |
| /get_beatmapset_discussions_posts | 🈳(Unstable return)    | Get beatmapset disscussions posts | `bsdpg`      |
| /get_beatmapset_discussions_votes | 🈳(Unstable return)    | Get beatmapset disscussions votes | `bsdvg`      |
| /get_beatmapset_discussions       | 🈳(Unstable return)    | Get beatmapset disscussions       | `bsdg`       |
| /search                           | ✅                     | Search beatmapsets                | `bss`        |
| /lookup                           | 🈳(The doc is unclear) | Look up beatmapset                | `bsl`        |
| /get_beatmapset                   | ✅                     | Get beatmapset                    | `bsg`        |
| /download                         | ❌                     | Download beatmapset (lazer)       | `bsd`        |

### [Changelog](https://osu.ppy.sh/docs/index.html#changelog)

| API                     | Support | Note               | Example Name |
| ----------------------- | ------- | ------------------ | ------------ |
| /get_changelog_build    | ✅       | Get changelog      | `cbg`        |
| /get_changelog_listing  | ✅       | Get changelog list | `clg`        |
| /lookup_changelog_build | ✅       | Look up changelog  | `cbl`        |

### [Chat](https://osu.ppy.sh/docs/index.html#chat)

| API                      | Support        | Note                  | Example Name |
| ------------------------ | -------------- | --------------------- | ------------ |
| /chat_keepalive          | ❌403 Forbidden | Keep connection alive | `chk`        |
| /create_new_pm           | ❌403 Forbidden | Create new PM         | `chpc`       |
| /get_updates             | ❌403 Forbidden | Get updates           | `chug`       |
| /get_channel_messages    | ❌403 Forbidden | Get channel messages  | `chmg`       |
| /send_message_to_channel | ❌403 Forbidden | Send message          | `chms`       |
| /join_channel            | ❌403 Forbidden | Join channel          | `chj`        |
| /leave_channel           | ❌403 Forbidden | Leave channel         | `chl`        |
| /mark_channel_as_read    | ❌403 Forbidden | Mark channel as read  | `chmr`       |
| /get_channel_list        | ❌403 Forbidden | Get channel list      | `chlg`       |
| /create_channel          | ❌403 Forbidden | Create channel        | `chc`        |
| /get_channel             | ❌403 Forbidden | Get channel           | `chg`        |

### [Comments](https://osu.ppy.sh/docs/index.html#comments)

| API                  | Support        | Note           | Example Name |
| -------------------- | -------------- | -------------- | ------------ |
| /get_comments        | ✅              | Get comments   | `csg`        |
| /post_comment        | ❌403 Forbidden | Post comment   | `cp`         |
| /get_comment         | ✅              | Get comment    | `cg`         |
| /edit_comment        | ❌403 Forbidden | Edit comment   | `ce`         |
| /delete_comment      | ❌403 Forbidden | Delete comment | `cd`         |
| /add_comment_vote    | ❌403 Forbidden | Add vote       | `cva`        |
| /remove_comment_vote | ❌403 Forbidden | Remove vote    | `cvr`        |

### [Events](https://osu.ppy.sh/docs/index.html#events)

| API         | Support | Note       | Example Name |
| ----------- | ------- | ---------- | ------------ |
| /get_events | ✅       | Get events | `events`     |

### [Forum](https://osu.ppy.sh/docs/index.html#forum)

| API                  | Support           | Note                | Example Name |
| -------------------- | ----------------- | ------------------- | ------------ |
| /reply_topic         | ❌401 Unauthorized | Reply to topic      | `ftr`        |
| /get_topics_listing  | ✅                 | Get topics list     | `ftlg`       |
| /create_topic        | ❌401 Unauthorized | Create topic        | `ftc`        |
| /get_topic_and_posts | ✅                 | Get topic and posts | `ftpg`       |
| /edit_topic          | ❌403 Forbidden    | Edit topic          | `fte`        |
| /edit_post           | ❌403 Forbidden    | Edit post           | `fpe`        |
| /get_forum_listing   | ✅                 | Get forum list      | `flg`        |
| /get_forum_and_topic | ✅                 | Get forum and topic | `ftg`        |

### [Home](https://osu.ppy.sh/docs/index.html#home)

| API     | Support | Note     | Example Name |
| ------- | ------- | -------- | ------------ |
| /search | ✅       | Get home | `search`     |

### [Matches](https://osu.ppy.sh/docs/index.html#matches)

| API                  | Support | Note           | Example Name |
| -------------------- | ------- | -------------- | ------------ |
| /get_matches_listing | ✅       | Get match list | `mlg`        |
| /get_match           | ✅       | Get match      | `mg`         |

### [Multiplayer](https://osu.ppy.sh/docs/index.html#multiplayer)

| API                    | Support        | Note                | Example Name |
| ---------------------- | -------------- | ------------------- | ------------ |
| /get_user_high_score   | ❌403 Forbidden | Get user high score | `muhsg`      |
| /get_scores            | ✅              | Get multiple scores | `mssg`       |
| /get_score             | ❌403 Forbidden | Get score           | `msg`        |
| /get_multiplayer_rooms | ✅              | Get rooms           | `mrg`        |

### [News](https://osu.ppy.sh/docs/index.html#news)

| API               | Support | Note          | Example Name |
| ----------------- | ------- | ------------- | ------------ |
| /get_news_listing | ✅       | Get news list | `nlg`        |
| /get_news_post    | ✅       | Get news      | `npg`        |

### [Notifications](https://osu.ppy.sh/docs/index.html#notifications)

| API                         | Support        | Note              | Example Name |
| --------------------------- | -------------- | ----------------- | ------------ |
| /get_notifications          | ❌403 Forbidden | Get notifications | `ng`         |
| /mark_notifications_as_read | ❌403 Forbidden | Mark as read      | `nm`         |

### [Rankings](https://osu.ppy.sh/docs/index.html#rankings)

| API                 | Support | Note               | Example Name |
| ------------------- | ------- | ------------------ | ------------ |
| /get_ranking        | ✅       | Get ranking        | `rg`         |
| /get_kudosu_ranking | ✅       | Get Kudosu ranking | `rkg`        |
| /get_spotlights     | ✅       | Get spotlights     | `rsg`        |

### [Scores](https://osu.ppy.sh/docs/index.html#scores)

| API         | Support | Note                  | Example Name |
| ----------- | ------- | --------------------- | ------------ |
| /get_scores | ✅       | Get up to 1000 scores | `scores`     |

### [Users](https://osu.ppy.sh/docs/index.html#users)

| API                                                | Support | Note                     | Example Name |
| -------------------------------------------------- | ------- | ------------------------ | ------------ |
| /get_own_data                                      | ✅       | Get own user info (CCG)  | `me`         |
| /get_user_kudosu                                   | ✅       | Get user Kudosu          | `ukg`        |
| /get_user_scores                                   | ✅       | Get user scores          | `ussg`       |
| /get_user_beatmaps, /get_user_beatmaps_most_played | ✅       | Get user beatmap info    | `ubsg`       |
| /get_user_recent_activity                          | ✅       | Get user recent activity | `urag`       |
| /get_user, /get_user_by_username                   | ✅       | Get user info            | `ug`         |
| /get_users                                         | ✅       | Get multiple users info  | `usg`        |

### [Wiki](https://osu.ppy.sh/docs/index.html#wiki)

| API            | Support | Note          | Example Name |
| -------------- | ------- | ------------- | ------------ |
| /get_wiki_page | ✅       | Get Wiki page | `wiki`       |

# ❤️ Acknowledgements

Originally, this project was intended to directly use the [rosu-v2](https://crates.io/crates/rosu-v2) library, but at that time, I noticed that `rosu-v2` hadn't been updated for several months, and I wasn't very comfortable with its project organization and usage patterns (possibly because `rosu-v2` has a four-year history, with many early Rust code patterns in the library that aren't very convenient to modify directly), so I decided to start from scratch and write my own;

During the development of `osynic_osuapi`, I still referenced the interface design of `rosu-v2` (though not adopted) and some type choices (such as the selection between u64 and u32). Thanks to the authors of [rosu-v2](https://crates.io/crates/rosu-v2)!

The `rosu-v2` project is based on the [MIT License](./licenses/LICENSE-rosu-v2), with the project license located in `licenses/LICENSE-rosu-v2`

# 🤝 Contribution Guidelines

This library is basically a module developed for the Osynic application, but it is also a complete Rust encapsulation of the osu!api.

Currently, the V1 API is fully supported, and the V2 API is almost complete(except for the undocumented modules).

The library is still in the early stages of development, and there may be some bugs or missing features.

So, if there is any problem with the code, or if you have any suggestions, please submit a PR or Issue, and I will deal with it as soon as possible~

If you want to contribute code, please follow these rules:

- Follow the official Rust coding specifications
- New features must be accompanied by test cases
- Run `cargo fmt` and `cargo clippy` before submitting

# 📜 License

This project is open source based on the [MIT License](LICENSE), please respect the original author's copyright.
