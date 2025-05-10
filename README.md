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

|API|支持|备注|示例名|
|---|---|---|---|
|/get_beatmaps|✅|获取谱面|`gb`|
|/get_user|✅|获取用户|`gu`|
|/get_user_best|✅|获取用户最佳成绩|`gub`|
|/get_user_recent|✅|获取用户最近成绩|`gur`|
|/get_match|✅|获取比赛|`gm`|
|/get_scores|✅|获取谱面成绩|`gss`|
|/get_replay|✅|获取回放|`gr`|

# 🤝 贡献指南

这个库基本上只是为Osynic这个应用开发的一个模块，但是同时也是一个功能完整的osu!api的Rust封装，目前只做了v2的部分，v1的部分和WASM支持还没有做。

所以，如果代码有任何问题，或者你有任何建议，欢迎提交PR或者Issue，我会尽快处理~

如果你想贡献代码，请遵循以下规则：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

# 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。
