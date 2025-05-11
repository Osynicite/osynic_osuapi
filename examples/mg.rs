// Get matchh page
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::matches::IMatches;
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
    let matchh = client
        .matches
        .get_match(118061894, None, None, None)
        .await?;
    println!("{:?}", matchh);
    Ok(())
}

/*
ReqwestMatches get_match
GetMatchResponse {
	matchh: Match {
		id: 118061894,
		start_time: "2025-05-11T12:48:24+00:00",
		end_time: None,
		name: "ACGL DIV 2: (Beatplayer4242) vs (Pixie_Mixie)"
	},
	events: [Event {
		id: 2437048760,
		detail: Detail {
			event_type: "match-created"
		},
		timestamp: "2025-05-11T12:48:24+00:00",
		user_id: Some(11690784)
	}, Event {
		id: 2437048854,
		detail: Detail {
			event_type: "player-joined"
		},
		timestamp: "2025-05-11T12:48:57+00:00",
		user_id: Some(18562553)
	}, Event {
		id: 2437048888,
		detail: Detail {
			event_type: "player-joined"
		},
		timestamp: "2025-05-11T12:49:11+00:00",
		user_id: Some(31382190)
	}],
	users: [User {
		avatar_url: "https://a.ppy.sh/11690784?1739479212.jpeg",
		country_code: "ZA",
		default_group: "default",
		id: 11690784,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: false,
		is_supporter: true,
		last_visit: None,
		pm_friends_only: false,
		profile_colour: None,
		username: "DragonMage",
		country: Country {
			code: "ZA",
			name: "South Africa"
		}
	}, User {
		avatar_url: "https://a.ppy.sh/18562553?1684249248.jpeg",
		country_code: "KE",
		default_group: "default",
		id: 18562553,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-11T12:48:40+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Beatplayer4242",
		country: Country {
			code: "KE",
			name: "Kenya"
		}
	}, User {
		avatar_url: "https://a.ppy.sh/31382190?1742916843.jpeg",
		country_code: "TZ",
		default_group: "default",
		id: 31382190,
		is_active: true,
		is_bot: false,
		is_deleted: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-11T12:53:52+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "Pixie_Mixie",
		country: Country {
			code: "TZ",
			name: "United Republic of Tanzania"
		}
	}],
	first_event_id: 2437048760,
	latest_event_id: 2437048888,
	current_game_id: None
}
*/
