// Get scores page
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::scores::IScores;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
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
    let scores = client
        .scores
        .get_scores(Some(Mode::Catch),None)
        .await?;
    // Print the first 5 scores
    println!("{:?}", scores.scores.iter().take(5).collect::<Vec<_>>());
    Ok(())
}

/*
ReqwestScores get_beatmapset
	[NonLegacyScore {
		classic_total_score: 6356275,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			miss: None,
			great: Some(807),
			large_tick_hit: Some(1),
			small_tick_hit: Some(527),
			small_tick_miss: None
		},
		mods: [AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			miss: Some(12),
			great: Some(795),
			large_tick_hit: Some(1),
			small_tick_hit: Some(512),
			small_tick_miss: Some(15)
		},
		beatmap_id: 4719779,
		best_id: None,
		id: 4827548560,
		rank: "A",
		ranked_type: "solo_score",
		user_id: 26981027,
		accuracy: 0.979775,
		build_id: None,
		ended_at: "2025-05-11T14:08:48Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: Some(220758981),
		legacy_total_score: 2466356,
		max_combo: 192,
		passed: true,
		pp: Some(8.5538),
		ruleset_id: 2,
		started_at: None,
		total_score: 668351,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: None
	}, NonLegacyScore {
		classic_total_score: 5229322,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			miss: None,
			great: Some(385),
			large_tick_hit: Some(53),
			small_tick_hit: Some(512),
			small_tick_miss: None
		},
		mods: [AcronymMod {
			acronym: "DT"
		}, AcronymMod {
			acronym: "PF"
		}, AcronymMod {
			acronym: "HR"
		}, AcronymMod {
			acronym: "HD"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			miss: None,
			great: Some(385),
			large_tick_hit: Some(53),
			small_tick_hit: Some(512),
			small_tick_miss: None
		},
		beatmap_id: 793353,
		best_id: None,
		id: 4827548561,
		rank: "XH",
		ranked_type: "solo_score",
		user_id: 2400918,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2025-05-11T14:08:49Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(220758982),
		legacy_total_score: 3905505,
		max_combo: 438,
		passed: true,
		pp: Some(77.6672),
		ruleset_id: 2,
		started_at: None,
		total_score: 1261914,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: None
	}, NonLegacyScore {
		classic_total_score: 6884287,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			miss: None,
			great: Some(491),
			large_tick_hit: Some(3),
			small_tick_hit: Some(300),
			small_tick_miss: None
		},
		mods: [AcronymMod {
			acronym: "HR"
		}, AcronymMod {
			acronym: "HD"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			miss: None,
			great: Some(491),
			large_tick_hit: Some(3),
			small_tick_hit: Some(300),
			small_tick_miss: None
		},
		beatmap_id: 5044971,
		best_id: None,
		id: 4827548769,
		rank: "XH",
		ranked_type: "solo_score",
		user_id: 4568537,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2025-05-11T14:08:51Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(220758983),
		legacy_total_score: 7049145,
		max_combo: 494,
		passed: true,
		pp: Some(73.3543),
		ruleset_id: 2,
		started_at: None,
		total_score: 1139712,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: None
	}, NonLegacyScore {
		classic_total_score: 5487330,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			miss: None,
			great: Some(599),
			large_tick_hit: Some(3),
			small_tick_hit: Some(227),
			small_tick_miss: None
		},
		mods: [],
		statistics: Statistics {
			miss: Some(6),
			great: Some(593),
			large_tick_hit: Some(3),
			small_tick_hit: Some(226),
			small_tick_miss: Some(1)
		},
		beatmap_id: 1319769,
		best_id: None,
		id: 4827550803,
		rank: "S",
		ranked_type: "solo_score",
		user_id: 32702900,
		accuracy: 0.991556,
		build_id: Some(7980),
		ended_at: "2025-05-11T14:09:15Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: None,
		legacy_total_score: 0,
		max_combo: 216,
		passed: true,
		pp: Some(24.0892),
		ruleset_id: 2,
		started_at: Some("2025-05-11T14:07:26Z"),
		total_score: 834637,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: None
	}, NonLegacyScore {
		classic_total_score: 7482058,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			miss: None,
			great: Some(671),
			large_tick_hit: None,
			small_tick_hit: Some(199),
			small_tick_miss: None
		},
		mods: [AcronymMod {
			acronym: "HD"
		}],
		statistics: Statistics {
			miss: Some(8),
			great: Some(663),
			large_tick_hit: None,
			small_tick_hit: Some(197),
			small_tick_miss: Some(2)
		},
		beatmap_id: 1319770,
		best_id: None,
		id: 4827550805,
		rank: "SH",
		ranked_type: "solo_score",
		user_id: 28173843,
		accuracy: 0.988506,
		build_id: Some(7980),
		ended_at: "2025-05-11T14:09:15Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: None,
		legacy_total_score: 0,
		max_combo: 257,
		passed: true,
		pp: Some(51.1163),
		ruleset_id: 2,
		started_at: Some("2025-05-11T14:07:26Z"),
		total_score: 871597,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: None
	}]
*/