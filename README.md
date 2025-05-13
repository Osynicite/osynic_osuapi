<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
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
    Osynic的OSU!API封装
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# 📄 OSU!API 官方文档

- [V1文档](https://github.com/ppy/osu-api/wiki)
- [V2文档](https://osu.ppy.sh/docs/index.html)

# 🧻 [API体验网站](https://osynic-osuapi.deno.dev/)

[![osynic-osuapi](https://s2.loli.net/2025/05/13/XrvbeEaPL3CWcRg.png)](https://osynic-osuapi.deno.dev/)

[LeptosOsuapiPlayground](https://github.com/islatri/leptos_osuapi_playground)是基于[leptos](https://www.leptos.dev/)框架快速搭建了一个`osynic_osuapi`体验网站，主要使用了的V1的WASM客户端支持（基于[gloo-net](https://crates.io/crates/gloo-net)）,不过很显然，由于CORS的问题，不代理直接在浏览器中使用V1的API会遇到跨域问题（毕竟WASM部分是浏览器前端发的请求嘛），所以用[Deno](https://deno.dev)来搭建了一个中转服务器[osynic-cors.deno.dev](https://osynic-cors.deno.dev)，配合WASM客户端的`proxy_url`来实现代理请求；

目前网站通过[Deno](https://deno.dev)部署在[osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/)上，目前基本上仅供观赏了

# 📜 特性

- **新旧 API 支持**: 支持 V1 的所有端点 以及 V2 的大部分端点（除了文档未归类的接口）
- **WASM 兼容性**: 为 V1 接口 提供了 WebAssembly 支持，支持直接从网页应用访问 OSU API（但是会遇到CORS）
- **项目结构良好**: 基于`client`、`interface`、`model`三重模块划分；`client`部分聚合`interface`接口并支持多种HTTP客户端，便于拓展
- **非常完整的示例支持**: 在`examples`目录下，我们提供了非常完整的示例代码与返回数据，详见下方的[API检查表](#-api检查表)部分
- **在示例中学习使用**: 学习使用本库的最佳方式就是直接查看`examples`中丰富的示例代码，或者直接运行`cargo run --example 示例名`来查看对应的返回数据，只要习惯示例的代码风格，很快就能上手使用

# 🚀 快速开始

## 1. 申请OSU!API的开放授权(V2)或者旧版本API(V1)

申请网址就在你的[osu设置页](https://osu.ppy.sh/home/account/edit)，在开放授权(V2)或者旧版本API(V1)中申请即可

## 2. 设置环境变量

在你的项目根目录下创建一个`.env`文件，内容如下

```env
# V2 API
CLIENT_ID="你的client_id"
CLIENT_SECRET="你的client_secret"
REDIRECT_URI="你的redirect_uri"
CODE="你的code" # Authorization Code Grant认证时需要

# V1 API
API_KEY="你的api_key"
```

然后我们就可以通过`dotenvy`依赖来读取`.env`中的环境变量了

## 3. 添加依赖并使用

首先在`Cargo.toml`中添加依赖

```toml
[dependencies]
osynic_osuapi = "0.1.0"
# 默认features是 ["v1", "v2", "not-wasm"]，如果需要在WASM环境中使用，需要关闭`not-wasm`特性，然后添加`wasm`特性，例如：
# osynic_osuapi = { version = "0.1.0", default-features = false, features = ["v1", "v2", "wasm"] }
```

然后在代码中使用即可~

### 示例一：用V2完成CCG认证并获取peppy的用户信息

下面的代码来自`examples/peppy.rs`，可以直接运行`cargo run --example peppy`来查看效果

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

### 示例二：用V1查阅谱面信息

下面的代码来自`examples/gb.rs`，可以直接运行`cargo run --example gb`来查看效果

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

# 🍕 API检查表

可通过`cargo run --exmaple 示例名`来运行API对应示例

## [V1](https://github.com/ppy/osu-api/wiki)

本条目基于[V1官方文档](https://github.com/ppy/osu-api/wiki)的API大类进行划分，分类如下

其中接口模块对应可以在`src/v1/interface`中找到，相应实现则在`src/v1/client/request/api`或者`src/v1/client/gloo/api`中可以找到

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

其中接口模块对应可以在`src/v2/interface`中找到，相应实现则在`src/v2/client/request/api`中可以找到，示例代码和相应数据在`src/v2/examples`中可以找到

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

| API             | 支持        | 备注                | 示例名 |
| --------------- | ----------- | ------------------- | ------ |
| /get_beatmapsets_discussions_posts| 🈳(不稳定接口)| 获取铺面集讨论区发布| `bsdpg` |
| /get_beatmapsets_discussions_vote| 🈳(不稳定接口)| 获取铺面集讨论区投票| `bsdvg` |
| /get_beatmapsets_discussions| 🈳(不稳定接口)| 获取铺面集讨论区| `bsdg` |
| /search         | ✅           | 搜索谱面集          | `bss`  |
| /lookup         | 🈳(文档不明) | 查阅谱面集          | `bsl`  |
| /get_beatmapset | ✅           | 获取谱面集          | `bsg`  |
| /download       | ❌           | 下载谱面集（lazer） | `bsd`  |

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

# ❤️ 鸣谢

最开始项目本来是打算直接用[rosu-v2](https://crates.io/crates/rosu-v2)这个库的，但是由于当时看到`rosu-v2`已经就大几个月没更新了，并且项目组织和使用方式也不太习惯（可能是`rosu-v2`至今已经有四年历史的缘故，库里面有很多早期Rust代码，也不是很方便直接修改），所以就另起炉灶决定自己写一个了；

在`osynic_osuapi`的开发过程中，还是参考了`rosu-v2`的接口设计（但并未沿用）和部分类型（比如u64和u32的选取），感谢[rosu-v2](https://crates.io/crates/rosu-v2)的作者们！

`rosu-v2`项目基于[MIT License](./licenses/LICENSE-rosu-v2)，项目证书放置在`licenses/LICENSE-rosu-v2`中

# 🤝 贡献指南

这个库基本上只是为Osynic这个应用开发的一个模块，但是同时也是一个功能完整的osu!api的Rust封装；

目前，v1和v2的大部分(除了文档未归类的接口)已经实现，v2的WASM支持暂时还没有做。

这个库仍然在开发中，可能会有一些bug或者不完善的地方；

所以，如果代码有任何问题，或者你有任何建议，欢迎提交PR或者Issue，我会尽快处理~

如果你想贡献代码，请遵循以下规则：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
