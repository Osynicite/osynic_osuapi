// Get matches listing
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
    let matches = client
        .matches
        .get_matches_listing(None,None,None)
        .await?;
    println!("{:?}", matches);
    Ok(())
}


/*
ReqwestMatches get_matches_listing
GetMatchesListingResponse {
	matches: [Match {
		id: 118061894,
		start_time: "2025-05-11T12:48:24+00:00",
		end_time: None,
		name: "ACGL DIV 2: (Beatplayer4242) vs (Pixie_Mixie)"
	}, Match {
		id: 118061893,
		start_time: "2025-05-11T12:48:22+00:00",
		end_time: None,
		name: "1234"
	}, Match {
		id: 118061892,
		start_time: "2025-05-11T12:48:13+00:00",
		end_time: None,
		name: "Camp Fire jilid 1"
	}, Match {
		id: 118061891,
		start_time: "2025-05-11T12:48:08+00:00",
		end_time: None,
		name: "orange"
	}, Match {
		id: 118061890,
		start_time: "2025-05-11T12:47:56+00:00",
		end_time: None,
		name: "inkdasut65's game"
	}, Match {
		id: 118061889,
		start_time: "2025-05-11T12:47:53+00:00",
		end_time: None,
		name: "Msvex's game"
	}, Match {
		id: 118061888,
		start_time: "2025-05-11T12:47:46+00:00",
		end_time: None,
		name: "-Try Again-'s game"
	}, Match {
		id: 118061887,
		start_time: "2025-05-11T12:47:45+00:00",
		end_time: None,
		name: "JOJOdngekhoa's game"
	}, Match {
		id: 118061886,
		start_time: "2025-05-11T12:47:25+00:00",
		end_time: None,
		name: "Zvenx's game"
	}, Match {
		id: 118061885,
		start_time: "2025-05-11T12:47:22+00:00",
		end_time: None,
		name: "Menyus's game"
	}, Match {
		id: 118061884,
		start_time: "2025-05-11T12:47:21+00:00",
		end_time: None,
		name: "SLT: (:fumocirno:) vs (scheiße wizard)"
	}, Match {
		id: 118061883,
		start_time: "2025-05-11T12:47:09+00:00",
		end_time: Some("2025-05-11T12:47:21+00:00"),
		name: "ETX: (Destoni) vs (GwGhofur)"
	}, Match {
		id: 118061882,
		start_time: "2025-05-11T12:47:07+00:00",
		end_time: None,
		name: "CDTS 3 Qualifiers: Lobby 4"
	}, Match {
		id: 118061881,
		start_time: "2025-05-11T12:47:04+00:00",
		end_time: None,
		name: "dungeonmaster22's game"
	}, Match {
		id: 118061880,
		start_time: "2025-05-11T12:47:04+00:00",
		end_time: None,
		name: "CATS3: FlanRemiSariel vs. Katsumi"
	}, Match {
		id: 118061879,
		start_time: "2025-05-11T12:47:02+00:00",
		end_time: None,
		name: "IZT2: (dingle prodigy) vs (Pokemak)"
	}, Match {
		id: 118061878,
		start_time: "2025-05-11T12:46:56+00:00",
		end_time: None,
		name: "Osu-Player's game"
	}, Match {
		id: 118061877,
		start_time: "2025-05-11T12:46:54+00:00",
		end_time: Some("2025-05-11T12:47:51+00:00"),
		name: "MegaND1213's game"
	}, Match {
		id: 118061876,
		start_time: "2025-05-11T12:46:37+00:00",
		end_time: None,
		name: "touchable"
	}, Match {
		id: 118061875,
		start_time: "2025-05-11T12:46:29+00:00",
		end_time: None,
		name: "uomare's game"
	}, Match {
		id: 118061874,
		start_time: "2025-05-11T12:46:29+00:00",
		end_time: Some("2025-05-11T12:46:44+00:00"),
		name: "matikota5656's game"
	}, Match {
		id: 118061873,
		start_time: "2025-05-11T12:46:25+00:00",
		end_time: None,
		name: "aa"
	}, Match {
		id: 118061872,
		start_time: "2025-05-11T12:46:23+00:00",
		end_time: None,
		name: "parola123"
	}, Match {
		id: 118061871,
		start_time: "2025-05-11T12:46:18+00:00",
		end_time: None,
		name: "zer01101111's game"
	}, Match {
		id: 118061870,
		start_time: "2025-05-11T12:46:17+00:00",
		end_time: None,
		name: "Sad77777's game"
	}, Match {
		id: 118061869,
		start_time: "2025-05-11T12:46:16+00:00",
		end_time: None,
		name: "IS TIME TO GO TO BED"
	}, Match {
		id: 118061868,
		start_time: "2025-05-11T12:46:10+00:00",
		end_time: None,
		name: "GBC 2025 Spring: S4 Sun 13"
	}, Match {
		id: 118061867,
		start_time: "2025-05-11T12:46:06+00:00",
		end_time: None,
		name: "[Haku]'s game"
	}, Match {
		id: 118061866,
		start_time: "2025-05-11T12:46:06+00:00",
		end_time: Some("2025-05-11T12:46:16+00:00"),
		name: "zaidankun's game"
	}, Match {
		id: 118061865,
		start_time: "2025-05-11T12:46:06+00:00",
		end_time: None,
		name: "sosiski"
	}, Match {
		id: 118061864,
		start_time: "2025-05-11T12:46:04+00:00",
		end_time: None,
		name: "klamzi's game"
	}, Match {
		id: 118061863,
		start_time: "2025-05-11T12:46:01+00:00",
		end_time: None,
		name: "LEIZ"
	}, Match {
		id: 118061862,
		start_time: "2025-05-11T12:46:00+00:00",
		end_time: None,
		name: "cutie potato's game"
	}, Match {
		id: 118061861,
		start_time: "2025-05-11T12:45:58+00:00",
		end_time: None,
		name: "miasgodlol's game"
	}, Match {
		id: 118061860,
		start_time: "2025-05-11T12:45:58+00:00",
		end_time: None,
		name: "Remastered's game"
	}, Match {
		id: 118061859,
		start_time: "2025-05-11T12:45:49+00:00",
		end_time: Some("2025-05-11T12:46:53+00:00"),
		name: "dungeonmaster22's game"
	}, Match {
		id: 118061858,
		start_time: "2025-05-11T12:45:38+00:00",
		end_time: None,
		name: "bucet"
	}, Match {
		id: 118061857,
		start_time: "2025-05-11T12:45:25+00:00",
		end_time: None,
		name: "whenpapu"
	}, Match {
		id: 118061856,
		start_time: "2025-05-11T12:45:24+00:00",
		end_time: None,
		name: "sunflower110620's game"
	}, Match {
		id: 118061855,
		start_time: "2025-05-11T12:45:18+00:00",
		end_time: None,
		name: "happy hamster"
	}, Match {
		id: 118061854,
		start_time: "2025-05-11T12:45:10+00:00",
		end_time: Some("2025-05-11T12:45:28+00:00"),
		name: "yue0513's game"
	}, Match {
		id: 118061853,
		start_time: "2025-05-11T12:45:08+00:00",
		end_time: None,
		name: "parrk"
	}, Match {
		id: 118061852,
		start_time: "2025-05-11T12:45:07+00:00",
		end_time: None,
		name: "-Sera's game"
	}, Match {
		id: 118061851,
		start_time: "2025-05-11T12:44:58+00:00",
		end_time: None,
		name: "Maumanana's game"
	}, Match {
		id: 118061850,
		start_time: "2025-05-11T12:44:49+00:00",
		end_time: None,
		name: "ETX Teams: (Lyea23) vs (Chersen)"
	}, Match {
		id: 118061848,
		start_time: "2025-05-11T12:44:40+00:00",
		end_time: Some("2025-05-11T12:44:58+00:00"),
		name: "test"
	}, Match {
		id: 118061847,
		start_time: "2025-05-11T12:44:17+00:00",
		end_time: None,
		name: "osu! India Summer Tournament 2025: (Qualifiers) vs (Match T1-X5)"
	}, Match {
		id: 118061846,
		start_time: "2025-05-11T12:44:06+00:00",
		end_time: None,
		name: "sebatian_224's game"
	}, Match {
		id: 118061845,
		start_time: "2025-05-11T12:44:00+00:00",
		end_time: None,
		name: "Innocentguy_z's game"
	}, Match {
		id: 118061844,
		start_time: "2025-05-11T12:43:59+00:00",
		end_time: Some("2025-05-11T12:45:13+00:00"),
		name: "rostikfacekids's game"
	}],
	params: Params {
		limit: 50,
		sort: "id_desc"
	},
	cursor: Cursor {
		match_id: 118061844
	},
	cursor_string: "eyJtYXRjaF9pZCI6MTE4MDYxODQ0fQ"
}
*/