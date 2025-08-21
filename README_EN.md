<!-- markdownlint-disable MD033 MD041 MD045 MD026-->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsuapi</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osuapi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osuapi"/></a>
  <a href="https://docs.rs/osynic_osuapi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osuapi/0.1.1"/></a>
  <a href="https://osynic-osuapi.deno.dev" target="_blank"><img src="https://img.shields.io/badge/Deno-white?logo=deno&logoColor=black"/></a>
  <a href="https://github.com/osynicite/osynic_osuapi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>

</p>

<p align="center">
    🚀 High Performance · 🏗️ Well Structured · 🔧 Highly Extensible<br/>
    Complete Rust osu! API client library supporting both WASM and Native environments
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

<hr />

# 📚 Table of Contents

- [📄 OSU!API Official Documentation](#-osuapi-official-documentation)
- [🧻 API Experience Website](#-api-experience-website)
- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
- [🍕 API Checklist](#-api-checklist)
- [❤️ Acknowledgements](#️-acknowledgements)
- [⚠️ Special Attention](#️-special-attention)
- [🤝 Contribution Guidelines](#-contribution-guidelines)
- [📜 License](#-license)

# 📄 OSU!API Official Documentation

- [V1 Documentation](https://github.com/ppy/osu-api/wiki)
- [V2 Documentation](https://osu.ppy.sh/docs/index.html)

# 🧻 [API Experience Website](https://osynic-osuapi.deno.dev/)

[![OsynicOsuapiEN.png](https://s2.loli.net/2025/05/15/qlgvPVuZhM5Gcod.png)](https://osynic-osuapi.deno.dev/)

## Website Features

**🌐 Online Experience**: An online demonstration platform for `osynic_osuapi` built with the [leptos](https://www.leptos.dev/) framework

**✨ Core Features**:

- WASM client demonstrations for both V1 and V2 APIs
- Network requests based on [gloo-net](https://crates.io/crates/gloo-net)
- CORS cross-origin issue resolution via [osynic-cors.deno.dev](https://osynic-cors.deno.dev) proxy
- Multi-language support: Chinese, English, Japanese, Korean, German, French, Russian

**🚀 Deployment**: Deployed on [osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/) using [Deno](https://deno.dev)

> **💡 Technical Note**: Due to browser CORS restrictions, WASM clients need to access the osu! API through a proxy server

# ✨ Features

- **🔄 Complete API Support**: Full support for all V1 endpoints + most V2 endpoints (except undocumented interfaces)
- **🌐 WASM Compatibility**: WebAssembly support for both V1 and V2 interfaces, enabling direct use in web applications
- **🏗️ Well-Architected**: Three-layer module design based on `client`, `interface`, and `model` for easy extension and maintenance
- **📖 Comprehensive Examples**: Rich example code and response data in the `examples` directory, see [API Checklist](#-api-checklist)
- **🎓 Example-Driven Learning**: Quick start by viewing example code or running `cargo run --example example_name`

# 🚀 Quick Start

## Step 1: Apply for OSU! API Authorization

Visit your [osu! settings page](https://osu.ppy.sh/home/account/edit) and apply for the appropriate API authorization:

- **V2 API**: Apply in the "OAuth" section
- **V1 API**: Apply in the "Legacy API" section

## Step 2: Configure Environment Variables

Create a `.env` file in your project root directory:

```env
# V2 API Configuration
CLIENT_ID="your_client_id"
CLIENT_SECRET="your_client_secret"
REDIRECT_URI="your_redirect_uri"
CODE="your_code"  # Required for Authorization Code Grant authentication

# V1 API Configuration
API_KEY="your_api_key"
```

## Step 3: Add Dependencies

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
osynic_osuapi = "0.1.1"
dotenvy = "0.15"  # For reading .env files

# WASM Environment Configuration (Optional)
# osynic_osuapi = { version = "0.1.1", default-features = false, features = ["v1", "v2", "wasm"] }
```

> **💡 Feature Description**:
>
> - Default features: `["v1", "v2", "not-wasm"]` (for Native environments)
> - WASM environment: Disable `not-wasm` and enable `wasm` feature

## Usage Examples

### Example 1: V2 API - CCG Authentication and User Information

Use Client Credentials Grant authentication to get peppy's user information:

```rust
// examples/peppy.rs - Run cargo run --example peppy to see the effect
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

// You can also import all client and interface modules by prelude
// use osynic_osuapi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    
    let client = OsynicOsuApiV2Client::default();
    
    // Get access token
    let token = client
        .oauth
        .get_token_without_code(client_id.parse()?, &client_secret)
        .await?;
    println!("Token: {:?}", token);

    // Get user information
    let peppy = client
        .users
        .get_user_by_username("peppy", None, None)
        .await?;
    println!("User: {:?}", peppy);

    Ok(())
}
```

### Example 2: V1 API - Query Beatmap Information

Query beatmap details by beatmap hash:

```rust
// examples/gb.rs - Run cargo run --example gb to see the effect
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set.");
    let client = OsynicOsuApiV1Client::new(api_key);
    
    // Query beatmap by hash
    let params = GetBeatmapsParams::default()
        .hash("69f77b0dfe67d288c1bf748f91ceb133".to_string());

    let beatmaps = client.beatmap.get_beatmaps(params).await?;
    println!("Beatmaps: {:?}", beatmaps);

    Ok(())
}
```

> **🎯 More Examples**: Check the `examples/` directory for complete examples, or run `cargo run --example example_name` to see actual results.

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

# ⚠️ Special Attention

When using this library, the most common issues stem from changes in the official osu! API entity structure:

## Common Issue Types

- **🔄 Entity Structure Changes**: The structure of osu! API may change at any time, but official documentation updates may not be timely
- **📝 Return Field Changes**: Return fields of some interfaces may change, especially for less frequently used endpoints
- **❓ Unexpected Null Values**: Some fields may return null under certain conditions, but are not marked as optional in the documentation

## Issue Reporting

If you encounter parsing errors or type mismatches during use, please submit an Issue with the following information:

1. **API endpoint used**
2. **Request parameters**
3. **Error message or exception stack trace**

I will handle and update the library as quickly as possible to adapt to API changes. Most models in this library are built based on actual request response results, but there may still be omissions or errors. Your feedback is crucial for improving this library!

# 🤝 Contribution Guidelines

## Project Overview

This library is primarily developed for the Osynic application, but also serves as a complete Rust wrapper for the osu! API.

## Current Status

✅ **Completed**:

- Most V1 and V2 API interfaces (except undocumented interfaces)
- WASM support for both V1 and V2

⚠️ **In Development**: May have bugs or incomplete features

## How to Contribute

We welcome PRs and Issues! If you find any problems or have suggestions for improvement, please follow these guidelines:

### Code Contribution Standards

- **Coding Standards**: Follow official Rust coding conventions
- **Testing Requirements**: New features must include test cases
- **Code Quality**: Run `cargo fmt` and `cargo clippy` before submitting
- **Documentation Updates**: Update relevant documentation and examples when necessary

### Issue Submission Guidelines

- Describe the specific scenario of the problem
- Provide reproduction steps and error information
- Include relevant API endpoints and parameter information

# 📜 License

This project is open source based on the [MIT License](LICENSE), please respect the original author's copyright.
