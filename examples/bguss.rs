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
    let scores = client.beatmaps.get_user_scores(3134382,31175842,None,Some(Mode::Catch),None).await?;
    println!("{:?}", scores);
    
    
    Ok(())
}

/*
ReqwestBeatmaps get_User_Scores
Scores {
	scores: [Score {
		accuracy: 0.9883720930232558,
		best_id: 219374634,
		created_at: "2024-12-29T13:27:41Z",
		id: 219374634,
		max_combo: 230,
		mode: Catch,
		mode_int: 2,
		mods: [],
		passed: true,
		perfect: false,
		pp: 36.7791,
		rank: S,
		replay: true,
		score: 1557194,
		statistics: Statistics {
			count_100: 4,
			count_300: 320,
			count_50: 271,
			count_geki: None,
			count_katu: 6,
			count_miss: 1
		},
		type: "score_best_fruits",
		user_id: 31175842,
		current_user_attributes: CurrentUserAttributes {
			pin: None
		},
		beatmap: None,
		user: None
	}, Score {
		accuracy: 0.9584717607973422,
		best_id: 219374527,
		created_at: "2024-12-29T13:17:38Z",
		id: 219374527,
		max_combo: 131,
		mode: Catch,
		mode_int: 2,
		mods: [],
		passed: true,
		perfect: false,
		pp: 15.0524,
		rank: A,
		replay: true,
		score: 751128,
		statistics: Statistics {
			count_100: 4,
			count_300: 311,
			count_50: 262,
			count_geki: None,
			count_katu: 15,
			count_miss: 10
		},
		type: "score_best_fruits",
		user_id: 31175842,
		current_user_attributes: CurrentUserAttributes {
			pin: None
		},
		beatmap: None,
		user: None
	}]
}
*/