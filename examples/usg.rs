// Get users information by ids
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

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
    let users = client.users.get_users(vec![2,31175842], None).await?;

    for user in users.users {
        println!("{:?}", user);
        println!("osu_account_id: {}", user.id);
        println!("username: {}", user.username);
        println!("join_date: {}", user.join_date.unwrap_or_default());
		println!("country_code: {}", user.country.as_ref().map_or("None".to_string(), |c| c.code.clone()));
	    println!("country_name: {}", user.country.as_ref().map_or("None".to_string(), |c| c.name.clone()));
        println!("cover_url: {}", user.cover_url.unwrap_or_default());
    }
    Ok(())
}


/*
ReqwestUsers get_users
User {
	avatar_url: "https://a.ppy.sh/2?1657169614.png",
	country_code: "AU",
	default_group: Some("default"),
	id: 2,
	is_active: true,
	is_bot: false,
	is_deleted: false,
	is_online: false,
	is_supporter: true,
	last_visit: Some("2025-05-11T12:40:33+00:00"),
	pm_friends_only: false,
	profile_colour: Some("#3366FF"),
	username: "peppy",
	cover_url: None,
	discord: None,
	has_supported: None,
	interests: None,
	join_date: None,
	location: None,
	max_blocks: None,
	max_friends: None,
	occupation: None,
	playmode: None,
	playstyle: None,
	post_count: None,
	profile_hue: None,
	profile_order: None,
	title: None,
	title_url: None,
	twitter: None,
	website: None,
	country: Country {
		code: "AU",
		name: "Australia"
	},
	cover: Cover {
		custom_url: Some("https://assets.ppy.sh/user-profile-covers/2/baba245ef60834b769694178f8f6d4f6166c5188c740de084656ad2b80f1eea7.jpeg"),
		url: "https://assets.ppy.sh/user-profile-covers/2/baba245ef60834b769694178f8f6d4f6166c5188c740de084656ad2b80f1eea7.jpeg",
		id: None
	},
	is_restricted: None,
	kudosu: None,
	account_history: None,
	active_tournament_banner: None,
	active_tournament_banners: None,
	badges: None,
	beatmap_playcounts_count: None,
	comments_count: None,
	daily_challenge_user_stats: None,
	favourite_beatmapset_count: None,
	follower_count: None,
	graveyard_beatmapset_count: None,
	groups: Some([Group {
		colour: "#0066FF",
		has_listing: false,
		has_playmodes: false,
		id: 33,
		identifier: "ppy",
		is_probationary: false,
		name: "ppy",
		short_name: "PPY",
		playmodes: None
	}, Group {
		colour: "#E45678",
		has_listing: true,
		has_playmodes: false,
		id: 11,
		identifier: "dev",
		is_probationary: false,
		name: "Developers",
		short_name: "DEV",
		playmodes: None
	}]),
	guest_beatmapset_count: None,
	loved_beatmapset_count: None,
	mapping_follower_count: None,
	monthly_playcounts: None,
	nominated_beatmapset_count: None,
	page: None,
	pending_beatmapset_count: None,
	previous_usernames: None,
	rank_highest: None,
	ranked_beatmapset_count: None,
	replays_watched_counts: None,
	scores_best_count: None,
	scores_first_count: None,
	scores_pinned_count: None,
	scores_recent_count: None,
	session_verified: None,
	statistics: None,
	statistics_rulesets: Some(StatisticsRulesets {
		osu: Statistics {
			count_100: 123089,
			count_300: 680893,
			count_50: 27074,
			count_miss: 73922,
			country_rank: None,
			level: Level {
				current: 67,
				progress: 12
			},
			global_rank: Some(731011),
			global_rank_exp: None,
			pp: 1145.05,
			pp_exp: Some(0.0),
			ranked_score: 449443736,
			hit_accuracy: 96.5903,
			play_count: 7709,
			play_time: 739975,
			total_score: 1993821320,
			total_hits: 831056,
			maximum_combo: 746,
			replays_watched_by_others: 16670,
			rank_change_since_30_days: None,
			is_ranked: true,
			grade_counts: GradeCounts {
				ss: 15,
				ssh: 0,
				s: 65,
				sh: 0,
				a: 174
			},
			rank: None,
			variants: None
		},
		taiko: Statistics {
			count_100: 12946,
			count_300: 113303,
			count_50: 0,
			count_miss: 10958,
			country_rank: None,
			level: Level {
				current: 22,
				progress: 60
			},
			global_rank: None,
			global_rank_exp: None,
			pp: 0.0,
			pp_exp: Some(0.0),
			ranked_score: 37776814,
			hit_accuracy: 82.0586,
			play_count: 411,
			play_time: 28397,
			total_score: 74535189,
			total_hits: 126249,
			maximum_combo: 1597,
			replays_watched_by_others: 7,
			rank_change_since_30_days: None,
			is_ranked: false,
			grade_counts: GradeCounts {
				ss: 0,
				ssh: 0,
				s: 0,
				sh: 0,
				a: 4
			},
			rank: None,
			variants: None
		},
		fruits: Statistics {
			count_100: 5632,
			count_300: 84800,
			count_50: 47808,
			count_miss: 5639,
			country_rank: None,
			level: Level {
				current: 40,
				progress: 11
			},
			global_rank: None,
			global_rank_exp: None,
			pp: 0.0,
			pp_exp: Some(0.0),
			ranked_score: 144123193,
			hit_accuracy: 97.9679,
			play_count: 409,
			play_time: 26884,
			total_score: 422307182,
			total_hits: 138240,
			maximum_combo: 1345,
			replays_watched_by_others: 12,
			rank_change_since_30_days: None,
			is_ranked: false,
			grade_counts: GradeCounts {
				ss: 0,
				ssh: 0,
				s: 2,
				sh: 0,
				a: 2
			},
			rank: None,
			variants: None
		},
		mania: Statistics {
			count_100: 154888,
			count_300: 723548,
			count_50: 8935,
			count_miss: 26926,
			country_rank: None,
			level: Level {
				current: 45,
				progress: 6
			},
			global_rank: Some(602816),
			global_rank_exp: None,
			pp: 25.5721,
			pp_exp: Some(0.0),
			ranked_score: 372290000,
			hit_accuracy: 74.1436,
			play_count: 1563,
			play_time: 129977,
			total_score: 600054028,
			total_hits: 887371,
			maximum_combo: 3060,
			replays_watched_by_others: 4,
			rank_change_since_30_days: None,
			is_ranked: true,
			grade_counts: GradeCounts {
				ss: 0,
				ssh: 0,
				s: 0,
				sh: 0,
				a: 1
			},
			rank: None,
			variants: None
		}
	}),
	support_level: None,
	team: Some(Team {
		flag_url: "https://assets.ppy.sh/teams/flag/1/b46fb10dbfd8a35dc50e6c00296c0dc6172dffc3ed3d3a4b379277ba498399fe.png",
		id: 1,
		name: "mom?",
		short_name: "MOM"
	}),
	user_achievements: None,
	rank_history: None,
	rank_istoriya: None,
	ranked_and_approved_beatmapset_count: None,
	unranked_beatmapset_count: None
}
osu_account_id: 2
username: peppy
join_date:
	country_code: AU
country_name: Australia
cover_url:
	User {
		avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
		country_code: "CN",
		default_group: Some("default"),
		id: 31175842,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: false,
		last_visit: Some("2025-05-11T12:46:15+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Islatri",
		cover_url: None,
		discord: None,
		has_supported: None,
		interests: None,
		join_date: None,
		location: None,
		max_blocks: None,
		max_friends: None,
		occupation: None,
		playmode: None,
		playstyle: None,
		post_count: None,
		profile_hue: None,
		profile_order: None,
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
		is_restricted: None,
		kudosu: None,
		account_history: None,
		active_tournament_banner: None,
		active_tournament_banners: None,
		badges: None,
		beatmap_playcounts_count: None,
		comments_count: None,
		daily_challenge_user_stats: None,
		favourite_beatmapset_count: None,
		follower_count: None,
		graveyard_beatmapset_count: None,
		groups: Some([]),
		guest_beatmapset_count: None,
		loved_beatmapset_count: None,
		mapping_follower_count: None,
		monthly_playcounts: None,
		nominated_beatmapset_count: None,
		page: None,
		pending_beatmapset_count: None,
		previous_usernames: None,
		rank_highest: None,
		ranked_beatmapset_count: None,
		replays_watched_counts: None,
		scores_best_count: None,
		scores_first_count: None,
		scores_pinned_count: None,
		scores_recent_count: None,
		session_verified: None,
		statistics: None,
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
		support_level: None,
		team: None,
		user_achievements: None,
		rank_history: None,
		rank_istoriya: None,
		ranked_and_approved_beatmapset_count: None,
		unranked_beatmapset_count: None
	}
osu_account_id: 31175842
username: Islatri
join_date:
	country_code: CN
country_name: China
cover_url:
*/