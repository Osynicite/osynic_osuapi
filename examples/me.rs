// Get your user information by oauth token
use osynic_osuapi::error::Result;
use osynic_osuapi::utils::country_code_to_unicode_flag;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // ACG can't get me
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let me = client.users.get_own_data(None, None).await?;
    println!("{:?}", me);
    println!("osu_account_id: {}", me.id);
    println!("username: {}", me.username);
    // println!("join_date: {}", me.join_date);
    // println!("country_name: {}", me.country.name);
    // println!("country_code: {}", me.country.code);
    let country_flag = country_code_to_unicode_flag(&me.country.code).unwrap_or_default();
    // println!("country_flag: {}", country_flag);
    let country_svg_path = format!(
        "https://osu.ppy.sh/assets/images/flags/{}.svg",
        country_flag
    );
    println!("country_svg_path: {}", country_svg_path);
    // playmode
    println!("playmode: {:?}", me.playmode);
    // // statistics
    // println!("statistics: {:?}", me.statistics);
    // // statistics_rulesets
    // println!("statistics_rulesets: {:?}", me.statistics_rulesets);

    // acc
    println!("acc: {:?}", me.statistics.as_ref().map(|s| s.hit_accuracy));
    // level
    println!("level: {:?}", me.statistics.as_ref().map(|s| s.level.current));
    // progress
    println!("progress: {:?}", me.statistics.as_ref().map(|s| s.level.progress));
    // global_rank
    println!("global_rank: {:?}", me.statistics.as_ref().map(|s| s.global_rank));
    // country_rank
    println!("country_rank: {:?}", me.statistics.as_ref().map(|s| s.country_rank));
    // pp
    println!("pp: {:?}", me.statistics.as_ref().map(|s| s.pp));
    println!("avatar_url: {}", me.avatar_url);
    println!("cover_url: {}", me.cover_url.unwrap_or_default());
    Ok(())
}


/*
ReqwestUsers get_own_data
User {
	avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
	country_code: "CN",
	default_group: Some("default"),
	id: 31175842,
	is_active: true,
	is_bot: false,
	is_deleted: false,
	is_online: true,
	is_supporter: false,
	last_visit: Some("2025-05-11T15:44:11+00:00"),
	pm_friends_only: false,
	profile_colour: None,
	username: "Islatri",
	cover_url: Some("https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png"),
	discord: None,
	has_supported: Some(true),
	interests: Some("QAQ"),
	join_date: Some("2022-08-30T06:57:47+00:00"),
	location: None,
	max_blocks: Some(100),
	max_friends: Some(500),
	occupation: Some("qwq"),
	playmode: Some("fruits"),
	playstyle: Some([Mouse, Keyboard, Tablet]),
	post_count: Some(0),
	profile_hue: Some(201),
	profile_order: Some([Me, RecentActivity, TopRanks, Medals, Historical, Beatmaps, Kudosu]),
	title: None,
	title_url: None,
	twitter: None,
	website: None,
	country: Country {
		code: "CN",
		name: "China"
	},
	cover: Cover {
		custom_url: Some("https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png"),
		url: "https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png",
		id: None
	},
	is_restricted: Some(false),
	kudosu: Some(Kudosu {
		available: 0,
		total: 0
	}),
	account_history: Some([]),
	active_tournament_banner: None,
	active_tournament_banners: Some([]),
	badges: Some([]),
	beatmap_playcounts_count: Some(2129),
	comments_count: Some(0),
	daily_challenge_user_stats: Some(DailyChallengeUserStats {
		daily_streak_best: 0,
		daily_streak_current: 0,
		last_update: "2000-01-01T00:00:00+00:00",
		last_weekly_streak: "2000-01-01T00:00:00+00:00",
		playcount: 0,
		top_10p_placements: 0,
		top_50p_placements: 0,
		user_id: 31175842,
		weekly_streak_best: 0,
		weekly_streak_current: 0
	}),
	favourite_beatmapset_count: Some(0),
	follower_count: Some(12),
	graveyard_beatmapset_count: Some(0),
	groups: Some([]),
	guest_beatmapset_count: Some(0),
	loved_beatmapset_count: Some(0),
	mapping_follower_count: Some(0),
	monthly_playcounts: Some([MonthlyPlaycounts {
		start_date: "2022-08-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2022-09-01",
		count: 314
	}, MonthlyPlaycounts {
		start_date: "2022-10-01",
		count: 127
	}, MonthlyPlaycounts {
		start_date: "2022-11-01",
		count: 377
	}, MonthlyPlaycounts {
		start_date: "2022-12-01",
		count: 1339
	}, MonthlyPlaycounts {
		start_date: "2023-01-01",
		count: 806
	}, MonthlyPlaycounts {
		start_date: "2023-02-01",
		count: 423
	}, MonthlyPlaycounts {
		start_date: "2023-03-01",
		count: 587
	}, MonthlyPlaycounts {
		start_date: "2023-04-01",
		count: 419
	}, MonthlyPlaycounts {
		start_date: "2023-05-01",
		count: 146
	}, MonthlyPlaycounts {
		start_date: "2023-06-01",
		count: 414
	}, MonthlyPlaycounts {
		start_date: "2023-07-01",
		count: 648
	}, MonthlyPlaycounts {
		start_date: "2023-08-01",
		count: 287
	}, MonthlyPlaycounts {
		start_date: "2023-09-01",
		count: 193
	}, MonthlyPlaycounts {
		start_date: "2023-10-01",
		count: 172
	}, MonthlyPlaycounts {
		start_date: "2023-11-01",
		count: 1415
	}, MonthlyPlaycounts {
		start_date: "2023-12-01",
		count: 308
	}, MonthlyPlaycounts {
		start_date: "2024-01-01",
		count: 110
	}, MonthlyPlaycounts {
		start_date: "2024-02-01",
		count: 24
	}, MonthlyPlaycounts {
		start_date: "2024-03-01",
		count: 289
	}, MonthlyPlaycounts {
		start_date: "2024-04-01",
		count: 189
	}, MonthlyPlaycounts {
		start_date: "2024-05-01",
		count: 462
	}, MonthlyPlaycounts {
		start_date: "2024-06-01",
		count: 167
	}, MonthlyPlaycounts {
		start_date: "2024-07-01",
		count: 131
	}, MonthlyPlaycounts {
		start_date: "2024-08-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2024-09-01",
		count: 7
	}, MonthlyPlaycounts {
		start_date: "2024-10-01",
		count: 82
	}, MonthlyPlaycounts {
		start_date: "2024-11-01",
		count: 5
	}, MonthlyPlaycounts {
		start_date: "2024-12-01",
		count: 105
	}, MonthlyPlaycounts {
		start_date: "2025-02-01",
		count: 48
	}, MonthlyPlaycounts {
		start_date: "2025-03-01",
		count: 182
	}, MonthlyPlaycounts {
		start_date: "2025-04-01",
		count: 1
	}]),
	nominated_beatmapset_count: Some(0),
	page: Some(Page {
		html: "",
		raw: ""
	}),
	pending_beatmapset_count: Some(0),
	previous_usernames: Some(["ChestZone"]),
	rank_highest: Some(RankHighest {
		rank: 22304,
		updated_at: "2025-03-13T15:05:32Z"
	}),
	ranked_beatmapset_count: Some(0),
	replays_watched_counts: Some([]),
	scores_best_count: Some(75),
	scores_first_count: Some(0),
	scores_pinned_count: Some(5),
	scores_recent_count: Some(0),
	session_verified: Some(true),
	statistics: Some(Statistics {
		count_100: 4148,
		count_300: 99865,
		count_50: 77032,
		count_miss: 7652,
		country_rank: Some(770),
		level: Level {
			current: 33,
			progress: 93
		},
		global_rank: Some(24425),
		global_rank_exp: None,
		pp: 530.051,
		pp_exp: Some(0.0),
		ranked_score: 160423637,
		hit_accuracy: 98.2296,
		play_count: 387,
		play_time: 27434,
		total_score: 254706165,
		total_hits: 181045,
		maximum_combo: 688,
		replays_watched_by_others: 0,
		rank_change_since_30_days: None,
		is_ranked: true,
		grade_counts: GradeCounts {
			ss: 1,
			ssh: 0,
			s: 37,
			sh: 0,
			a: 26
		},
		rank: Some(Rank {
			global: None,
			country: 770
		}),
		variants: None
	}),
	statistics_rulesets: Some(StatisticsRulesets {
		osu: Statistics {
			count_100: 136433,
			count_300: 676156,
			count_50: 22732,
			count_miss: 43914,
			country_rank: None,
			level: Level {
				current: 71,
				progress: 51
			},
			global_rank: None,
			global_rank_exp: None,
			pp: 0.0,
			pp_exp: Some(0.0),
			ranked_score: 625695631,
			hit_accuracy: 92.7799,
			play_count: 4966,
			play_time: 321075,
			total_score: 2413072414,
			total_hits: 835321,
			maximum_combo: 1230,
			replays_watched_by_others: 0,
			rank_change_since_30_days: None,
			is_ranked: false,
			grade_counts: GradeCounts {
				ss: 2,
				ssh: 1,
				s: 47,
				sh: 1,
				a: 112
			},
			rank: None,
			variants: None
		},
		taiko: Statistics {
			count_100: 2428,
			count_300: 5940,
			count_50: 0,
			count_miss: 4321,
			country_rank: None,
			level: Level {
				current: 9,
				progress: 80
			},
			global_rank: Some(102857),
			global_rank_exp: None,
			pp: 166.875,
			pp_exp: Some(0.0),
			ranked_score: 1446812,
			hit_accuracy: 91.3801,
			play_count: 71,
			play_time: 4760,
			total_score: 5810050,
			total_hits: 8368,
			maximum_combo: 222,
			replays_watched_by_others: 0,
			rank_change_since_30_days: None,
			is_ranked: true,
			grade_counts: GradeCounts {
				ss: 0,
				ssh: 0,
				s: 0,
				sh: 0,
				a: 7
			},
			rank: None,
			variants: None
		},
		fruits: Statistics {
			count_100: 4148,
			count_300: 99865,
			count_50: 77032,
			count_miss: 7652,
			country_rank: None,
			level: Level {
				current: 33,
				progress: 93
			},
			global_rank: Some(24425),
			global_rank_exp: None,
			pp: 530.051,
			pp_exp: Some(0.0),
			ranked_score: 160423637,
			hit_accuracy: 98.2296,
			play_count: 387,
			play_time: 27434,
			total_score: 254706165,
			total_hits: 181045,
			maximum_combo: 688,
			replays_watched_by_others: 0,
			rank_change_since_30_days: None,
			is_ranked: true,
			grade_counts: GradeCounts {
				ss: 1,
				ssh: 0,
				s: 37,
				sh: 0,
				a: 26
			},
			rank: None,
			variants: None
		},
		mania: Statistics {
			count_100: 412301,
			count_300: 1417953,
			count_50: 22874,
			count_miss: 71735,
			country_rank: None,
			level: Level {
				current: 57,
				progress: 23
			},
			global_rank: Some(137627),
			global_rank_exp: None,
			pp: 1651.16,
			pp_exp: Some(0.0),
			ranked_score: 302522555,
			hit_accuracy: 95.403,
			play_count: 4360,
			play_time: 340840,
			total_score: 1233779843,
			total_hits: 1853128,
			maximum_combo: 3578,
			replays_watched_by_others: 0,
			rank_change_since_30_days: None,
			is_ranked: true,
			grade_counts: GradeCounts {
				ss: 6,
				ssh: 0,
				s: 218,
				sh: 0,
				a: 122
			},
			rank: None,
			variants: None
		}
	}),
	support_level: Some(0),
	team: None,
	user_achievements: Some([UserAchievements {
		achieved_at: "2025-03-02T01:30:27Z",
		achievement_id: 81
	}, UserAchievements {
		achieved_at: "2024-12-19T11:37:56Z",
		achievement_id: 50
	}, UserAchievements {
		achieved_at: "2024-06-03T07:11:07Z",
		achievement_id: 103
	}, UserAchievements {
		achieved_at: "2024-05-29T04:17:45Z",
		achievement_id: 309
	}, UserAchievements {
		achieved_at: "2024-05-20T14:23:33Z",
		achievement_id: 13
	}, UserAchievements {
		achieved_at: "2024-05-19T10:02:50Z",
		achievement_id: 80
	}, UserAchievements {
		achieved_at: "2024-03-20T12:39:16Z",
		achievement_id: 328
	}, UserAchievements {
		achieved_at: "2023-12-31T15:10:14Z",
		achievement_id: 331
	}, UserAchievements {
		achieved_at: "2023-12-15T06:34:32Z",
		achievement_id: 90
	}, UserAchievements {
		achieved_at: "2023-11-27T14:53:17Z",
		achievement_id: 59
	}, UserAchievements {
		achieved_at: "2023-11-19T09:21:00Z",
		achievement_id: 132
	}, UserAchievements {
		achieved_at: "2023-11-04T15:51:33Z",
		achievement_id: 148
	}, UserAchievements {
		achieved_at: "2023-11-03T10:48:21Z",
		achievement_id: 72
	}, UserAchievements {
		achieved_at: "2023-10-30T23:36:53Z",
		achievement_id: 303
	}, UserAchievements {
		achieved_at: "2023-09-13T08:24:12Z",
		achievement_id: 113
	}, UserAchievements {
		achieved_at: "2023-06-16T14:19:59Z",
		achievement_id: 89
	}, UserAchievements {
		achieved_at: "2023-06-08T06:11:41Z",
		achievement_id: 112
	}, UserAchievements {
		achieved_at: "2023-05-17T08:24:32Z",
		achievement_id: 47
	}, UserAchievements {
		achieved_at: "2023-02-10T09:28:44Z",
		achievement_id: 88
	}, UserAchievements {
		achieved_at: "2023-01-31T04:30:55Z",
		achievement_id: 46
	}, UserAchievements {
		achieved_at: "2023-01-27T00:53:14Z",
		achievement_id: 111
	}, UserAchievements {
		achieved_at: "2023-01-10T05:44:18Z",
		achievement_id: 4
	}, UserAchievements {
		achieved_at: "2023-01-04T11:13:16Z",
		achievement_id: 3
	}, UserAchievements {
		achieved_at: "2022-12-18T07:07:10Z",
		achievement_id: 58
	}, UserAchievements {
		achieved_at: "2022-12-16T23:38:01Z",
		achievement_id: 65
	}, UserAchievements {
		achieved_at: "2022-12-16T12:34:44Z",
		achievement_id: 131
	}, UserAchievements {
		achieved_at: "2022-12-16T01:15:02Z",
		achievement_id: 125
	}, UserAchievements {
		achieved_at: "2022-12-16T01:09:13Z",
		achievement_id: 124
	}, UserAchievements {
		achieved_at: "2022-12-16T00:58:21Z",
		achievement_id: 128
	}, UserAchievements {
		achieved_at: "2022-12-16T00:56:40Z",
		achievement_id: 138
	}, UserAchievements {
		achieved_at: "2022-12-16T00:55:22Z",
		achievement_id: 126
	}, UserAchievements {
		achieved_at: "2022-12-15T14:45:38Z",
		achievement_id: 123
	}, UserAchievements {
		achieved_at: "2022-12-15T14:44:24Z",
		achievement_id: 122
	}, UserAchievements {
		achieved_at: "2022-12-15T14:39:31Z",
		achievement_id: 121
	}, UserAchievements {
		achieved_at: "2022-12-15T14:31:34Z",
		achievement_id: 120
	}, UserAchievements {
		achieved_at: "2022-12-15T14:28:37Z",
		achievement_id: 119
	}, UserAchievements {
		achieved_at: "2022-12-15T13:49:31Z",
		achievement_id: 39
	}, UserAchievements {
		achieved_at: "2022-12-15T13:17:38Z",
		achievement_id: 64
	}, UserAchievements {
		achieved_at: "2022-12-06T15:18:56Z",
		achievement_id: 71
	}, UserAchievements {
		achieved_at: "2022-11-26T09:01:34Z",
		achievement_id: 127
	}, UserAchievements {
		achieved_at: "2022-11-20T01:10:19Z",
		achievement_id: 87
	}, UserAchievements {
		achieved_at: "2022-11-20T01:10:19Z",
		achievement_id: 54
	}, UserAchievements {
		achieved_at: "2022-11-16T04:40:30Z",
		achievement_id: 79
	}, UserAchievements {
		achieved_at: "2022-09-21T07:44:36Z",
		achievement_id: 57
	}, UserAchievements {
		achieved_at: "2022-09-07T06:08:05Z",
		achievement_id: 63
	}, UserAchievements {
		achieved_at: "2022-09-05T12:25:25Z",
		achievement_id: 1
	}, UserAchievements {
		achieved_at: "2022-09-02T04:40:30Z",
		achievement_id: 176
	}, UserAchievements {
		achieved_at: "2022-08-31T01:44:30Z",
		achievement_id: 56
	}, UserAchievements {
		achieved_at: "2022-08-30T07:14:28Z",
		achievement_id: 55
	}]),
	rank_history: Some(RankHistory {
		mode: "fruits",
		data: [32942, 32994, 33046, 33095, 33130, 33165, 33207, 33256, 33311, 33355, 33400, 33437, 33482, 33532, 33592, 33646, 33695, 33733, 32221, 31934, 31981, 32034, 30164, 29923, 29967, 29472, 29360, 22635, 22673, 22532, 22338, 22384, 22443, 22485, 22514, 22554, 22590, 22629, 22680, 22732, 22765, 22797, 22835, 22870, 22904, 22937, 22984, 23022, 23057, 23090, 23124, 23158, 23190, 23240, 23262, 23294, 23322, 23362, 23393, 23424, 23463, 23507, 23538, 23576, 23619, 23658, 23704, 23740, 23779, 23817, 23847, 23880, 23907, 23936, 23976, 24024, 24047, 24078, 24102, 24136, 24170, 24203, 24232, 24265, 24290, 24319, 24349, 24386, 24425, 24425]
	}),
	rank_istoriya: Some(RankHistory {
		mode: "fruits",
		data: [32942, 32994, 33046, 33095, 33130, 33165, 33207, 33256, 33311, 33355, 33400, 33437, 33482, 33532, 33592, 33646, 33695, 33733, 32221, 31934, 31981, 32034, 30164, 29923, 29967, 29472, 29360, 22635, 22673, 22532, 22338, 22384, 22443, 22485, 22514, 22554, 22590, 22629, 22680, 22732, 22765, 22797, 22835, 22870, 22904, 22937, 22984, 23022, 23057, 23090, 23124, 23158, 23190, 23240, 23262, 23294, 23322, 23362, 23393, 23424, 23463, 23507, 23538, 23576, 23619, 23658, 23704, 23740, 23779, 23817, 23847, 23880, 23907, 23936, 23976, 24024, 24047, 24078, 24102, 24136, 24170, 24203, 24232, 24265, 24290, 24319, 24349, 24386, 24425, 24425]
	}),
	ranked_and_approved_beatmapset_count: Some(0),
	unranked_beatmapset_count: Some(0)
}
osu_account_id: 31175842
username: Islatri
country_svg_path: https: //osu.ppy.sh/assets/images/flags/1f1e8-1f1f3.svg
	playmode: Some("fruits")
acc: Some(98.2296)
level: Some(33)
progress: Some(93)
global_rank: Some(Some(24425))
country_rank: Some(Some(770))
pp: Some(530.051)
avatar_url: https: //a.ppy.sh/31175842?1738717996.jpeg
	cover_url: https: //assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png
*/