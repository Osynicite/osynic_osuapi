	// "id": 1354457,
	// "name": "4 star acc training",
	// "category": "normal",
	// "status": "idle",
	// "type": "playlists",
	// "user_id": 15403214,
	// "starts_at": "2025-05-12T06:00:05+00:00",
	// "ends_at": "2025-05-12T06:30:05+00:00",
	// "max_attempts": null,
	// "participant_count": 1,
	// "channel_id": 58938270,
	// "active": true,
	// "has_password": false,
	// "queue_mode": "host_only",
	// "auto_skip": false,
	// "current_playlist_item"

use serde::{Deserialize, Serialize};

use crate::v2::model::room::structs::playlist::Playlist;
use crate::v2::model::room::structs::difficulty_range::DifficultyRange;
use crate::v2::model::room::structs::host::Host;
use crate::v2::model::room::structs::playlist::PlaylistItemStats;
use crate::v2::model::room::structs::recent_participant::RecentParticipant;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub status: String,
    #[serde(rename = "type")]
    pub room_type: String,
    pub user_id: u32,
    pub starts_at: String,
    pub ends_at: String,
    pub max_attempts: Option<u32>,
    pub participant_count: u32,
    pub channel_id: u32,
    pub active: bool,
    pub has_password: bool,
    pub queue_mode: String,
    pub auto_skip: bool,
    pub current_playlist_item: Playlist,
    pub difficulty_range: DifficultyRange,
    pub host: Host,
    pub playlist_item_stats: PlaylistItemStats,
    pub recent_participants: Vec<RecentParticipant>,
}