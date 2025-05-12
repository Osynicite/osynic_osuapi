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

# 📜 Features

- **Support for both old and new APIs**: Supports all V1 endpoints and most V2 endpoints (except for Chat, Comments, Forums, and undocumented interfaces)
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
```

Then you can use it in your code~

For example, the following code completes the CCG authentication and retrieves user information for "peppy". You can also find the corresponding example code `peppy.rs` in the `examples` directory.

```rust
// Client Credentials Grant and Get Peppy's User Info
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

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
    println!("osu_account_id: {}", peppy.id);
    println!("username: {}", peppy.username);
    println!("join_date: {}", peppy.join_date.unwrap_or_default());
    println!("country_code: {}", peppy.country.as_ref().map_or("None".to_string(), |c| c.code.clone()));
    println!("country_name: {}", peppy.country.as_ref().map_or("None".to_string(), |c| c.name.clone()));
    println!("cover_url: {}", peppy.cover_url.unwrap_or_default());

    Ok(())
}
```

# 🍕 API Checklist

Examples can be run using `cargo run --example example_name`

## V1

This section is categorized based on the API classes from the [V1 Official Documentation](https://github.com/ppy/osu-api/wiki).

Interface modules can be found in `src/v1/interface`, with corresponding implementations in `src/v1/client/request/api` or `src/v1/client/gloo/api`.

| API              | Support | Note             | Example Name | Module Name   |
| ---------------- | ------- | ---------------- | ------------ | ------------- |
| /get_beatmaps    | ✅       | Get beatmaps     | `gb`         | `beatmap`     |
| /get_user        | ✅       | Get user         | `gu`         | `user`        |
| /get_user_best   | ✅       | Get user's best  | `gub`        | `user`        |
| /get_user_recent | ✅       | Get user's recent| `gur`        | `user`        |
| /get_match       | ✅       | Get match        | `gm`         | `multiplayer` |
| /get_scores      | ✅       | Get scores       | `gss`        | `scores`      |
| /get_replay      | ✅       | Get replay       | `gr`         | `replay`      |

## V2

This section is categorized based on the API classes from the [V2 Official Documentation](https://osu.ppy.sh/docs/index.html).

Interface modules can be found in `src/v2/interface`, with corresponding implementations in `src/v2/client/request/api`.

| Category       | Total APIs | Supported        | Notes        | Module Name     |
| -------------- | ---------- | ---------------- | ------------ | --------------- |
| Authentication | 4          | 4 ✅             | OAuth & Auth | `oauth`         |
| Beatmaps       | 7          | 7 ✅             | Beatmap API  | `beatmaps`      |
| Beatmapsets    | 3          | 2 ⚠️403 Forbidden | Beatmapset API| `beatmapsets`   |
| Changelog      | 3          | 3 ✅             | Changelog API| `changelog`     |
| Chat           | 11         | 0 🈳             | Chat API     | `chat`          |
| Comments       | 7          | 0 🈳             | Comments API | `comments`      |
| Events         | 1          | 1 ✅             | Events API   | `events`        |
| Forums         | 8          | 0 🈳             | Forums API   | `forums`        |
| Home           | 1          | 1 ✅             | Home API     | `search`        |
| Matches        | 1          | 2 ✅             | Matches API  | `matches`       |
| Multiplayer    | 4          | 2 ⚠️403 Forbidden | Multiplayer API| `multiplayer`   |
| News           | 2          | 2 ✅             | News API     | `news`          |
| Notifications  | 2          | 0 ❌403 Forbidden | Notifications API| `notifications` |
| Rankings       | 3          | 3 ✅             | Rankings API | `rankings`      |
| Scores         | 1          | 1 ✅             | Scores API   | `scores`        |
| Users          | 7          | 7 ✅             | Users API    | `users`         |
| Wiki           | 1          | 1 ✅             | Wiki API     | `wiki`          |

### Authentication

| API                     | Support | Note                                                                                                      | Example Name |
| ----------------------- | ------- | --------------------------------------------------------------------------------------------------------- | ------------ |
| /get_token_with_code    | ✅       | Short for Authorization Code Grant. Requires user browser OAuth authorization to get code for token request. No client_secret needed | `acg`      |
| /get_token_without_code | ✅       | Short for Client Credentials Grant. Directly requests token without user authorization. client_secret must be set in environment variables | `ccg`      |
| /refresh_token          | ✅       | Refresh token using refresh_token obtained with CCG authentication                                         | `refresh`    |
| /revoke_current_token   | ✅       | Revoke current token                                                                                       | `revoke`     |

### Beatmaps

| API                     | Support | Note                     | Example Name |
| ----------------------- | ------- | ------------------------ | ------------ |
| /get_beatmap            | ✅       | Get beatmap              | `bg`         |
| /get_beatmap_attributes | ✅       | Get beatmap attributes   | `bga`        |
| /get_beatmaps           | ✅       | Get multiple beatmaps    | `bgs`        |
| /get_scores             | ✅       | Get beatmap scores       | `bgss`       |
| /get_solo_scores        | ✅       | Get beatmap scores (Legacy)| `bgssn`      |
| /get_user_score         | ✅       | Get user score           | `bgus`       |
| /get_user_scores        | ✅       | Get multiple user scores | `bguss`      |

### Beatmapsets

| API             | Support | Note                 | Example Name |
| --------------- | ------- | -------------------- | ------------ |
| /download       | ❌       | Download beatmapset (lazer) | `bsd`  |
| /get_beatmapset | ✅       | Get beatmapset       | `bsg`        |
| /search         | ✅       | Search beatmapsets    | `bss`        |

### Changelog

| API                     | Support | Note               | Example Name |
| ----------------------- | ------- | ------------------ | ------------ |
| /get_changelog_build    | ✅       | Get changelog      | `cbg`        |
| /get_changelog_listing  | ✅       | Get changelog list | `clg`        |
| /lookup_changelog_build | ✅       | Look up changelog  | `cbl`        |

### Chat

| API                      | Support | Note                  | Example Name |
| ------------------------ | ------- | --------------------- | ------------ |
| /chat_keepalive          | 🈳       | Keep connection alive | `chk`        |
| /create_new_pm           | 🈳       | Create new PM         | `chpc`       |
| /get_updates             | 🈳       | Get updates           | `chug`       |
| /get_channel_messages    | 🈳       | Get channel messages  | `chmg`       |
| /send_message_to_channel | 🈳       | Send message          | `chms`       |
| /join_channel            | 🈳       | Join channel          | `chj`        |
| /leave_channel           | 🈳       | Leave channel         | `chl`        |
| /mark_channel_as_read    | 🈳       | Mark channel as read  | `chmr`       |
| /get_channel_list        | 🈳       | Get channel list      | `chlg`       |
| /create_channel          | 🈳       | Create channel        | `chc`        |
| /get_channel             | 🈳       | Get channel           | `chg`        |

### Comments

| API                  | Support | Note            | Example Name |
| -------------------- | ------- | --------------- | ------------ |
| /get_comments        | 🈳       | Get comments    | `csg`        |
| /post_comment        | 🈳       | Post comment    | `cp`         |
| /get_comment         | 🈳       | Get comment     | `cg`         |
| /edit_comment        | 🈳       | Edit comment    | `ce`         |
| /delete_comment      | 🈳       | Delete comment  | `cd`         |
| /add_comment_vote    | 🈳       | Add vote        | `cva`        |
| /remove_comment_vote | 🈳       | Remove vote     | `cvr`        |

### Events

| API         | Support | Note       | Example Name |
| ----------- | ------- | ---------- | ------------ |
| /get_events | ✅       | Get events | `events`     |

### Forums

| API                  | Support | Note                       | Example Name |
| -------------------- | ------- | -------------------------- | ------------ |
| /reply_topic         | 🈳       | Reply to topic             | `ftr`        |
| /get_topics_listing  | 🈳       | Get topics list            | `ftlg`       |
| /create_topic        | 🈳       | Create topic               | `ftc`        |
| /get_topic_and_posts | 🈳       | Get topic and posts        | `ftpg`       |
| /edit_topic          | 🈳       | Edit topic                 | `fte`        |
| /edit_post           | 🈳       | Edit post                  | `fpe`        |
| /get_forum_listing   | 🈳       | Get forum list             | `flg`        |
| /get_forum_and_topic | 🈳       | Get forum and topic        | `ftg`        |

### Home

| API     | Support | Note     | Example Name |
| ------- | ------- | -------- | ------------ |
| /search | ✅       | Get home | `search`     |

### Matches

| API                  | Support | Note           | Example Name |
| -------------------- | ------- | -------------- | ------------ |
| /get_matches_listing | ✅       | Get match list | `mlg`        |
| /get_match           | ✅       | Get match      | `mg`         |

### Multiplayer

| API                    | Support           | Note               | Example Name |
| ---------------------- | ----------------- | ------------------ | ------------ |
| /get_user_high_score   | ❌403 Forbidden    | Get user high score| `muhsg`      |
| /get_scores            | ✅                 | Get multiple scores| `mssg`       |
| /get_score             | ❌403 Forbidden    | Get score          | `msg`        |
| /get_multiplayer_rooms | ✅                 | Get rooms          | `mrg`        |

### News

| API               | Support | Note           | Example Name |
| ----------------- | ------- | -------------- | ------------ |
| /get_news_listing | ✅       | Get news list  | `nlg`        |
| /get_news_post    | ✅       | Get news       | `npg`        |

### Notifications

| API                         | Support           | Note              | Example Name |
| --------------------------- | ----------------- | ----------------- | ------------ |
| /get_notifications          | ❌403 Forbidden    | Get notifications | `ng`         |
| /mark_notifications_as_read | ❌403 Forbidden    | Mark as read      | `nm`         |

### Rankings

| API                 | Support | Note                | Example Name |
| ------------------- | ------- | ------------------- | ------------ |
| /get_ranking        | ✅       | Get ranking         | `rg`         |
| /get_kudosu_ranking | ✅       | Get Kudosu ranking  | `rkg`        |
| /get_spotlights     | ✅       | Get spotlights      | `rsg`        |

### Scores

| API         | Support | Note                  | Example Name |
| ----------- | ------- | --------------------- | ------------ |
| /get_scores | ✅       | Get up to 1000 scores | `scores`     |

### Users

| API                                                | Support | Note                          | Example Name |
| -------------------------------------------------- | ------- | ----------------------------- | ------------ |
| /get_own_data                                      | ✅       | Get own user info (CCG)       | `me`         |
| /get_user_kudosu                                   | ✅       | Get user Kudosu               | `ukg`        |
| /get_user_scores                                   | ✅       | Get user scores               | `ussg`       |
| /get_user_beatmaps, /get_user_beatmaps_most_played | ✅       | Get user beatmap info         | `ubsg`       |
| /get_user_recent_activity                          | ✅       | Get user recent activity      | `urag`       |
| /get_user, /get_user_by_username                   | ✅       | Get user info                 | `ug`         |
| /get_users                                         | ✅       | Get multiple users info       | `usg`        |

### Wiki

| API            | Support | Note           | Example Name |
| -------------- | ------- | -------------- | ------------ |
| /get_wiki_page | ✅       | Get Wiki page  | `wiki`       |

# ❤️ Acknowledgements

Originally, this project was intended to directly use the [rosu-v2](https://crates.io/crates/rosu-v2) library, but at that time, I noticed that `rosu-v2` hadn't been updated for several months, and I wasn't very comfortable with its project organization and usage patterns (possibly because `rosu-v2` has a four-year history, with many early Rust code patterns in the library that aren't very convenient to modify directly), so I decided to start from scratch and write my own;

During the development of `osynic_osuapi`, I still referenced the interface design of `rosu-v2` (though not adopted) and some type choices (such as the selection between u64 and u32). Thanks to the authors of [rosu-v2](https://crates.io/crates/rosu-v2)!

The `rosu-v2` project is based on the [MIT License](./licenses/LICENSE-rosu-v2), with the project license located in `licenses/LICENSE-rosu-v2`

# 🤝 Contribution Guidelines

This library is basically a module developed for the Osynic application, but it is also a complete Rust encapsulation of the osu!api.

Currently, the V1 API is fully supported, and the V2 API is almost complete(except for the chat, comment, forum, and undocumented modules).

The library is still in the early stages of development, and there may be some bugs or missing features.

So, if there is any problem with the code, or if you have any suggestions, please submit a PR or Issue, and I will deal with it as soon as possible~

If you want to contribute code, please follow these rules:

- Follow the official Rust coding specifications
- New features must be accompanied by test cases
- Run `cargo fmt` and `cargo clippy` before submitting

# 📜 License

This project is open source based on the [MIT License](LICENSE), please respect the original author's copyright.
