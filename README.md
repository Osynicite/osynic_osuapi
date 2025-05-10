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

# OSU!API 官方文档

- [V1文档](https://github.com/ppy/osu-api/wiki)
- [V2文档](https://osu.ppy.sh/docs/index.html)

# API检查表

可通过`cargo run --exmaple 示例名`来运行API对应示例

## V1

| API              | 支持 | 备注             | 示例名 |
| ---------------- | ---- | ---------------- | ------ |
| /get_beatmaps    | ✅    | 获取谱面         | `gb`   |
| /get_user        | ✅    | 获取用户         | `gu`   |
| /get_user_best   | ✅    | 获取用户最佳成绩 | `gub`  |
| /get_user_recent | ✅    | 获取用户最近成绩 | `gur`  |
| /get_match       | ✅    | 获取比赛         | `gm`   |
| /get_scores      | ✅    | 获取谱面成绩     | `gss`  |
| /get_replay      | ✅    | 获取回放         | `gr`   |

## V2

本条目基于[官方文档](https://osu.ppy.sh/docs/index.html)的API大类进行划分，分类如下

其中接口模块对应可以在`src/v2/interface`中找到，相应实现则在`src/v2/client/request/api`或者`src/v2/client/gloo/api`中可以找到

| 大类           | API数量 | 备注        | 模块名        |
| -------------- | ------- | ----------- | ------------- |
| Authentication | 3       | OAuth与认证 | oauth          |
| Beatmaps       | 4       | 谱面API     | beatmaps      |
| Beatmapsets    | 4       | 谱面集API   | beatmapsets   |
| Changelogs     | 1       | 变更日志API | changelogs    |
| Chat           | 1       | 聊天API     | chat          |
| Comments       | 2       | 评论API     | comments      |
| Events         | 1       | 事件API     | events        |
| Forums         | 1       | 论坛API     | forums        |
| Home           | 1       | 首页API     | home          |
| Matches        | 1       | 比赛API     | matches       |
| Multiplayer    | 1       | 多人API     | multiplayer   |
| News           | 1       | 新闻API     | news          |
| Notifications  | 1       | 通知API     | notifications |
| Rankings       | 1       | 排行榜API   | rankings      |
| Scores         | 1       | 成绩API     | scores        |
| Search         | 1       | 搜索API     | search        |
| Users          | 1       | 用户API     | users         |
| Wiki           | 1       | Wiki API    | wiki          |

### Authentication

| API                     | 支持 | 备注                                                                                                        | 示例名 |
| ----------------------- | ---- | ----------------------------------------------------------------------------------------------------------- | ------ |
| /get_token_with_code    | ✅    | 即Authorization Code Grant的缩写，需要用户在浏览器OAuth授权来拿到code，进而来请求token，不需要client_secret | `acg`  |
| /get_token_without_code | ✅    | 即Client Credentials Grant的缩写，直接请求token，不需要用户授权，client_secret需要在环境变量中设置          | `ccg`  |
| /refresh_token         | ✅    | CCG认证下，通过拿到的refresh_token刷新token                                                                                                  | `refresh`   |
|/revoke_current_token| ✅    | 撤销当前token                                                                                              | `revoke`  |


# 🤝 贡献指南

这个库基本上只是为Osynic这个应用开发的一个模块，但是同时也是一个功能完整的osu!api的Rust封装，目前只做了v2的部分，v1的部分和WASM支持还没有做。

所以，如果代码有任何问题，或者你有任何建议，欢迎提交PR或者Issue，我会尽快处理~

如果你想贡献代码，请遵循以下规则：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
