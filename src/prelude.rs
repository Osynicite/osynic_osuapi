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
    beatmap::IBeatmap, multiplayer::IMultiplayer as IMultiplayerV1, replay::IReplay,
    scores::IScores as IScoresV1, user::IUser,
};

#[cfg(feature = "v2")]
pub use crate::v2::interface::{
    beatmaps::IBeatmaps, beatmapsets::IBeatmapsets, changelog::IChangelog, chat::IChat,
    comments::IComments, events::IEvents, forum::IForum, matches::IMatches,
    multiplayer::IMultiplayer, news::INews, notifications::INotifications, oauth::IOauth,
    ranking::IRanking, scores::IScores, search::ISearch, users::IUsers, wiki::IWiki,
};
