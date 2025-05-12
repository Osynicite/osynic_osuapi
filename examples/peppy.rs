// Client Credentials Grant and Get Peppy's User Info
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let client = OsynicOsuApiV2Client::default();
    let token = client
        .oauth
        .get_token_without_code(client_id.parse()?, &client_secret)
        .await?;
    println!("{:?}", token);

    let peppy = client
        .users
        .get_user_by_username("peppy", None, None)
        .await?;
    println!("{:?}", peppy);
    println!("osu_account_id: {}", peppy.id);
    println!("username: {}", peppy.username);
    println!("join_date: {}", peppy.join_date.unwrap_or_default());
    println!("country_code: {}", peppy.country.as_ref().map_or("None".to_string(), |c| c.code.clone()));
    println!("country_name: {}", peppy.country.as_ref().map_or("None".to_string(), |c| c.name.clone()));
    println!("cover_url: {}", peppy.cover_url.unwrap_or_default());

    
    Ok(())
}

/*
ReqwestOauth get_token_without_code
OToken {
	access_token: "This is a secret~",
	expires_in: 86400,
	refresh_token: None,
	token_type: "Bearer"
}
ReqwestUsers get_user_by_username
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
	cover_url: Some("https://assets.ppy.sh/user-profile-covers/2/baba245ef60834b769694178f8f6d4f6166c5188c740de084656ad2b80f1eea7.jpeg"),
	discord: None,
	has_supported: Some(true),
	interests: None,
	join_date: Some("2007-08-28T03:09:12+00:00"),
	location: None,
	max_blocks: Some(200),
	max_friends: Some(1000),
	occupation: None,
	playmode: Some("osu"),
	playstyle: Some([Mouse, Touch]),
	post_count: Some(19322),
	profile_hue: Some(252),
	profile_order: Some([Me, RecentActivity, Beatmaps, Historical, Kudosu, TopRanks, Medals]),
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
	kudosu: Some(Kudosu {
		available: 62,
		total: 88
	}),
	account_history: Some([]),
	active_tournament_banner: None,
	active_tournament_banners: Some([]),
	badges: Some([]),
	beatmap_playcounts_count: Some(440),
	comments_count: Some(2733),
	daily_challenge_user_stats: Some(DailyChallengeUserStats {
		daily_streak_best: 2,
		daily_streak_current: 0,
		last_update: "2024-08-29T00:00:00+00:00",
		last_weekly_streak: "2024-08-29T00:00:00+00:00",
		playcount: 3,
		top_10p_placements: 0,
		top_50p_placements: 0,
		user_id: 2,
		weekly_streak_best: 1,
		weekly_streak_current: 0
	}),
	favourite_beatmapset_count: Some(14),
	follower_count: Some(55917),
	graveyard_beatmapset_count: Some(8),
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
	guest_beatmapset_count: Some(2),
	loved_beatmapset_count: Some(0),
	mapping_follower_count: Some(2114),
	monthly_playcounts: Some([MonthlyPlaycounts {
		start_date: "2007-10-01",
		count: 483
	}, MonthlyPlaycounts {
		start_date: "2007-11-01",
		count: 381
	}, MonthlyPlaycounts {
		start_date: "2007-12-01",
		count: 94
	}, MonthlyPlaycounts {
		start_date: "2008-01-01",
		count: 63
	}, MonthlyPlaycounts {
		start_date: "2008-02-01",
		count: 272
	}, MonthlyPlaycounts {
		start_date: "2008-03-01",
		count: 261
	}, MonthlyPlaycounts {
		start_date: "2008-04-01",
		count: 527
	}, MonthlyPlaycounts {
		start_date: "2008-05-01",
		count: 331
	}, MonthlyPlaycounts {
		start_date: "2008-06-01",
		count: 120
	}, MonthlyPlaycounts {
		start_date: "2008-07-01",
		count: 58
	}, MonthlyPlaycounts {
		start_date: "2008-08-01",
		count: 303
	}, MonthlyPlaycounts {
		start_date: "2008-09-01",
		count: 268
	}, MonthlyPlaycounts {
		start_date: "2008-10-01",
		count: 98
	}, MonthlyPlaycounts {
		start_date: "2008-11-01",
		count: 119
	}, MonthlyPlaycounts {
		start_date: "2008-12-01",
		count: 199
	}, MonthlyPlaycounts {
		start_date: "2009-01-01",
		count: 105
	}, MonthlyPlaycounts {
		start_date: "2009-02-01",
		count: 94
	}, MonthlyPlaycounts {
		start_date: "2009-03-01",
		count: 133
	}, MonthlyPlaycounts {
		start_date: "2009-04-01",
		count: 63
	}, MonthlyPlaycounts {
		start_date: "2009-05-01",
		count: 32
	}, MonthlyPlaycounts {
		start_date: "2009-06-01",
		count: 158
	}, MonthlyPlaycounts {
		start_date: "2009-07-01",
		count: 125
	}, MonthlyPlaycounts {
		start_date: "2009-08-01",
		count: 84
	}, MonthlyPlaycounts {
		start_date: "2009-09-01",
		count: 43
	}, MonthlyPlaycounts {
		start_date: "2009-10-01",
		count: 401
	}, MonthlyPlaycounts {
		start_date: "2009-11-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2009-12-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2010-01-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2010-02-01",
		count: 25
	}, MonthlyPlaycounts {
		start_date: "2010-03-01",
		count: 20
	}, MonthlyPlaycounts {
		start_date: "2010-04-01",
		count: 13
	}, MonthlyPlaycounts {
		start_date: "2010-08-01",
		count: 24
	}, MonthlyPlaycounts {
		start_date: "2010-09-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2010-11-01",
		count: 53
	}, MonthlyPlaycounts {
		start_date: "2010-12-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2012-04-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2012-09-01",
		count: 22
	}, MonthlyPlaycounts {
		start_date: "2012-10-01",
		count: 189
	}, MonthlyPlaycounts {
		start_date: "2012-11-01",
		count: 14
	}, MonthlyPlaycounts {
		start_date: "2012-12-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2013-01-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2013-02-01",
		count: 61
	}, MonthlyPlaycounts {
		start_date: "2013-03-01",
		count: 196
	}, MonthlyPlaycounts {
		start_date: "2013-04-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2013-05-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2013-06-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2013-07-01",
		count: 13
	}, MonthlyPlaycounts {
		start_date: "2013-08-01",
		count: 26
	}, MonthlyPlaycounts {
		start_date: "2013-09-01",
		count: 155
	}, MonthlyPlaycounts {
		start_date: "2013-10-01",
		count: 31
	}, MonthlyPlaycounts {
		start_date: "2013-11-01",
		count: 29
	}, MonthlyPlaycounts {
		start_date: "2013-12-01",
		count: 197
	}, MonthlyPlaycounts {
		start_date: "2014-01-01",
		count: 25
	}, MonthlyPlaycounts {
		start_date: "2014-02-01",
		count: 113
	}, MonthlyPlaycounts {
		start_date: "2014-03-01",
		count: 270
	}, MonthlyPlaycounts {
		start_date: "2014-04-01",
		count: 52
	}, MonthlyPlaycounts {
		start_date: "2014-05-01",
		count: 21
	}, MonthlyPlaycounts {
		start_date: "2014-06-01",
		count: 126
	}, MonthlyPlaycounts {
		start_date: "2014-07-01",
		count: 158
	}, MonthlyPlaycounts {
		start_date: "2014-08-01",
		count: 19
	}, MonthlyPlaycounts {
		start_date: "2014-09-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2014-10-01",
		count: 22
	}, MonthlyPlaycounts {
		start_date: "2014-11-01",
		count: 28
	}, MonthlyPlaycounts {
		start_date: "2014-12-01",
		count: 45
	}, MonthlyPlaycounts {
		start_date: "2015-01-01",
		count: 24
	}, MonthlyPlaycounts {
		start_date: "2015-02-01",
		count: 15
	}, MonthlyPlaycounts {
		start_date: "2015-03-01",
		count: 24
	}, MonthlyPlaycounts {
		start_date: "2015-04-01",
		count: 34
	}, MonthlyPlaycounts {
		start_date: "2015-05-01",
		count: 15
	}, MonthlyPlaycounts {
		start_date: "2015-06-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2015-07-01",
		count: 26
	}, MonthlyPlaycounts {
		start_date: "2015-08-01",
		count: 36
	}, MonthlyPlaycounts {
		start_date: "2015-09-01",
		count: 35
	}, MonthlyPlaycounts {
		start_date: "2015-10-01",
		count: 104
	}, MonthlyPlaycounts {
		start_date: "2015-11-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2016-01-01",
		count: 13
	}, MonthlyPlaycounts {
		start_date: "2016-02-01",
		count: 30
	}, MonthlyPlaycounts {
		start_date: "2016-03-01",
		count: 21
	}, MonthlyPlaycounts {
		start_date: "2016-04-01",
		count: 69
	}, MonthlyPlaycounts {
		start_date: "2016-05-01",
		count: 27
	}, MonthlyPlaycounts {
		start_date: "2016-06-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2016-07-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2016-08-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2016-09-01",
		count: 37
	}, MonthlyPlaycounts {
		start_date: "2016-10-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2016-12-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2017-01-01",
		count: 52
	}, MonthlyPlaycounts {
		start_date: "2017-03-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2017-04-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2017-08-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2017-10-01",
		count: 36
	}, MonthlyPlaycounts {
		start_date: "2017-11-01",
		count: 33
	}, MonthlyPlaycounts {
		start_date: "2017-12-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2018-03-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2018-04-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2018-05-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2018-07-01",
		count: 30
	}, MonthlyPlaycounts {
		start_date: "2018-08-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2018-09-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2018-10-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2018-11-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2018-12-01",
		count: 60
	}, MonthlyPlaycounts {
		start_date: "2019-02-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2019-04-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2019-05-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2019-07-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2019-08-01",
		count: 33
	}, MonthlyPlaycounts {
		start_date: "2019-10-01",
		count: 5
	}, MonthlyPlaycounts {
		start_date: "2019-11-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2019-12-01",
		count: 5
	}, MonthlyPlaycounts {
		start_date: "2020-01-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2020-03-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2020-04-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2020-05-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2020-06-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2020-07-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2020-08-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2020-09-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2020-10-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2020-11-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2020-12-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2021-01-01",
		count: 16
	}, MonthlyPlaycounts {
		start_date: "2021-02-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2021-03-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2021-04-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2021-05-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2021-06-01",
		count: 27
	}, MonthlyPlaycounts {
		start_date: "2021-07-01",
		count: 12
	}, MonthlyPlaycounts {
		start_date: "2021-08-01",
		count: 9
	}, MonthlyPlaycounts {
		start_date: "2021-09-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2021-10-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2021-11-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2021-12-01",
		count: 7
	}, MonthlyPlaycounts {
		start_date: "2022-01-01",
		count: 13
	}, MonthlyPlaycounts {
		start_date: "2022-02-01",
		count: 21
	}, MonthlyPlaycounts {
		start_date: "2022-03-01",
		count: 41
	}, MonthlyPlaycounts {
		start_date: "2022-04-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2022-06-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2022-07-01",
		count: 16
	}, MonthlyPlaycounts {
		start_date: "2022-08-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2022-09-01",
		count: 10
	}, MonthlyPlaycounts {
		start_date: "2022-10-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2022-11-01",
		count: 15
	}, MonthlyPlaycounts {
		start_date: "2022-12-01",
		count: 29
	}, MonthlyPlaycounts {
		start_date: "2023-01-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2023-02-01",
		count: 13
	}, MonthlyPlaycounts {
		start_date: "2023-03-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2023-05-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2023-06-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2023-07-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2023-08-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2023-09-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2023-10-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2023-11-01",
		count: 8
	}, MonthlyPlaycounts {
		start_date: "2023-12-01",
		count: 6
	}, MonthlyPlaycounts {
		start_date: "2024-01-01",
		count: 65
	}, MonthlyPlaycounts {
		start_date: "2024-03-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2024-04-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2024-07-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2024-08-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2024-09-01",
		count: 3
	}, MonthlyPlaycounts {
		start_date: "2024-12-01",
		count: 1
	}, MonthlyPlaycounts {
		start_date: "2025-01-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2025-02-01",
		count: 2
	}, MonthlyPlaycounts {
		start_date: "2025-03-01",
		count: 4
	}, MonthlyPlaycounts {
		start_date: "2025-04-01",
		count: 1
	}]),
	nominated_beatmapset_count: Some(183),
	page: Some(Page {
		html: "<div class='bbcode bbcode--profile-page'><center><br /><span style=\"font-size:85%;\">(<span style=\"color:#f462a3;\">๑</span>・ω・<span style=\"color:#f462a3;\">๑</span>)</span><br /><br />(<span style=\"color:#f462a3;\">๑</span>・ω・<span style=\"color:#f462a3;\">๑</span>)<br /><br /><span style=\"font-size:150%;\">(<span style=\"color:#f462a3;\">๑</span>・ω・<span style=\"color:#f462a3;\">๑</span>)</span><br /><br /></center><br /><br /><center><span style=\"font-size:85%;\"><a rel=\"nofollow\" href=\"http://blog.ppy.sh/\">dev blog</a> | <a rel=\"nofollow\" href=\"http://osu.ppy.sh/p/changelog\">changelog</a> | <a rel=\"nofollow\" href=\"http://osu.ppy.sh/community/forums/topics/83155\">make osu! more awesome!</a> | <a rel=\"nofollow\" href=\"http://osustream.com\">osu!stream (iOS)</a></span></center></div>",
		raw: "[centre]\n[size=85]([color=#f462a3]๑[/color]・ω・[color=#f462a3]๑[/color])[/size]\n\n([color=#f462a3]๑[/color]・ω・[color=#f462a3]๑[/color])\n\n[size=150]([color=#f462a3]๑[/color]・ω・[color=#f462a3]๑[/color])[/size]\n\n[/centre]\n\n[centre][size=85][url=http://blog.ppy.sh/]dev blog[/url] | [url=http://osu.ppy.sh/p/changelog]changelog[/url] | [url=http://osu.ppy.sh/community/forums/topics/83155]make osu! more awesome![/url] | [url=http://osustream.com]osu!stream (iOS)[/url][/size][/centre]"
	}),
	pending_beatmapset_count: Some(1),
	previous_usernames: Some([]),
	rank_highest: Some(RankHighest {
		rank: 243738,
		updated_at: "2017-07-07T00:00:00Z"
	}),
	ranked_beatmapset_count: Some(11),
	replays_watched_counts: Some([ReplaysWatchedCount {
		start_date: "2012-09-01",
		count: 25
	}, ReplaysWatchedCount {
		start_date: "2012-10-01",
		count: 28
	}, ReplaysWatchedCount {
		start_date: "2013-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2013-02-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2013-03-01",
		count: 14
	}, ReplaysWatchedCount {
		start_date: "2013-09-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2013-10-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2013-11-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2013-12-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-02-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2014-03-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2014-04-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2014-05-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-06-01",
		count: 5
	}, ReplaysWatchedCount {
		start_date: "2014-07-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-08-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-10-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-11-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2014-12-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2015-01-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2015-02-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2015-03-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2015-04-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2015-05-01",
		count: 6
	}, ReplaysWatchedCount {
		start_date: "2015-07-01",
		count: 12
	}, ReplaysWatchedCount {
		start_date: "2015-08-01",
		count: 17
	}, ReplaysWatchedCount {
		start_date: "2015-09-01",
		count: 34
	}, ReplaysWatchedCount {
		start_date: "2015-10-01",
		count: 85
	}, ReplaysWatchedCount {
		start_date: "2015-11-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2015-12-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2016-01-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2016-02-01",
		count: 7
	}, ReplaysWatchedCount {
		start_date: "2016-03-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2016-05-01",
		count: 5
	}, ReplaysWatchedCount {
		start_date: "2016-06-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2016-07-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2016-08-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2016-09-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2016-10-01",
		count: 52
	}, ReplaysWatchedCount {
		start_date: "2016-11-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2016-12-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2017-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-02-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-03-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-04-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2017-05-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-06-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-07-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-08-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2017-09-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2017-10-01",
		count: 9
	}, ReplaysWatchedCount {
		start_date: "2017-11-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2017-12-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2018-01-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2018-02-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2018-03-01",
		count: 5
	}, ReplaysWatchedCount {
		start_date: "2018-05-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2018-06-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2018-07-01",
		count: 8
	}, ReplaysWatchedCount {
		start_date: "2018-08-01",
		count: 636
	}, ReplaysWatchedCount {
		start_date: "2018-09-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2018-10-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2018-12-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2019-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2019-02-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2019-04-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2019-05-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2019-06-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2019-07-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2019-10-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2019-12-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2020-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2020-02-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2020-03-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2020-04-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2020-05-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2020-06-01",
		count: 115
	}, ReplaysWatchedCount {
		start_date: "2020-07-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2020-08-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2020-09-01",
		count: 13
	}, ReplaysWatchedCount {
		start_date: "2020-10-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2020-11-01",
		count: 6
	}, ReplaysWatchedCount {
		start_date: "2021-01-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2021-02-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2021-03-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2021-04-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2021-05-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2021-06-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2021-08-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2021-09-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2021-12-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2022-03-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2022-04-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2022-05-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2022-06-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2022-07-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2022-09-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2022-11-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2022-12-01",
		count: 11
	}, ReplaysWatchedCount {
		start_date: "2023-01-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2023-02-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2023-05-01",
		count: 14
	}, ReplaysWatchedCount {
		start_date: "2023-06-01",
		count: 5
	}, ReplaysWatchedCount {
		start_date: "2023-08-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2023-09-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2023-11-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2024-01-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2024-02-01",
		count: 11
	}, ReplaysWatchedCount {
		start_date: "2024-03-01",
		count: 4
	}, ReplaysWatchedCount {
		start_date: "2024-04-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2024-06-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2024-07-01",
		count: 13
	}, ReplaysWatchedCount {
		start_date: "2024-08-01",
		count: 3
	}, ReplaysWatchedCount {
		start_date: "2024-11-01",
		count: 2
	}, ReplaysWatchedCount {
		start_date: "2024-12-01",
		count: 1
	}, ReplaysWatchedCount {
		start_date: "2025-03-01",
		count: 2
	}]),
	scores_best_count: Some(200),
	scores_first_count: Some(0),
	scores_pinned_count: Some(5),
	scores_recent_count: Some(0),
	session_verified: None,
	statistics: Some(Statistics {
		count_100: 123089,
		count_300: 680893,
		count_50: 27074,
		count_miss: 73922,
		country_rank: Some(15054),
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
		rank: Some(Rank {
			global: None,
			country: 15054
		}),
		variants: None
	}),
	statistics_rulesets: None,
	support_level: Some(3),
	team: Some(Team {
		flag_url: "https://assets.ppy.sh/teams/flag/1/b46fb10dbfd8a35dc50e6c00296c0dc6172dffc3ed3d3a4b379277ba498399fe.png",
		id: 1,
		name: "mom?",
		short_name: "MOM"
	}),
	user_achievements: Some([UserAchievements {
		achieved_at: "2024-09-06T11:57:49Z",
		achievement_id: 328
	}, UserAchievements {
		achieved_at: "2024-08-29T00:32:43Z",
		achievement_id: 95
	}, UserAchievements {
		achieved_at: "2024-08-29T00:32:43Z",
		achievement_id: 31
	}, UserAchievements {
		achieved_at: "2019-10-24T09:16:47Z",
		achievement_id: 177
	}, UserAchievements {
		achieved_at: "2019-10-24T09:07:16Z",
		achievement_id: 124
	}, UserAchievements {
		achieved_at: "2019-07-29T02:40:32Z",
		achievement_id: 79
	}, UserAchievements {
		achieved_at: "2018-12-19T07:33:32Z",
		achievement_id: 71
	}, UserAchievements {
		achieved_at: "2018-12-19T07:23:27Z",
		achievement_id: 63
	}, UserAchievements {
		achieved_at: "2017-10-28T00:56:59Z",
		achievement_id: 55
	}, UserAchievements {
		achieved_at: "2017-01-23T09:35:35Z",
		achievement_id: 72
	}, UserAchievements {
		achieved_at: "2017-01-14T05:12:27Z",
		achievement_id: 128
	}, UserAchievements {
		achieved_at: "2016-10-17T06:04:28Z",
		achievement_id: 42
	}, UserAchievements {
		achieved_at: "2016-09-26T15:08:31Z",
		achievement_id: 64
	}, UserAchievements {
		achieved_at: "2016-09-26T15:08:31Z",
		achievement_id: 56
	}, UserAchievements {
		achieved_at: "2016-09-26T14:35:17Z",
		achievement_id: 127
	}, UserAchievements {
		achieved_at: "2016-09-26T14:23:04Z",
		achievement_id: 132
	}, UserAchievements {
		achieved_at: "2016-09-26T14:23:04Z",
		achievement_id: 58
	}, UserAchievements {
		achieved_at: "2016-05-05T11:24:22Z",
		achievement_id: 57
	}, UserAchievements {
		achieved_at: "2016-04-20T05:47:10Z",
		achievement_id: 54
	}, UserAchievements {
		achieved_at: "2013-02-27T12:31:07Z",
		achievement_id: 1
	}, UserAchievements {
		achieved_at: "2013-02-27T12:15:27Z",
		achievement_id: 39
	}, UserAchievements {
		achieved_at: "2012-10-28T14:57:49Z",
		achievement_id: 20
	}]),
	rank_history: Some(RankHistory {
		mode: "osu",
		data: [757994, 758765, 759485, 760055, 760657, 761299, 761943, 762590, 763349, 764085, 764716, 765327, 765935, 766562, 767279, 767993, 768681, 769252, 769868, 770455, 771082, 771683, 772302, 772899, 773450, 773981, 774543, 685532, 686279, 687101, 687886, 688721, 689696, 690349, 691095, 691854, 692577, 693345, 694285, 695240, 696116, 696906, 697690, 698426, 699210, 700042, 700959, 701800, 702549, 703263, 703983, 704626, 705332, 706171, 706944, 707620, 708284, 708914, 709619, 710359, 711163, 711894, 712566, 713245, 713959, 714706, 715503, 716312, 717024, 717692, 718385, 719053, 719725, 720477, 721215, 722008, 722680, 723325, 724019, 724776, 725499, 726286, 727037, 727664, 728274, 728889, 729547, 730218, 731011, 731011]
	}),
	rank_istoriya: Some(RankHistory {
		mode: "osu",
		data: [757994, 758765, 759485, 760055, 760657, 761299, 761943, 762590, 763349, 764085, 764716, 765327, 765935, 766562, 767279, 767993, 768681, 769252, 769868, 770455, 771082, 771683, 772302, 772899, 773450, 773981, 774543, 685532, 686279, 687101, 687886, 688721, 689696, 690349, 691095, 691854, 692577, 693345, 694285, 695240, 696116, 696906, 697690, 698426, 699210, 700042, 700959, 701800, 702549, 703263, 703983, 704626, 705332, 706171, 706944, 707620, 708284, 708914, 709619, 710359, 711163, 711894, 712566, 713245, 713959, 714706, 715503, 716312, 717024, 717692, 718385, 719053, 719725, 720477, 721215, 722008, 722680, 723325, 724019, 724776, 725499, 726286, 727037, 727664, 728274, 728889, 729547, 730218, 731011, 731011]
	}),
	ranked_and_approved_beatmapset_count: Some(11),
	unranked_beatmapset_count: Some(1)
}
osu_account_id: 2
username: peppy
join_date: 2007 - 08 - 28 T03: 09: 12 + 00: 00
country_code: AU
country_name: Australia
cover_url: https: //assets.ppy.sh/user-profile-covers/2/baba245ef60834b769694178f8f6d4f6166c5188c740de084656ad2b80f1eea7.jpeg
*/