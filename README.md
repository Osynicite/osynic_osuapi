<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsuapi</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osuapi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osuapi"/></a>
  <a href="https://docs.rs/osynic_osuapi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osuapi/0.1.11"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osuapi" target="_blank"><img src="https://img.shields.io/npm/v/@osynicite/osynic-osuapi"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osuapi" target="_blank"><img src="https://img.shields.io/npm/dm/@osynicite/osynic-osuapi"/></a>
  <a href="https://hakochest.github.io/osynic-osuapi/" target="_blank"><img src="https://img.shields.io/badge/Typedoc-Documentation-blue"/></a>
  <a href="https://osynic-osuapi.deno.dev" target="_blank"><img src="https://img.shields.io/badge/Deno-white?logo=deno&logoColor=black"/></a>
  <a href="https://github.com/osynicite/osynic_osuapi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>
</p>

<p align="center">
    🚀 高性能 · 🏗️ 结构优良 · 🔧 易于扩展<br/>
    功能完整的 Rust osu! API 客户端库，支持 WASM 和 Native 环境
</p>

<p align="center">
  <a href="README.md">🇨🇳 中文</a> ·
  <a href="README_EN.md">🇺🇸 English</a>
</p>

<hr />

# 📚 目录

- [📄 OSU!API 官方文档](#-osuapi-官方文档)
- [🧻 API体验网站](#-api体验网站)
- [✨ 特性](#-特性)
- [🚀 快速开始](#-快速开始)
- [🍕 API检查表](#-api检查表)
- [❤️ 鸣谢](#️-鸣谢)
- [⚠️ 特别注意](#️-特别注意)
- [🤝 贡献指南](#-贡献指南)
- [📜 开源协议](#-开源协议)

# 📄 OSU!API 官方文档

- [V1文档](https://github.com/ppy/osu-api/wiki)
- [V2文档](https://osu.ppy.sh/docs/index.html)

# 🧻 [API体验网站](https://osynic-osuapi.deno.dev/)

[![OSUAPIV1CN.png](https://s2.loli.net/2025/11/05/ahR96WKd5u1UrVy.png)](https://osynic-osuapi.deno.dev/)

## 网站特色

**🌐 在线体验**：基于 [leptos](https://www.leptos.dev/) 框架构建的 `osynic_osuapi` 在线体验平台

**✨ 核心功能**：

- 支持 V1 和 V2 API 的 WASM 客户端演示
- 基于 [gloo-net](https://crates.io/crates/gloo-net) 的网络请求
- 通过 [osynic-cors.deno.dev](https://osynic-cors.deno.dev) 代理解决 CORS 跨域问题
- 多语言支持：中文、英语、日语、韩语、德语、法语、俄语

**🚀 部署方式**：使用 [Deno](https://deno.dev) 部署在 [osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/)

> **💡 技术说明**：由于浏览器 CORS 限制，WASM 客户端需要通过代理服务器来访问 osu! API

# ✨ 特性

- **🔄 新旧 API 全支持**: 完整支持 V1 所有端点 + V2 大部分端点（除文档未归类接口）
- **🌐 WASM 兼容性**: V1 和 V2 接口均提供 WebAssembly 支持，可直接在网页应用中使用
- **🏗️ 架构设计优良**: 基于 `client`、`interface`、`model` 三层模块设计，易于扩展和维护
- **📖 完整示例支持**: `examples` 目录提供丰富的示例代码和返回数据，详见 [API检查表](#-api检查表)
- **🎓 示例驱动学习**: 通过查看示例代码或运行 `cargo run --example 示例名` 快速上手

# 🚀 快速开始

## 步骤一：申请 OSU! API 授权

访问您的 [osu! 设置页面](https://osu.ppy.sh/home/account/edit)，在以下位置申请相应的 API 授权：

- **V2 API**: 在 "OAuth" 或 "开放授权" 部分申请
- **V1 API**: 在 "Legacy API" 或 "旧版本 API" 部分申请

## 步骤二：配置环境变量

在项目根目录创建 `.env` 文件：

```env
# V2 API 配置
CLIENT_ID="你的client_id"
CLIENT_SECRET="你的client_secret"
REDIRECT_URI="你的redirect_uri"
CODE="你的code"  # Authorization Code Grant 认证时需要

# V1 API 配置
API_KEY="你的api_key"
```

## 步骤三：添加依赖

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
osynic_osuapi = "0.1.11"
dotenvy = "0.15.7"  # 用于读取 .env 文件

# WASM 环境配置（可选）
# osynic_osuapi = { version = "0.1.11", default-features = false, features = ["v1", "v2", "wasm"] }
```

> **💡 特性说明**：
>
> - 默认特性：`["v1", "v2", "not-wasm"]`（适用于 Native 环境）
> - WASM 环境：需要关闭 `not-wasm` 并启用 `wasm` 特性

## 使用示例

### 示例一：V2 API - CCG 认证获取用户信息

使用 Client Credentials Grant 认证方式获取 peppy 的用户信息：

```rust
// examples/peppy.rs - 可运行 cargo run --example peppy 查看效果
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

// 也可以通过 prelude 导入所有客户端和接口模块
// use osynic_osuapi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    
    let client = OsynicOsuApiV2Client::default();
    
    // 获取访问令牌
    let token = client
        .oauth
        .get_token_without_code(client_id.parse()?, &client_secret)
        .await?;
    println!("Token: {:?}", token);

    // 获取用户信息
    let peppy = client
        .users
        .get_user_by_username("peppy", None, None)
        .await?;
    println!("User: {:?}", peppy);

    Ok(())
}
```

### 示例二：V1 API - 查询谱面信息

通过谱面哈希值查询谱面详细信息：

```rust
// examples/gb.rs - 可运行 cargo run --example gb 查看效果
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set.");
    let client = OsynicOsuApiV1Client::new(api_key);
    
    // 通过哈希值查询谱面
    let params = GetBeatmapsParams::default()
        .hash("69f77b0dfe67d288c1bf748f91ceb133".to_string());

    let beatmaps = client.beatmap.get_beatmaps(params).await?;
    println!("Beatmaps: {:?}", beatmaps);

    Ok(())
}
```

> **🎯 更多示例**：查看 `examples/` 目录获取完整示例，或运行 `cargo run --example 示例名` 查看实际效果。

# 🍕 API检查表

可通过 `cargo run --example 示例名` 来运行API对应示例

## [V1](https://github.com/ppy/osu-api/wiki)

本条目基于[V1官方文档](https://github.com/ppy/osu-api/wiki)的API大类进行划分，分类如下

其中接口模块对应可以在 `src/v1/interface` 中找到，相应实现则在 `src/v1/client/request/api` 或者 `src/v1/client/gloo/api` 中可以找到

| API              | 支持 | 备注             | 示例名 | 模块名        |
| ---------------- | ---- | ---------------- | ------ | ------------- |
| /get_beatmaps    | ✅    | 获取谱面         | `gb`   | `beatmap`     |
| /get_user        | ✅    | 获取用户         | `gu`   | `user`        |
| /get_user_best   | ✅    | 获取用户最佳成绩 | `gub`  | `user`        |
| /get_user_recent | ✅    | 获取用户最近成绩 | `gur`  | `user`        |
| /get_match       | ✅    | 获取比赛         | `gm`   | `multiplayer` |
| /get_scores      | ✅    | 获取谱面成绩     | `gss`  | `scores`      |
| /get_replay      | ✅    | 获取回放         | `gr`   | `replay`      |

## [V2](https://osu.ppy.sh/docs/index.html)

本条目基于[V2官方文档](https://osu.ppy.sh/docs/index.html)的API大类进行划分，分类如下

其中接口模块对应可以在 `src/v2/interface` 中找到，相应实现则在 `src/v2/client/request/api` 中可以找到，示例代码和相应数据在 `src/v2/examples` 中可以找到

| 大类           | API总数 | API支持数        | 备注        | 模块名          |
| -------------- | ------- | ---------------- | ----------- | --------------- |
| Authentication | 4       | 4 ✅              | OAuth与认证 | `oauth`         |
| Beatmaps       | 10      | 10 ✅             | 谱面API     | `beatmaps`      |
| Beatmapsets    | 7       | 2 ⚠️403 Forbidden | 谱面集API   | `beatmapsets`   |
| Changelog      | 3       | 3 ✅              | 变更日志API | `changelog`     |
| Chat           | 11      | 0 ❌403 Forbidden | 聊天API     | `chat`          |
| Comments       | 7       | 2 ⚠️403 Forbidden | 评论API     | `comments`      |
| Events         | 1       | 1 ✅              | 事件API     | `events`        |
| Forum          | 8       | 4 ⚠️403 Forbidden | 论坛API     | `forum`         |
| Home           | 1       | 1 ✅              | 首页API     | `search`        |
| Matches        | 1       | 2 ✅              | 比赛API     | `matches`       |
| Multiplayer    | 4       | 2 ⚠️403 Forbidden | 多人API     | `multiplayer`   |
| News           | 2       | 2 ✅              | 新闻API     | `news`          |
| Notifications  | 2       | 0 ❌403 Forbidden | 通知API     | `notifications` |
| Rankings       | 3       | 3 ✅              | 排行榜API   | `rankings`      |
| Scores         | 1       | 1 ✅              | 成绩API     | `scores`        |
| Users          | 7       | 7 ✅              | 用户API     | `users`         |
| Wiki           | 1       | 1 ✅              | Wiki API    | `wiki`          |
| Friends        | 2       | 2 ✅              | 好友API     | `friends`       |

### [Authentication](https://osu.ppy.sh/docs/index.html#authentication)

| API                     | 支持 | 备注                                                                                                        | 示例名    |
| ----------------------- | ---- | ----------------------------------------------------------------------------------------------------------- | --------- |
| /get_token_with_code    | ✅    | 即Authorization Code Grant的缩写，需要用户在浏览器OAuth授权来拿到code，进而来请求token，不需要client_secret | `acg`     |
| /get_token_without_code | ✅    | 即Client Credentials Grant的缩写，直接请求token，不需要用户授权，client_secret需要在环境变量中设置          | `ccg`     |
| /refresh_token          | ✅    | CCG认证下，通过拿到的refresh_token刷新token                                                                 | `refresh` |
| /revoke_current_token   | ✅    | 撤销当前token                                                                                               | `revoke`  |

### [Beatmaps](https://osu.ppy.sh/docs/index.html#beatmaps)

| API                     | 支持 | 备注                 | 示例名  |
| ----------------------- | ---- | -------------------- | ------- |
| /get_beatmap_packs      | ✅    | 获取多个谱面包       | `bpsg`  |
| /get_beatmap_pack       | ✅    | 获取谱面包           | `bpg`   |
| /lookup_beatmap         | ✅    | 查阅谱面             | `bl`    |
| /get_beatmap            | ✅    | 获取谱面             | `bg`    |
| /get_beatmap_attributes | ✅    | 获取谱面属性         | `bga`   |
| /get_beatmaps           | ✅    | 获取多个谱面         | `bgs`   |
| /get_scores             | ✅    | 获取谱面成绩         | `bgss`  |
| /get_solo_scores        | ✅    | 获取谱面成绩(Legacy) | `bgssn` |
| /get_user_score         | ✅    | 获取用户成绩         | `bgus`  |
| /get_user_scores        | ✅    | 获取用户多个成绩     | `bguss` |

### [Beatmapsets](https://osu.ppy.sh/docs/index.html#beatmapsets)

| API                                | 支持          | 备注                 | 示例名  |
| ---------------------------------- | ------------- | -------------------- | ------- |
| /get_beatmapsets_discussions_posts | 🈳(不稳定接口) | 获取铺面集讨论区发布 | `bsdpg` |
| /get_beatmapsets_discussions_vote  | 🈳(不稳定接口) | 获取铺面集讨论区投票 | `bsdvg` |
| /get_beatmapsets_discussions       | 🈳(不稳定接口) | 获取铺面集讨论区     | `bsdg`  |
| /search                            | ✅             | 搜索谱面集           | `bss`   |
| /lookup                            | 🈳(文档不明)   | 查阅谱面集           | `bsl`   |
| /get_beatmapset                    | ✅             | 获取谱面集           | `bsg`   |
| /download                          | ❌             | 下载谱面集（lazer）  | `bsd`   |

### [Changelog](https://osu.ppy.sh/docs/index.html#changelog)

| API                     | 支持 | 备注             | 示例名 |
| ----------------------- | ---- | ---------------- | ------ |
| /get_changelog_build    | ✅    | 获取变更日志     | `cbg`  |
| /get_changelog_listing  | ✅    | 获取变更日志列表 | `clg`  |
| /lookup_changelog_build | ✅    | 查阅变更日志     | `cbl`  |

### [Chat](https://osu.ppy.sh/docs/index.html#chat)

| API                      | 支持           | 备注         | 示例名 |
| ------------------------ | -------------- | ------------ | ------ |
| /chat_keepalive          | ❌403 Forbidden | 保持连接     | `chk`  |
| /create_new_pm           | ❌403 Forbidden | 创建新私信   | `chpc` |
| /get_updates             | ❌403 Forbidden | 获取更新     | `chug` |
| /get_channel_messages    | ❌403 Forbidden | 获取频道消息 | `chmg` |
| /send_message_to_channel | ❌403 Forbidden | 发送消息     | `chms` |
| /join_channel            | ❌403 Forbidden | 加入频道     | `chj`  |
| /leave_channel           | ❌403 Forbidden | 离开频道     | `chl`  |
| /mark_channel_as_read    | ❌403 Forbidden | 标记频道已读 | `chmr` |
| /get_channel_list        | ❌403 Forbidden | 获取频道列表 | `chlg` |
| /create_channel          | ❌403 Forbidden | 创建频道     | `chc`  |
| /get_channel             | ❌403 Forbidden | 获取频道     | `chg`  |

### [Comments](https://osu.ppy.sh/docs/index.html#comments)

| API                  | 支持           | 备注         | 示例名 |
| -------------------- | -------------- | ------------ | ------ |
| /get_comments        | ✅              | 获取多条评论 | `csg`  |
| /post_comment        | ❌403 Forbidden | 发送评论     | `cp`   |
| /get_comment         | ✅              | 获取评论     | `cg`   |
| /edit_comment        | ❌403 Forbidden | 编辑评论     | `ce`   |
| /delete_comment      | ❌403 Forbidden | 删除评论     | `cd`   |
| /add_comment_vote    | ❌403 Forbidden | 投票         | `cva`  |
| /remove_comment_vote | ❌403 Forbidden | 撤销投票     | `cvr`  |

### [Events](https://osu.ppy.sh/docs/index.html#events)

| API         | 支持 | 备注     | 示例名   |
| ----------- | ---- | -------- | -------- |
| /get_events | ✅    | 获取事件 | `events` |

### [Forum](https://osu.ppy.sh/docs/index.html#forum)

| API                  | 支持              | 备注             | 示例名 |
| -------------------- | ----------------- | ---------------- | ------ |
| /reply_topic         | ❌401 Unauthorized | 回帖             | `ftr`  |
| /get_topics_listing  | ✅                 | 获取主题列表     | `ftlg` |
| /create_topic        | ❌401 Unauthorized | 创建主题         | `ftc`  |
| /get_topic_and_posts | ✅                 | 获取主题及其帖子 | `ftpg` |
| /edit_topic          | ❌403 Forbidden    | 编辑主题         | `fte`  |
| /edit_post           | ❌403 Forbidden    | 编辑帖子         | `fpe`  |
| /get_forum_listing   | ✅                 | 获取论坛列表     | `flg`  |
| /get_forum_and_topic | ✅                 | 获取论坛及其主题 | `ftg`  |

### [Home](https://osu.ppy.sh/docs/index.html#home)

| API     | 支持 | 备注     | 示例名   |
| ------- | ---- | -------- | -------- |
| /search | ✅    | 获取首页 | `search` |

### [Matches](https://osu.ppy.sh/docs/index.html#matches)

| API                  | 支持 | 备注         | 示例名 |
| -------------------- | ---- | ------------ | ------ |
| /get_matches_listing | ✅    | 获取比赛列表 | `mlg`  |
| /get_match           | ✅    | 获取比赛     | `mg`   |

### [Multiplayer](https://osu.ppy.sh/docs/index.html#multiplayer)

| API                    | 支持           | 备注         | 示例名  |
| ---------------------- | -------------- | ------------ | ------- |
| /get_user_high_score   | ❌403 Forbidden | 获取用户高分 | `muhsg` |
| /get_scores            | ✅              | 获取多个分数 | `mssg`  |
| /get_score             | ❌403 Forbidden | 获取分数     | `msg`   |
| /get_multiplayer_rooms | ✅              | 获取房间     | `mrg`   |

### [News](https://osu.ppy.sh/docs/index.html#news)

| API               | 支持 | 备注         | 示例名 |
| ----------------- | ---- | ------------ | ------ |
| /get_news_listing | ✅    | 获取新闻列表 | `nlg`  |
| /get_news_post    | ✅    | 获取新闻     | `npg`  |

### [Notifications](https://osu.ppy.sh/docs/index.html#notifications)

| API                         | 支持           | 备注     | 示例名 |
| --------------------------- | -------------- | -------- | ------ |
| /get_notifications          | ❌403 Forbidden | 获取通知 | `ng`   |
| /mark_notifications_as_read | ❌403 Forbidden | 标为已读 | `nm`   |

### [Rankings](https://osu.ppy.sh/docs/index.html#rankings)

| API                 | 支持 | 备注             | 示例名 |
| ------------------- | ---- | ---------------- | ------ |
| /get_ranking        | ✅    | 获取排行榜       | `rg`   |
| /get_kudosu_ranking | ✅    | 获取Kudosu排行榜 | `rkg`  |
| /get_spotlights     | ✅    | 获取聚光灯       | `rsg`  |

### [Scores](https://osu.ppy.sh/docs/index.html#scores)

| API         | 支持 | 备注               | 示例名   |
| ----------- | ---- | ------------------ | -------- |
| /get_scores | ✅    | 获取最多1000个成绩 | `scores` |

### [Users](https://osu.ppy.sh/docs/index.html#users)

| API                                                | 支持 | 备注                    | 示例名 |
| -------------------------------------------------- | ---- | ----------------------- | ------ |
| /get_own_data                                      | ✅    | 获取自己的用户信息(CCG) | `me`   |
| /get_user_kudosu                                   | ✅    | 获取用户Kudosu          | `ukg`  |
| /get_user_scores                                   | ✅    | 获取用户分数            | `ussg` |
| /get_user_beatmaps, /get_user_beatmaps_most_played | ✅    | 获取用户谱面信息        | `ubsg` |
| /get_user_recent_activity                          | ✅    | 获取用户最近活动        | `urag` |
| /get_user, /get_user_by_username                   | ✅    | 获取用户信息            | `ug`   |
| /get_users                                         | ✅    | 获取多个用户信息        | `usg`  |

### [Wiki](https://osu.ppy.sh/docs/index.html#wiki)

| API            | 支持 | 备注         | 示例名 |
| -------------- | ---- | ------------ | ------ |
| /get_wiki_page | ✅    | 获取Wiki页面 | `wiki` |

### [Friends](https://osu.ppy.sh/docs/index.html#get-apiv2friends)

| API                        | 支持 | 备注                        | 示例名 |
| -------------------------- | ---- | --------------------------- | ------ |
| /get_friends               | ✅    | 获取好友列表                | `fg`   |
| /get_friends_x_api_version | ✅    | 获取好友列表（指定API版本） | `fgx`  |

# ❤️ 鸣谢

最开始项目本来是打算直接用[rosu-v2](https://crates.io/crates/rosu-v2)这个库的，但是由于当时看到`rosu-v2`已经就大几个月没更新了，并且项目组织和使用方式也不太习惯（可能是`rosu-v2`至今已经有四年历史的缘故，库里面有很多早期Rust代码，也不是很方便直接修改），所以就另起炉灶决定自己写一个了；

在`osynic_osuapi`的开发过程中，还是参考了`rosu-v2`的接口设计（但并未沿用）和部分类型（比如u64和u32的选取），感谢[rosu-v2](https://crates.io/crates/rosu-v2)的作者们！

`rosu-v2`项目基于[MIT License](./licenses/LICENSE-rosu-v2)，项目证书放置在`licenses/LICENSE-rosu-v2`中

# ⚠️ 特别注意

使用本库时，最常见的问题来源于 osu! API 官方实体结构的变动：

## 常见问题类型

- **🔄 实体结构变动**：osu! API 的结构可能随时变化，官方文档更新可能不及时
- **📝 返回字段变动**：某些接口的返回字段可能发生变化，尤其是较少使用的端点  
- **❓ 异常空值**：某些字段可能在特定情况下返回 null，但文档中未标明为可选

## 问题反馈

如果您在使用过程中遇到解析错误或类型不匹配等问题，请直接提交 Issue 并附上：

1. **使用的 API 端点**
2. **请求参数**  
3. **错误信息或异常堆栈**

我会尽快处理并更新库以适应 API 的变化。本库的大部分模型都是基于实际请求返回结果构建的，但仍可能存在遗漏或错误。您的反馈对完善本库至关重要！

# 🤝 贡献指南

## 项目概述

本库主要为 Osynic 应用开发，同时也是一个功能完整的 osu! API Rust 封装库。

## 当前状态

✅ **已完成**：

- V1 和 V2 大部分 API 接口（除文档未归类的接口）
- V1 和 V2 的 WASM 支持

⚠️ **开发中**：可能存在 bug 或不完善的地方

## 如何贡献

欢迎提交 PR 或 Issue！如果您发现任何问题或有改进建议，请遵循以下规则：

### 代码贡献规范

- **编码规范**：遵循 Rust 官方编码规范
- **测试要求**：新增功能需附带测试用例  
- **代码质量**：提交前运行 `cargo fmt` 和 `cargo clippy`
- **文档更新**：必要时更新相关文档和示例

### Issue 提交指南

- 描述问题的具体场景
- 提供复现步骤和错误信息
- 附上相关的 API 端点和参数信息

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
