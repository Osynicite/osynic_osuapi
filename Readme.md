gloo和request两个模块的划分，前者基于gloo_net，后者基于reqwest库。

但是我这个作为一个osu!api的wrapper，两个模块要实现一样的api包装，但是client肯定是不同的，并且发送请求的方法也不一样，可以抽象成一个trait的两种实现了

我现在正在想我这个osynic_osuapi这个库如何划分项目结构，目前
[features]
default = ["v2","wasm"]
v1 = ["tokio","reqwest"]
v2 = ["tokio","reqwest"]
wasm = ["gloo-net"]

暂时做了模块划分，用来实现The following endpoints are currently supported:

    beatmaps/lookup: A specific beatmap including its beatmapset
    beatmaps: Up to 50 beatmaps at once including their beatmapsets
    beatmaps/{map_id}/attributes: The difficulty attributes of a beatmap
    beatmaps/{map_id}/scores: The global score leaderboard for a beatmap
    beatmaps/{map_id}/scores/users/{user_id}[/all]: Get (all) top score(s) of a user on a beatmap. Defaults to the play with the max score, not pp
    beatmapsets/{mapset_id}: The beatmapset including all of its difficulty beatmaps
    beatmapsets/events: Various events around a beatmapset such as status, genre, or language updates, kudosu transfers, or new issues
    beatmapsets/search: Search for beatmapsets; the same search as on the osu! website
    comments: Most recent comments and their replies up to two levels deep
    events: Collection of events in order of creation time
    forums/topics/{topic_id}: A forum topic and its posts
    matches: List of currently open multiplayer lobbies
    matches/{match_id}: More specific data about a specific multiplayer lobby including participating players and occured events
    me[/{mode}]: Detailed info about the authenticated user [in the specified mode] (requires OAuth)
    news: Recent news

就像上面这些API，目前，请你帮我设计一下项目结构和编写思路吧！

- api
- model
- client
  - gloo
  - request

- oauth
- beatmaps
  - pack
- beatmapsets
- user

其他的暂不需要，像是chat之类的，这些确实实现一下就好

接口，组合，实现，

我在想trait和sturct是不是都能聚合

IGetBeatmaps 是trait

GlooGetBeatmaps 是struct

最后要用的是GlooClient

哦哦其实我还是混淆了一些，应该是

IBeatmaps,IUser 是trait
GlooBeatmaps,GlooUser 是struct

GlooClient 是一个结构体，包含了GlooBeatmaps,GlooUser

https://osu.ppy.sh/oauth/authorize?client_id=36104&redirect_uri=https%3A%2F%2Fapi.osynic.moe%2Fapi%2Fo_oauth&response_type=code&scope=public+identify&state=osynic_ccb

主要用这个来测试，然后把拿到的code写到env里面

今天完善，现在需要快速完成CMS，也是数据库和架构等等，还是分BC吗？

免费版的话，暂时提供每个用户可以在5台设备之间同步？每个device上有不同时间点的songs，可以恢复到任意时间点的状态？

当然，还有用户自制的合集可以用来分享等等，但是其他用户不能看到这个用户的devices

- CMS
  - device
    - device_id
    - login_time
    - location
    - ip
    - song_id[]
  - song
    - song_id
    - song_name
    - artist_name
    - no_video
    - mapper_name 这五个是卡片的主要显示信息
  - collection
    - collection_id
    - collection_name
    - song_id[]
    - user_id
    - download_times

CMS明天写吧，今天先把这byd登陆注册和刷新otoken以及mq的通信写完

肯定还是微服务，不过看salvo了，用这个框架来写微服务本身也就已经是一个挑战了，大概

整洁，洋葱，六边形都试过了，DDD传统分层，在这也行吧，难度应该还低一些，还行

