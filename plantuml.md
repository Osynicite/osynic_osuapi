# Osynic Osuapi - PlantUML Architecture Diagrams

This document contains C4 and UML diagrams for the Osynic Osuapi project - a high-performance, well-structured Rust osu! API client library.

## C4 System Context Diagram

```plantuml
@startuml C4_System_Context
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

SHOW_PERSON_OUTLINE()

Person(developer, "Developer", "Uses the Osynic Osuapi library")
System(osuapi, "Osynic Osuapi", "High-performance Rust osu! API client")
System_Ext(osu_api_v1, "osu! API v1", "Official osu! API endpoints")
System_Ext(osu_api_v2, "osu! API v2", "Official osu! API endpoints")

Rel(developer, osuapi, "Calls API through client")
Rel(osuapi, osu_api_v1, "HTTP requests")
Rel(osuapi, osu_api_v2, "HTTP requests")

@enduml
```

## C4 Container Diagram

```plantuml
@startuml C4_Container
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

System_Boundary(osuapi, "Osynic Osuapi") {
    Container(client_native, "Client (Native)", "Rust + Tokio + Reqwest", "Handles HTTP requests in native Rust environments")
    Container(client_wasm, "Client (WASM)", "Rust + WASM + Gloo", "Handles HTTP requests in WebAssembly environments")
    Container(interface, "API Interfaces", "Rust trait/impl", "Defines API endpoints for V1 and V2")
    Container(models, "Data Models", "Rust struct", "Represents osu! API data structures")
    Container(error, "Error Handling", "Rust enum", "Custom error types and error handling")
    Container(utils, "Utilities", "Rust functions", "Helper functions and common utilities")
}

Container(v1_api, "osu! API v1", "HTTP Endpoints", "Official osu! API v1")
Container(v2_api, "osu! API v2", "HTTP Endpoints", "Official osu! API v2")

Rel(client_native, v1_api, "HTTP GET/POST")
Rel(client_native, v2_api, "HTTP GET/POST")
Rel(client_wasm, v1_api, "HTTP Fetch")
Rel(client_wasm, v2_api, "HTTP Fetch")
Rel(interface, models, "uses")
Rel(interface, error, "throws")
Rel(client_native, interface, "implements")
Rel(client_wasm, interface, "implements")

@enduml
```

## C4 Component Diagram (V2 API)

```plantuml
@startuml C4_Component_V2
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

System_Boundary(v2_system, "osu! API v2 Module") {
    Component(oauth, "OAuth Interface", "OAuth/Session management", "Handles authentication and token management")
    Component(beatmaps, "Beatmaps Interface", "Beatmap queries", "Provides beatmap data endpoints")
    Component(beatmapsets, "Beatmapsets Interface", "Beatmapset queries", "Provides beatmapset data endpoints")
    Component(users, "Users Interface", "User queries", "Provides user profile endpoints")
    Component(scores, "Scores Interface", "Score queries", "Provides score data endpoints")
    Component(matches, "Matches Interface", "Match queries", "Provides multiplayer match data")
    Component(ranking, "Ranking Interface", "Ranking queries", "Provides ranking data")
    Component(search, "Search Interface", "Search functionality", "Provides search endpoints")
    Component(news, "News Interface", "News queries", "Provides news endpoints")
    Component(wiki, "Wiki Interface", "Wiki queries", "Provides wiki endpoints")
    Component(chat, "Chat Interface", "Chat functionality", "Provides chat endpoints")
    Component(friends, "Friends Interface", "Friend queries", "Provides friend list endpoints")
    
    Component(client, "V2 Client", "Request dispatcher", "Main client that routes requests")
}

Rel(client, oauth, "uses")
Rel(client, beatmaps, "uses")
Rel(client, beatmapsets, "uses")
Rel(client, users, "uses")
Rel(client, scores, "uses")
Rel(client, matches, "uses")
Rel(client, ranking, "uses")
Rel(client, search, "uses")
Rel(client, news, "uses")
Rel(client, wiki, "uses")
Rel(client, chat, "uses")
Rel(client, friends, "uses")

@enduml
```

## UML 类图 - 核心架构

```plantuml
@startuml UML_Core_Architecture

package "osynic_osuapi::v2" {
    interface IClient {
        +beatmaps: BeatmapInterface
        +beatmapsets: BeatmapsetInterface
        +users: UserInterface
        +scores: ScoreInterface
        +matches: MatchInterface
        +ranking: RankingInterface
        +oauth: OAuthInterface
        +search: SearchInterface
        +news: NewsInterface
        +wiki: WikiInterface
        +chat: ChatInterface
        +friends: FriendsInterface
    }
    
    class ClientNative {
        -http_client: reqwest::Client
        -token: String
        +new(token: String)
        +request()
        +authenticate()
    }
    
    class ClientWasm {
        -window: web_sys::Window
        -token: String
        +new(token: String)
        +request()
        +authenticate()
    }
}

package "osynic_osuapi::v2::interface" {
    interface IBeatmap {
        +lookup()
        +get_beatmap()
    }
    
    interface IUser {
        +get_user()
        +get_scores()
    }
    
    interface IScore {
        +get_score()
        +get_scores()
    }
    
    interface IRanking {
        +get_ranking()
    }
    
    interface IOAuth {
        +refresh_token()
        +revoke_token()
    }
}

package "osynic_osuapi::v2::model" {
    class Beatmap {
        -id: u32
        -title: String
        -artist: String
        -version: String
        -status: String
    }
    
    class User {
        -id: u32
        -username: String
        -country_code: String
        -statistics: UserStatistics
    }
    
    class Score {
        -id: u64
        -score: u32
        -user_id: u32
        -beatmap_id: u32
        -mods: Vec<String>
    }
    
    class Beatmapset {
        -id: u32
        -title: String
        -artist: String
        -beatmaps: Vec<Beatmap>
    }
}

package "osynic_osuapi::error" {
    enum ApiError {
        NotFound
        Unauthorized
        BadRequest
        RateLimited
        ServerError
    }
}

ClientNative --|> IClient
ClientWasm --|> IClient
IClient --> IBeatmap
IClient --> IUser
IClient --> IScore
IClient --> IRanking
IClient --> IOAuth

IBeatmap --> Beatmap
IUser --> User
IScore --> Score
IBeatmap --> Beatmapset

ClientNative ..> ApiError
ClientWasm ..> ApiError

@enduml
```

## UML 类图 - 数据模型

```plantuml
@startuml UML_DataModels

package "osynic_osuapi::v2::model" {
    class Beatmap {
        -id: u32
        -title: String
        -artist: String
        -creator: String
        -version: String
        -source: String
        -tags: String
        -status: BeatmapStatus
        -ranked_date: DateTime<Utc>
        -updated_at: DateTime<Utc>
        -last_updated: DateTime<Utc>
        +to_json()
        +from_json()
    }
    
    class Beatmapset {
        -id: u32
        -title: String
        -artist: String
        -creator: String
        -beatmaps: Vec<Beatmap>
        -status: BeatmapsetStatus
        -ranked_date: DateTime<Utc>
        +get_beatmap(version: String)
        +to_json()
    }
    
    class User {
        -id: u32
        -username: String
        -name_history: Vec<String>
        -country_code: String
        -is_online: bool
        -is_bot: bool
        -statistics: UserStatistics
        -scores: Vec<Score>
        +to_json()
        +from_json()
    }
    
    class UserStatistics {
        -rank: u32
        -pp: f32
        -level: UserLevel
        -accuracy: f32
        -play_count: u32
        -play_time: u32
        -total_score: u64
        -ranked_score: u64
        -count_300: u32
        -count_100: u32
        -count_50: u32
    }
    
    class Score {
        -id: u64
        -score: u32
        -user_id: u32
        -beatmap_id: u32
        -accuracy: f32
        -mods: Vec<String>
        -combo: u32
        -perfect: bool
        -created_at: DateTime<Utc>
        -pp: Option<f32>
    }
    
    class Mode {
        -id: i32
        -name: String
    }
    
    enum BeatmapStatus {
        GRAVEYARD
        WIP
        PENDING
        RANKED
        APPROVED
        QUALIFIED
        LOVED
    }
    
    enum BeatmapsetStatus {
        GRAVEYARD
        WIP
        PENDING
        RANKED
        APPROVED
        QUALIFIED
        LOVED
    }
}

Beatmapset "1" --> "*" Beatmap
User "1" --> "1" UserStatistics
User "1" --> "*" Score
Score "*" --> "1" Beatmap
Score "*" --> "1" Mode

@enduml
```

## UML Sequence Diagram - OAuth Flow

```plantuml
@startuml UML_OAuth_Flow

participant Developer
participant "Osynic Client" as Client
participant "osu! API v2" as API

Developer -> Client: new(client_id, client_secret)
activate Client

Client -> API: POST /oauth/token (client credentials)
activate API
API --> Client: access_token, refresh_token
deactivate API

Developer -> Client: request_user_data()
activate Client
Client -> API: GET /api/v2/me\n(with access_token)
activate API
API --> Client: user_data
deactivate API
Client --> Developer: user_data
deactivate Client

Developer -> Client: refresh_token()
activate Client
Client -> API: POST /oauth/token (refresh_token)
activate API
API --> Client: new_access_token
deactivate API
Client --> Developer: success
deactivate Client

@enduml
```

## UML Sequence Diagram - API Request Flow

```plantuml
@startuml UML_API_Request_Flow

participant Developer
participant "Osynic Client" as Client
participant "HTTP Client" as HTTP
participant "osu! API" as API

Developer -> Client: get_user(user_id)
activate Client

Client -> Client: validate_input()
activate Client
deactivate Client

Client -> Client: build_request()
activate Client
deactivate Client

Client -> HTTP: send_request()
activate HTTP

HTTP -> API: GET /api/v2/users/{user_id}
activate API
API --> HTTP: 200 OK + user_data
deactivate API

HTTP --> Client: Response
deactivate HTTP

Client -> Client: parse_response()
activate Client
Client -> Client: deserialize_json()
deactivate Client

Client -> Client: create_user_object()
activate Client
deactivate Client

Client --> Developer: User
deactivate Client

@enduml
```

## UML State Diagram - Client States

```plantuml
@startuml UML_ClientStates

state "Uninitialized" as Uninit
state "Authenticated" as Auth
state "Token Expired" as Expired
state "Refreshing Token" as Refreshing
state "Error" as Error

[*] --> Uninit

Uninit --> Auth: initialize(token)
Auth --> Expired: token_expires()
Auth --> Error: invalid_request()
Auth --> Auth: make_request()

Expired --> Refreshing: refresh_token()
Refreshing --> Auth: token_refreshed()
Refreshing --> Error: refresh_failed()

Error --> Uninit: reset()

@enduml
```

## UML Package Diagram - Module Structure

```plantuml
@startuml UML_PackageStructure

package "osynic_osuapi" {
    package "v1" {
        package "client" {
            package "gloo" {
                [WASM Client]
            }
            package "request" {
                [Request Builder]
            }
        }
        package "interface" {
            [Beatmap API]
            [User API]
            [Score API]
        }
        package "model" {
            [Beatmap Model]
            [User Model]
            [Score Model]
        }
    }
    
    package "v2" {
        package "client" {
            package "gloo" {
                [WASM Client]
            }
            package "request" {
                [Request Builder]
            }
        }
        package "interface" {
            [OAuth Interface]
            [Users Interface]
            [Beatmaps Interface]
            [Scores Interface]
            [Ranking Interface]
            [Search Interface]
            [Chat Interface]
        }
        package "model" {
            [User Model]
            [Beatmap Model]
            [Score Model]
            [Beatmapset Model]
            [Friend Model]
        }
    }
    
    package "core" {
        [Error Handling]
        [Utilities]
        [Prelude]
    }
    
    package "wasm" {
        [WASM Bindings]
        [TypeScript Exports]
    }
}

[v1] -.-> [core]
[v2] -.-> [core]
[wasm] -.-> [v1]
[wasm] -.-> [v2]

@enduml
```

## UML Component Diagram - Dependency Graph

```plantuml
@startuml UML_Dependencies

component "Osynic Osuapi" as OsynicCore {
    component "Error Module" as ErrorMod
    component "Utils Module" as UtilsMod
    component "V1 API" as V1
    component "V2 API" as V2
    component "WASM Bindings" as WasmBind
}

component "External Dependencies" as External {
    component "Serde" as Serde
    component "Chrono" as Chrono
    component "Tokio" as Tokio
    component "Reqwest" as Reqwest
    component "Gloo-net" as Gloo
    component "WASM-bindgen" as WasmBind_Ext
}

OsynicCore o-- External

V1 --> ErrorMod
V1 --> UtilsMod
V1 --> Serde
V1 --> Chrono

V2 --> ErrorMod
V2 --> UtilsMod
V2 --> Serde
V2 --> Chrono

WasmBind --> V1
WasmBind --> V2
WasmBind --> WasmBind_Ext
WasmBind --> Gloo

ErrorMod --> Serde

@enduml
```

## V1 API 实现的端点概览

```plantuml
@startuml V1_Endpoints_Overview

package "V1 API 端点 (7个方法)" {
    usecase UC1 as "👤 Users\nget_user()"
    usecase UC2 as "get_user_best()"
    usecase UC3 as "get_user_recent()"
    usecase UC4 as "🎵 Beatmaps\nget_beatmaps()"
    usecase UC5 as "⭐ Scores\nget_scores()"
    usecase UC6 as "👥 Multiplayer\nget_match()"
    usecase UC7 as "🎬 Replay\nget_replay()"
}

actor "应用程序" as App

App --> UC1
App --> UC2
App --> UC3
App --> UC4
App --> UC5
App --> UC6
App --> UC7

@enduml
```

## V2 API 实现的端点概览

```plantuml
@startuml V2_Endpoints_Overview

package "V2 API 端点 (31个方法)" {
    package "👤 Users (6)" {
        usecase UC1 as "get_own_data()"
        usecase UC2 as "get_user_kudosu()"
        usecase UC3 as "get_user_scores()"
        usecase UC4 as "get_user_beatmaps()"
        usecase UC5 as "get_user_beatmaps_most_played()"
        usecase UC6 as "get_user_recent_activity()"
    }
    
    package "🎵 Beatmaps (3)" {
        usecase UC7 as "lookup_beatmap()"
        usecase UC8 as "get_beatmap_attributes()"
        usecase UC9 as "get_beatmap_scores()"
    }
    
    package "📦 Beatmapsets (2)" {
        usecase UC10 as "get_beatmapset()"
        usecase UC11 as "get_beatmapset_discussion()"
    }
    
    package "⭐ Scores (2)" {
        usecase UC12 as "get_score()"
        usecase UC13 as "get_score_own()"
    }
    
    package "🏆 Ranking (2)" {
        usecase UC14 as "get_ranking()"
        usecase UC15 as "get_ranking_spotlights()"
    }
    
    package "🔍 Search (1)" {
        usecase UC16 as "search()"
    }
    
    package "👥 Matches (1)" {
        usecase UC17 as "get_match()"
    }
    
    package "💬 Chat (2)" {
        usecase UC18 as "get_messages()"
        usecase UC19 as "send_message()"
    }
    
    package "📰 News (2)" {
        usecase UC20 as "get_news_listing()"
        usecase UC21 as "get_news_post()"
    }
    
    package "📖 Wiki (1)" {
        usecase UC22 as "get_wiki_page()"
    }
    
    package "👫 Friends (1)" {
        usecase UC23 as "get_friends()"
    }
    
    package "🔑 OAuth (2)" {
        usecase UC24 as "refresh_token()"
        usecase UC25 as "revoke_token()"
    }
    
    package "🔄 Session (1)" {
        usecase UC26 as "heartbeat()"
    }
    
    package "📡 Events (1)" {
        usecase UC27 as "get_events()"
    }
}

actor "应用程序" as App

App --> UC1
App --> UC2
App --> UC3
App --> UC4
App --> UC5
App --> UC6
App --> UC7
App --> UC8
App --> UC9
App --> UC10
App --> UC11
App --> UC12
App --> UC13
App --> UC14
App --> UC15
App --> UC16
App --> UC17
App --> UC18
App --> UC19
App --> UC20
App --> UC21
App --> UC22
App --> UC23
App --> UC24
App --> UC25
App --> UC26
App --> UC27

@enduml
```

## API 端点实现矩阵

```plantuml
@startuml API_Implementation_Matrix

skinparam backgroundColor #FEFEFE

component "API 端点分布" {
    component "V1 API (7)" as v1_total
    component "V2 API (31)" as v2_total
    
    component "Users" as users_comp
    component "Beatmaps" as beatmaps_comp
    component "Scores" as scores_comp
    component "Others" as others_comp
}

v1_total --> users_comp : 3个
v1_total --> beatmaps_comp : 1个
v1_total --> scores_comp : 1个
v1_total --> others_comp : 2个

v2_total --> users_comp : 6个
v2_total --> beatmaps_comp : 3个
v2_total --> scores_comp : 2个
v2_total --> others_comp : 20个

@enduml
```

## API 端点实现对比图

```plantuml
@startuml API_Implementation_Compare

object "V1 API 统计" as v1 {
    模块数 = 5
    总方法 = 7
    "Users" = 3
    "Beatmaps" = 1
    "Scores" = 1
    "Multiplayer" = 1
    "Replay" = 1
}

object "V2 API 统计" as v2 {
    模块数 = 16
    总方法 = 31
    "Users" = 6
    "Beatmaps" = 3
    "Beatmapsets" = 2
    "Scores" = 2
    "Ranking" = 2
    "Search" = 1
    "Matches" = 1
    "Chat" = 2
    "News" = 2
    "Wiki" = 1
    "Friends" = 1
    "OAuth" = 2
    "Session" = 1
    "Events" = 1
}

object "性能对比" as compare {
    "模块数增长" = "+220%"
    "方法数增长" = "+343%"
    "功能完整度" = "V2更全面"
    "推荐使用" = "优先V2"
}

v1 .. compare
v2 .. compare

@enduml
```

## V1 vs V2 API 功能对比

```plantuml
@startuml V1_VS_V2_Comparison

left to right direction

package "API V1" {
    usecase "获取用户信息" as U1V1
    usecase "获取最佳成绩" as B1V1
    usecase "获取最近游玩" as R1V1
    usecase "查询谱面" as BM1V1
    usecase "获取排行榜" as SC1V1
}

package "API V2" {
    usecase "获取自己数据" as U1V2
    usecase "获取用户Kudosu" as K2V2
    usecase "获取多种成绩类型" as B2V2
    usecase "获取用户作者谱面" as UA2V2
    usecase "获取最常玩谱面" as UM2V2
    usecase "获取最近活动" as RA2V2
    usecase "查询谱面+属性" as BM2V2
    usecase "获取谱面集" as BMS2V2
    usecase "获取谱面讨论" as BMD2V2
    usecase "综合搜索" as S2V2
    usecase "多人游戏" as M2V2
    usecase "聊天功能" as C2V2
    usecase "新闻系统" as N2V2
    usecase "Wiki" as W2V2
    usecase "好友列表" as F2V2
    usecase "OAuth认证" as O2V2
}

@enduml
```

## V1和V2 API 模块关系

```plantuml
@startuml V1_V2_Module_Relations

card "V1 API模块\n(5个主要模块)" {
    frame "Users" {
        [get_user]
        [get_user_best]
        [get_user_recent]
    }
    frame "Beatmaps" {
        [get_beatmaps]
    }
    frame "Scores" {
        [get_scores]
    }
    frame "Multiplayer" {
        [get_match]
    }
    frame "Replay" {
        [get_replay]
    }
}

card "V2 API模块\n(16个主要模块)" {
    frame "Users" {
        [get_own_data]
        [get_user_kudosu]
        [get_user_scores]
        [get_user_beatmaps]
        [get_user_beatmaps_most_played]
        [get_user_recent_activity]
    }
    frame "Beatmaps" {
        [lookup_beatmap]
        [get_beatmap_attributes]
        [get_beatmap_scores]
    }
    frame "Beatmapsets" {
        [get_beatmapset]
        [get_beatmapset_discussion]
    }
    frame "Scores" {
        [get_score]
        [get_score_own]
    }
    frame "Ranking" {
        [get_ranking]
        [get_ranking_spotlights]
    }
    frame "Search" {
        [search]
    }
    frame "Matches" {
        [get_match]
    }
    frame "Chat" {
        [get_messages]
        [send_message]
    }
    frame "News" {
        [get_news_listing]
        [get_news_post]
    }
    frame "Wiki" {
        [get_wiki_page]
    }
    frame "Friends" {
        [get_friends]
    }
    frame "OAuth" {
        [refresh_token]
        [revoke_token]
    }
    frame "Session" {
        [heartbeat]
    }
    frame "Events" {
        [get_events]
    }
}

@enduml
```

---

## 架构说明

---

## 架构说明

### 系统概述
- **Osynic Osuapi** 是一个Rust库，为osu! API（V1和V2）提供类型安全的绑定
- 支持Native Rust环境（通过Tokio + Reqwest）和WebAssembly环境（通过Gloo）
- 设计时考虑了可扩展性，通过基于特征的接口实现

### 关键组件

1. **V1 API模块**：支持遗留的osu! API v1
2. **V2 API模块**：支持现代的osu! API v2
3. **客户端层**：抽象HTTP通信（Native vs WASM）
4. **接口层**：API端点的特征定义
5. **模型层**：强类型的数据结构
6. **错误处理**：API特定的自定义错误类型
7. **WASM绑定**：Web环境的TypeScript/JavaScript集成

### 设计模式

- **基于特征的设计**：接口定义为特征以获得最大灵活性
- **构建者模式**：请求构建遵循构建者模式
- **错误处理**：自定义`ApiError`枚举用于结构化错误处理
- **特征标志**：通过Cargo特征进行模块化编译
- **依赖注入**：客户端在初始化时接受配置

### 支持的端点统计

#### V1 API 统计
- **总模块数**：5个
- **总方法数**：7个
  - Users: 3个方法
  - Beatmaps: 1个方法
  - Scores: 1个方法
  - Multiplayer: 1个方法
  - Replay: 1个方法

#### V2 API 统计
- **总模块数**：16个
- **总方法数**：31个
  - Users: 6个方法（用户数据获取和管理）
  - Beatmaps: 3个方法（谱面信息和查询）
  - Beatmapsets: 2个方法（谱面集操作）
  - Scores: 2个方法（成绩数据）
  - Ranking: 2个方法（排行榜）
  - Search: 1个方法（综合搜索）
  - Matches: 1个方法（多人游戏匹配）
  - Chat: 2个方法（聊天功能）
  - News: 2个方法（新闻系统）
  - Wiki: 1个方法（Wiki页面）
  - Friends: 1个方法（好友列表）
  - OAuth: 2个方法（认证）
  - Session: 1个方法（会话管理）
  - Events: 1个方法（事件流）
  - Comments: 1个方法（评论）
  - Forum: 1个方法（论坛）
  - Changelog: 1个方法（更新日志）
  - Notifications: 1个方法（通知）

### V1 vs V2 对比

| 功能     | V1     | V2                           |
| -------- | ------ | ---------------------------- |
| 用户数据 | ✓ 基础 | ✓ 详细（含Kudosu、活动）     |
| 谱面查询 | ✓ 简单 | ✓ 详细（含属性、讨论）       |
| 成绩排行 | ✓ 基础 | ✓ 多种类型（第一/最近/最佳） |
| 认证     | ✗      | ✓ OAuth 2.0                  |
| 聊天     | ✗      | ✓                            |
| 搜索     | ✗      | ✓ 综合搜索                   |
| 新闻     | ✗      | ✓                            |
| Wiki     | ✗      | ✓                            |
| 好友列表 | ✗      | ✓                            |
| 多人游戏 | ✓ 基础 | ✓ 详细                       |

### 实现进度

✅ **已实现**：所有V1和V2的核心端点都已实现和测试
📊 **功能完整度**：V2 API覆盖了官方文档的所有主要功能
🚀 **性能优化**：使用Tokio和Reqwest实现高性能网络请求
🔒 **类型安全**：利用Rust的类型系统提供编译时安全检查

