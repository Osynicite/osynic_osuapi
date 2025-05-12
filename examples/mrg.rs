// Get multiplayer page
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::multiplayer::IMultiplayer;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let rooms = client
        .multiplayer
        .get_multiplayer_rooms(Some(3),None,None,None,None)
        .await?;
    println!("{:?}", rooms);
    Ok(())
}

/*
ReqwestMultiplayer get_multiplayer_rooms
[Room {
	id: 1354371,
	name: "Practice! Discord: Volume 8 - Tech",
	category: "normal",
	status: "idle",
	room_type: "playlists",
	user_id: 34156354,
	starts_at: "2025-05-12T04:01:13+00:00",
	ends_at: "2025-05-26T04:01:13+00:00",
	max_attempts: None,
	participant_count: 2,
	channel_id: 58937675,
	active: true,
	has_password: false,
	queue_mode: "host_only",
	auto_skip: false,
	current_playlist_item: Playlist {
		id: 16011916,
		room_id: 1354371,
		beatmap_id: 4469966,
		ruleset_id: 0,
		allowed_mods: [PlaylistMod {
			acronym: "EZ",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HT",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "PF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DT",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "FL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "ST",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "MR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SG",
			settings: Object {}
		}, PlaylistMod {
			acronym: "RX",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SO",
			settings: Object {}
		}, PlaylistMod {
			acronym: "TR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "WG",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SI",
			settings: Object {}
		}, PlaylistMod {
			acronym: "GR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "TC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "MU",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NS",
			settings: Object {}
		}, PlaylistMod {
			acronym: "RP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "FR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BU",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SY",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BM",
			settings: Object {}
		}],
		required_mods: [],
		freestyle: false,
		expired: false,
		owner_id: 34156354,
		playlist_order: None,
		played_at: None,
		beatmap: Some(Beatmap {
			beatmapset_id: 2051792,
			difficulty_rating: 5.18,
			id: 4469966,
			mode: "osu",
			status: "ranked",
			total_length: 174,
			user_id: 12551840,
			version: "0pp's ANOTHER",
			beatmapset: BeatmapSet {
				artist: "N2",
				artist_unicode: "N²",
				covers: Covers {
					cover: "https://assets.ppy.sh/beatmaps/2051792/covers/cover.jpg?1707958437",
					cover_2x: "https://assets.ppy.sh/beatmaps/2051792/covers/cover@2x.jpg?1707958437",
					card: "https://assets.ppy.sh/beatmaps/2051792/covers/card.jpg?1707958437",
					card_2x: "https://assets.ppy.sh/beatmaps/2051792/covers/card@2x.jpg?1707958437",
					list: "https://assets.ppy.sh/beatmaps/2051792/covers/list.jpg?1707958437",
					list_2x: "https://assets.ppy.sh/beatmaps/2051792/covers/list@2x.jpg?1707958437",
					slimcover: "https://assets.ppy.sh/beatmaps/2051792/covers/slimcover.jpg?1707958437",
					slimcover_2x: "https://assets.ppy.sh/beatmaps/2051792/covers/slimcover@2x.jpg?1707958437"
				},
				creator: "pocket-",
				favourite_count: 263,
				hype: None,
				id: 2051792,
				nsfw: false,
				offset: 0,
				play_count: 117136,
				preview_url: "//b.ppy.sh/preview/2051792.mp3",
				source: "Arcaea",
				spotlight: false,
				status: "ranked",
				title: "NULL APOPHENIA",
				title_unicode: "NULL APOPHENIA",
				track_id: None,
				user_id: 6808091,
				video: false
			}
		})
	},
	difficulty_range: DifficultyRange {
		max: 7.86,
		min: 5.18
	},
	host: Host {
		avatar_url: "https://a.ppy.sh/34156354?1737423744.jpeg",
		country_code: "US",
		default_group: "default",
		id: 34156354,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: true,
		last_visit: Some("2025-05-12T06:43:37+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "headpats2you",
		country: Some(Country {
			code: "US",
			name: "United States"
		})
	},
	playlist_item_stats: PlaylistItemStats {
		count_active: 12,
		count_total: 12,
		ruleset_ids: [0]
	},
	recent_participants: [RecentParticipant {
		avatar_url: "https://a.ppy.sh/37520771?1742408059.jpeg",
		country_code: "FI",
		default_group: "default",
		id: 37520771,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:41:56+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Robeliuss"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/3980260?1737844081.jpeg",
		country_code: "US",
		default_group: "default",
		id: 3980260,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: true,
		last_visit: Some("2025-05-12T05:16:15+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "twee"
	}]
}, Room {
	id: 1354141,
	name: "Daily Challenge: May 12, 2025",
	category: "daily_challenge",
	status: "idle",
	room_type: "playlists",
	user_id: 3,
	starts_at: "2025-05-12T00:05:00+00:00",
	ends_at: "2025-05-13T00:00:00+00:00",
	max_attempts: None,
	participant_count: 707,
	channel_id: 58936768,
	active: true,
	has_password: false,
	queue_mode: "host_only",
	auto_skip: false,
	current_playlist_item: Playlist {
		id: 16008432,
		room_id: 1354141,
		beatmap_id: 1383338,
		ruleset_id: 0,
		allowed_mods: [PlaylistMod {
			acronym: "EZ",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HT",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "PF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DT",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "HD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "FL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "ST",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "TP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DA",
			settings: Object {}
		}, PlaylistMod {
			acronym: "CL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "MR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AL",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SG",
			settings: Object {}
		}, PlaylistMod {
			acronym: "RX",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SO",
			settings: Object {}
		}, PlaylistMod {
			acronym: "TR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "WG",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SI",
			settings: Object {}
		}, PlaylistMod {
			acronym: "GR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DF",
			settings: Object {}
		}, PlaylistMod {
			acronym: "WU",
			settings: Object {}
		}, PlaylistMod {
			acronym: "WD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "TC",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AD",
			settings: Object {}
		}, PlaylistMod {
			acronym: "MU",
			settings: Object {}
		}, PlaylistMod {
			acronym: "NS",
			settings: Object {}
		}, PlaylistMod {
			acronym: "MG",
			settings: Object {}
		}, PlaylistMod {
			acronym: "RP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "AS",
			settings: Object {}
		}, PlaylistMod {
			acronym: "FR",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BU",
			settings: Object {}
		}, PlaylistMod {
			acronym: "SY",
			settings: Object {}
		}, PlaylistMod {
			acronym: "DP",
			settings: Object {}
		}, PlaylistMod {
			acronym: "BM",
			settings: Object {}
		}],
		required_mods: [],
		freestyle: false,
		expired: false,
		owner_id: 3,
		playlist_order: None,
		played_at: None,
		beatmap: Some(Beatmap {
			beatmapset_id: 652397,
			difficulty_rating: 5.07,
			id: 1383338,
			mode: "osu",
			status: "ranked",
			total_length: 354,
			user_id: 8688812,
			version: "Nightfall",
			beatmapset: BeatmapSet {
				artist: "Rusty K",
				artist_unicode: "Rusty K",
				covers: Covers {
					cover: "https://assets.ppy.sh/beatmaps/652397/covers/cover.jpg?1631510069",
					cover_2x: "https://assets.ppy.sh/beatmaps/652397/covers/cover@2x.jpg?1631510069",
					card: "https://assets.ppy.sh/beatmaps/652397/covers/card.jpg?1631510069",
					card_2x: "https://assets.ppy.sh/beatmaps/652397/covers/card@2x.jpg?1631510069",
					list: "https://assets.ppy.sh/beatmaps/652397/covers/list.jpg?1631510069",
					list_2x: "https://assets.ppy.sh/beatmaps/652397/covers/list@2x.jpg?1631510069",
					slimcover: "https://assets.ppy.sh/beatmaps/652397/covers/slimcover.jpg?1631510069",
					slimcover_2x: "https://assets.ppy.sh/beatmaps/652397/covers/slimcover@2x.jpg?1631510069"
				},
				creator: "Mir",
				favourite_count: 448,
				hype: None,
				id: 652397,
				nsfw: false,
				offset: 0,
				play_count: 348597,
				preview_url: "//b.ppy.sh/preview/652397.mp3",
				source: "",
				spotlight: false,
				status: "ranked",
				title: "Dark Eyes",
				title_unicode: "Dark Eyes",
				track_id: Some(2238),
				user_id: 8688812,
				video: false
			}
		})
	},
	difficulty_range: DifficultyRange {
		max: 5.07,
		min: 5.07
	},
	host: Host {
		avatar_url: "https://a.ppy.sh/3?1528948612.png",
		country_code: "SH",
		default_group: "bot",
		id: 3,
		is_active: false,
		is_bot: true,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: Some("2025-02-12T08:37:45+00:00"),
		pm_friends_only: false,
		profile_colour: Some("#e45678"),
		username: "BanchoBot",
		country: Some(Country {
			code: "SH",
			name: "Saint Helena"
		})
	},
	playlist_item_stats: PlaylistItemStats {
		count_active: 1,
		count_total: 1,
		ruleset_ids: [0]
	},
	recent_participants: [RecentParticipant {
		avatar_url: "https://a.ppy.sh/31685311?1745088794.png",
		country_code: "TH",
		default_group: "default",
		id: 31685311,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: None,
		pm_friends_only: false,
		profile_colour: None,
		username: "WERO8001"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/11432670?1703469720.jpeg",
		country_code: "US",
		default_group: "default",
		id: 11432670,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:41:48+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Puding"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/34490131?1746083184.jpeg",
		country_code: "PH",
		default_group: "default",
		id: 34490131,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:42:11+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "AngryRabbit11"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/6417190?1705699625.jpeg",
		country_code: "US",
		default_group: "default",
		id: 6417190,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:42:04+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Brames"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/5770189?1746586246.jpeg",
		country_code: "FI",
		default_group: "default",
		id: 5770189,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:40:45+00:00"),
		pm_friends_only: true,
		profile_colour: None,
		username: "Usadereh"
	}, RecentParticipant {
		avatar_url: "https://osu.ppy.sh/images/layout/avatar-guest@2x.png",
		country_code: "BR",
		default_group: "default",
		id: 4223762,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:42:17+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "BalaBalance"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/27919272?1712860164.jpeg",
		country_code: "DE",
		default_group: "default",
		id: 27919272,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: true,
		last_visit: Some("2025-05-12T06:39:18+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Sirkey"
	}, RecentParticipant {
		avatar_url: "https://osu.ppy.sh/images/layout/avatar-guest@2x.png",
		country_code: "RU",
		default_group: "default",
		id: 37720076,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:38:43+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "iluhins"
	}, RecentParticipant {
		avatar_url: "https://osu.ppy.sh/images/layout/avatar-guest@2x.png",
		country_code: "US",
		default_group: "default",
		id: 14416187,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T06:42:36+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "gacha"
	}, RecentParticipant {
		avatar_url: "https://a.ppy.sh/15441061?1614297404.jpeg",
		country_code: "VN",
		default_group: "default",
		id: 15441061,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: None,
		pm_friends_only: false,
		profile_colour: None,
		username: "TomFG"
	}]
}, Room {
	id: 1353852,
	name: "I LOVE BPM>=200",
	category: "normal",
	status: "idle",
	room_type: "playlists",
	user_id: 29259837,
	starts_at: "2025-05-11T20:41:19+00:00",
	ends_at: "2025-05-25T20:41:19+00:00",
	max_attempts: None,
	participant_count: 1,
	channel_id: 58935538,
	active: true,
	has_password: false,
	queue_mode: "host_only",
	auto_skip: false,
	current_playlist_item: Playlist {
		id: 16003783,
		room_id: 1353852,
		beatmap_id: 4115911,
		ruleset_id: 1,
		allowed_mods: [],
		required_mods: [],
		freestyle: true,
		expired: false,
		owner_id: 29259837,
		playlist_order: None,
		played_at: None,
		beatmap: Some(Beatmap {
			beatmapset_id: 1981014,
			difficulty_rating: 4.31,
			id: 4115911,
			mode: "taiko",
			status: "ranked",
			total_length: 115,
			user_id: 3196091,
			version: "Collab Oni",
			beatmapset: BeatmapSet {
				artist: "Amuro vs. Killer",
				artist_unicode: "Amuro vs. Killer",
				covers: Covers {
					cover: "https://assets.ppy.sh/beatmaps/1981014/covers/cover.jpg?1684867176",
					cover_2x: "https://assets.ppy.sh/beatmaps/1981014/covers/cover@2x.jpg?1684867176",
					card: "https://assets.ppy.sh/beatmaps/1981014/covers/card.jpg?1684867176",
					card_2x: "https://assets.ppy.sh/beatmaps/1981014/covers/card@2x.jpg?1684867176",
					list: "https://assets.ppy.sh/beatmaps/1981014/covers/list.jpg?1684867176",
					list_2x: "https://assets.ppy.sh/beatmaps/1981014/covers/list@2x.jpg?1684867176",
					slimcover: "https://assets.ppy.sh/beatmaps/1981014/covers/slimcover.jpg?1684867176",
					slimcover_2x: "https://assets.ppy.sh/beatmaps/1981014/covers/slimcover@2x.jpg?1684867176"
				},
				creator: "radar",
				favourite_count: 17,
				hype: None,
				id: 1981014,
				nsfw: false,
				offset: 0,
				play_count: 8496,
				preview_url: "//b.ppy.sh/preview/1981014.mp3",
				source: "beatmania IIDX 12 HAPPY SKY",
				spotlight: false,
				status: "ranked",
				title: "Mei",
				title_unicode: "冥",
				track_id: None,
				user_id: 7131099,
				video: false
			}
		})
	},
	difficulty_range: DifficultyRange {
		max: 6.78,
		min: 4.31
	},
	host: Host {
		avatar_url: "https://a.ppy.sh/29259837?1719074414.jpeg",
		country_code: "GB",
		default_group: "default",
		id: 29259837,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: Some("2025-05-11T22:45:46+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "DiamondN1nja",
		country: Some(Country {
			code: "GB",
			name: "United Kingdom"
		})
	},
	playlist_item_stats: PlaylistItemStats {
		count_active: 16,
		count_total: 16,
		ruleset_ids: [1]
	},
	recent_participants: [RecentParticipant {
		avatar_url: "https://a.ppy.sh/29259837?1719074414.jpeg",
		country_code: "GB",
		default_group: "default",
		id: 29259837,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: Some("2025-05-11T22:45:46+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "DiamondN1nja"
	}]
}]
*/