#[cfg(all(feature = "v1", feature = "not-wasm"))]
pub use crate::v1::client::request::client::OsynicOsuApiV1Client;

#[cfg(all(feature = "v2", feature = "not-wasm"))]
pub use crate::v2::client::request::client::OsynicOsuApiV2Client;

#[cfg(all(feature = "v1", feature = "wasm"))]
pub use crate::v1::client::gloo::client::OsynicOsuApiV1GlooClient;

#[cfg(all(feature = "v2", feature = "wasm"))]
pub use crate::v2::client::gloo::client::OsynicOsuApiV2GlooClient;

#[cfg(feature = "v1")]
pub use crate::v1::interface::{
    beatmap::IBeatmap,
    user::IUser,
    replay::IReplay,
    scores::IScores as IScoresV1,
    multiplayer::IMultiplayer as IMultiplayerV1,
};

#[cfg(feature = "v2")]
pub use crate::v2::interface::{
    oauth::IOauth,
    beatmapsets::IBeatmapsets,
    beatmaps::IBeatmaps,
    changelog::IChangelog,
    chat::IChat,
    comments::IComments,
    events::IEvents,
    forum::IForum,
    search::ISearch,
    matches::IMatches,
    multiplayer::IMultiplayer,
    news::INews,
    notifications::INotifications,
    ranking::IRanking,
    scores::IScores,
    users::IUsers,
    wiki::IWiki,
};
