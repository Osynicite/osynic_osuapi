use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::beatmaps::IBeatmaps;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env
        ::var("ACCESS_TOKEN")
        .expect(
            "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token"
        );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let solo_scores = client.beatmaps.get_solo_scores(4801662,Some(Mode::Catch),None,None).await?;
    println!("{:?}", solo_scores);
    
    
    Ok(())
}

/*
ReqwestBeatmaps get_Solo_Scores
NonLegacyScores {
	scores: [NonLegacyScore {
		classic_total_score: 17399431,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "HD"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3730861767,
		rank: "XH",
		ranked_type: "solo_score",
		user_id: 869471,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2024-10-23T17:24:57Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(218650806),
		legacy_total_score: 20727177,
		max_combo: 911,
		passed: true,
		pp: 96.3353,
		ruleset_id: 2,
		started_at: None,
		total_score: 1017600,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/869471?1654883742.jpeg",
			country_code: "ES",
			default_group: "default",
			id: 869471,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-03-07T00:45:26+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "DPJ9fuegos",
			country: Country {
				code: "ES",
				name: "Spain"
			},
			cover: Cover {
				custom_url: None,
				url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg",
				id: Some("8")
			},
			team: None
		})
	}, NonLegacyScore {
		classic_total_score: 17399431,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "HD"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3733445798,
		rank: "XH",
		ranked_type: "solo_score",
		user_id: 1375955,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2024-10-24T05:42:04Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(218655611),
		legacy_total_score: 20727177,
		max_combo: 911,
		passed: true,
		pp: 96.3353,
		ruleset_id: 2,
		started_at: None,
		total_score: 1017600,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/1375955?1728883293.png",
			country_code: "US",
			default_group: "default",
			id: 1375955,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: true,
			is_supporter: true,
			last_visit: "2025-03-07T05:56:14+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Zak",
			country: Country {
				code: "US",
				name: "United States"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg"),
				url: "https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg",
				id: None
			},
			team: None
		})
	}, NonLegacyScore {
		classic_total_score: 15490866,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "PF"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3731821455,
		rank: "X",
		ranked_type: "solo_score",
		user_id: 14477534,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2024-10-23T20:40:27Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(218652437),
		legacy_total_score: 19569782,
		max_combo: 911,
		passed: true,
		pp: 80.2794,
		ruleset_id: 2,
		started_at: None,
		total_score: 960000,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/14477534?1741043325.jpeg",
			country_code: "RS",
			default_group: "default",
			id: 14477534,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: true,
			last_visit: "2025-03-07T00:51:29+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "optm",
			country: Country {
				code: "RS",
				name: "Serbia"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/14477534/ce2cb985b4db886dfdf2352859d5ac5c2b12ad272ee32f32c2ed5fb001daf24d.gif"),
				url: "https://assets.ppy.sh/user-profile-covers/14477534/ce2cb985b4db886dfdf2352859d5ac5c2b12ad272ee32f32c2ed5fb001daf24d.gif",
				id: None
			},
			team: Some(Team {
				flag_url: "https://assets.ppy.sh/teams/flag/649/108cd4439bc2a5206a0c3e529e147d2897bb41afbd904b46ac358410b20a0b73.gif",
				id: 649,
				name: "squadra scimmie",
				short_name: "79"
			})
		})
	}, NonLegacyScore {
		classic_total_score: 15490866,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3899319873,
		rank: "X",
		ranked_type: "solo_score",
		user_id: 4577865,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2024-11-22T08:20:40Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(218984888),
		legacy_total_score: 19569782,
		max_combo: 911,
		passed: true,
		pp: 80.2794,
		ruleset_id: 2,
		started_at: None,
		total_score: 960000,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/4577865?1738113771.gif",
			country_code: "PH",
			default_group: "default",
			id: 4577865,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-03-07T04:57:47+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Chloebe",
			country: Country {
				code: "PH",
				name: "Philippines"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg"),
				url: "https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg",
				id: None
			},
			team: Some(Team {
				flag_url: "https://assets.ppy.sh/teams/flag/7/b12f51d25f89767fc05bbd503a26e7211a460fa2db01282c1c2999b611de4928.png",
				id: 7,
				name: "osu! Philippines",
				short_name: "OPH"
			})
		})
	}, NonLegacyScore {
		classic_total_score: 15490866,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "PF"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 4348085377,
		rank: "X",
		ranked_type: "solo_score",
		user_id: 2400918,
		accuracy: 1.0,
		build_id: None,
		ended_at: "2025-02-13T15:53:44Z",
		has_replay: true,
		is_perfect_combo: true,
		legacy_perfect: true,
		legacy_score_id: Some(219844868),
		legacy_total_score: 19569782,
		max_combo: 911,
		passed: true,
		pp: 80.2794,
		ruleset_id: 2,
		started_at: None,
		total_score: 960000,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/2400918?1730679337.jpeg",
			country_code: "JP",
			default_group: "default",
			id: 2400918,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: true,
			last_visit: "2025-03-06T22:20:03+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "syu",
			country: Country {
				code: "JP",
				name: "Japan"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg"),
				url: "https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg",
				id: None
			},
			team: Some(Team {
				flag_url: "https://assets.ppy.sh/teams/flag/5956/50b612dabacc7f98704579782ef9f9dc35a092da0f34026f64fcd8a4f3be074a.png",
				id: 5956,
				name: "osu!catch Convert Enjoyer",
				short_name: "CONV"
			})
		})
	}, NonLegacyScore {
		classic_total_score: 13911675,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "HD"
		}, AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 876,
			large_tick_hit: 32,
			small_tick_hit: 595
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3830675760,
		rank: "SH",
		ranked_type: "solo_score",
		user_id: 13382488,
		accuracy: 0.996684,
		build_id: None,
		ended_at: "2024-11-09T19:43:34Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: Some(218839926),
		legacy_total_score: 8807008,
		max_combo: 480,
		passed: true,
		pp: 51.7066,
		ruleset_id: 2,
		started_at: None,
		total_score: 909596,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/13382488?1741110998.jpeg",
			country_code: "US",
			default_group: "default",
			id: 13382488,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-03-07T04:45:23+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Rickster_rick",
			country: Country {
				code: "US",
				name: "United States"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/13382488/4c726721bb2c7621c00b43dbaa7d6e4d88342ed589d306b2a60864b9e23964db.jpeg"),
				url: "https://assets.ppy.sh/user-profile-covers/13382488/4c726721bb2c7621c00b43dbaa7d6e4d88342ed589d306b2a60864b9e23964db.jpeg",
				id: None
			},
			team: None
		})
	}, NonLegacyScore {
		classic_total_score: 11765652,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 875,
			large_tick_hit: 32,
			small_tick_hit: 595
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3860600918,
		rank: "S",
		ranked_type: "solo_score",
		user_id: 10932328,
		accuracy: 0.996021,
		build_id: None,
		ended_at: "2024-11-15T05:59:43Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: Some(218904272),
		legacy_total_score: 7422306,
		max_combo: 480,
		passed: true,
		pp: 41.6434,
		ruleset_id: 2,
		started_at: None,
		total_score: 836262,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/10932328?1707817072.jpeg",
			country_code: "CN",
			default_group: "default",
			id: 10932328,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: true,
			last_visit: "2025-03-07T04:17:39+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "kcwt",
			country: Country {
				code: "CN",
				name: "China"
			},
			cover: Cover {
				custom_url: None,
				url: "https://assets.ppy.sh/user-cover-presets/1/df28696b58541a9e67f6755918951d542d93bdf1da41720fcca2fd2c1ea8cf51.jpeg",
				id: Some("1")
			},
			team: Some(Team {
				flag_url: "https://assets.ppy.sh/teams/flag/3761/c8435c6881219d5d71c003535873e96b6a034ccb077b65d80e64c611aa173920.png",
				id: 3761,
				name: "barbecue stand",
				short_name: "BBQ"
			})
		})
	}, NonLegacyScore {
		classic_total_score: 9648577,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [],
		statistics: Statistics {
			great: 863,
			large_tick_hit: 32,
			small_tick_hit: 589
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3731928946,
		rank: "S",
		ranked_type: "solo_score",
		user_id: 36528993,
		accuracy: 0.984085,
		build_id: Some(7739),
		ended_at: "2024-10-23T21:05:29Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: None,
		legacy_total_score: 0,
		max_combo: 239,
		passed: true,
		pp: 15.5346,
		ruleset_id: 2,
		started_at: Some("2024-10-23T21:00:18Z"),
		total_score: 757015,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/36528993?1730976075.jpeg",
			country_code: "FI",
			default_group: "default",
			id: 36528993,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-03-01T16:36:50+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Juibbi",
			country: Country {
				code: "FI",
				name: "Finland"
			},
			cover: Cover {
				custom_url: None,
				url: "https://assets.ppy.sh/user-cover-presets/3/32ddb3eb261e38a82067f9ef4ea96c12f6abf8bd228e6413330f9d351420301b.jpeg",
				id: Some("3")
			},
			team: None
		})
	}, NonLegacyScore {
		classic_total_score: 7261304,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 861,
			large_tick_hit: 32,
			small_tick_hit: 592
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 3742408098,
		rank: "S",
		ranked_type: "solo_score",
		user_id: 4296063,
		accuracy: 0.984748,
		build_id: None,
		ended_at: "2024-10-25T19:42:01Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: Some(218672285),
		legacy_total_score: 3305580,
		max_combo: 270,
		passed: true,
		pp: 16.1743,
		ruleset_id: 2,
		started_at: None,
		total_score: 656325,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/4296063?1452191303.png",
			country_code: "GB",
			default_group: "default",
			id: 4296063,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-02-20T22:36:45+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Ch3zGr8r",
			country: Country {
				code: "GB",
				name: "United Kingdom"
			},
			cover: Cover {
				custom_url: None,
				url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg",
				id: Some("8")
			},
			team: None
		})
	}, NonLegacyScore {
		classic_total_score: 7030387,
		preserve: true,
		processed: true,
		ranked: true,
		maximum_statistics: Statistics {
			great: 879,
			large_tick_hit: 32,
			small_tick_hit: 597
		},
		mods: [AcronymMod {
			acronym: "CL"
		}],
		statistics: Statistics {
			great: 861,
			large_tick_hit: 32,
			small_tick_hit: 585
		},
		beatmap_id: 4801662,
		best_id: None,
		id: 4449820241,
		rank: "S",
		ranked_type: "solo_score",
		user_id: 31175842,
		accuracy: 0.980106,
		build_id: None,
		ended_at: "2025-03-03T05:10:56Z",
		has_replay: true,
		is_perfect_combo: false,
		legacy_perfect: false,
		legacy_score_id: Some(220037892),
		legacy_total_score: 3218918,
		max_combo: 238,
		passed: true,
		pp: 14.1953,
		ruleset_id: 2,
		started_at: None,
		total_score: 645757,
		replay: true,
		current_user_attributes: Some(CurrentUserAttributes {
			pin: None
		}),
		user: Some(User {
			avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
			country_code: "CN",
			default_group: "default",
			id: 31175842,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-03-07T05:29:56+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Islatri",
			country: Country {
				code: "CN",
				name: "China"
			},
			cover: Cover {
				custom_url: Some("https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png"),
				url: "https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png",
				id: None
			},
			team: None
		})
	}]
}
*/