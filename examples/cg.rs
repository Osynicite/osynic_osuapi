// Get comments
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::comments::IComments;
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
    let comment = client
        .comments
        .get_comment(
            "3612366".to_string(),
        )
        .await?;
    println!("{:?}", comment);
    Ok(())
}


/*
ReqwestComments get_comment
CommentBundle {
	commentable_meta: Some([CommentableMeta {
		current_user_attributes: Some(CurrentUserAttributes {
			can_new_comment_reason: Some("Please sign in to proceed.")
		}),
		id: Some(2282646),
		owner_id: Some(14906730),
		owner_title: Some("MAPPER"),
		title: "Machine Love",
		object_type: Some("beatmapset"),
		url: Some("https://osu.ppy.sh/beatmapsets/2282646")
	}, CommentableMeta {
		current_user_attributes: None,
		id: None,
		owner_id: None,
		owner_title: None,
		title: "Deleted Item",
		object_type: None,
		url: None
	}]),
	comments: [Comment {
		commentable_id: 2282646,
		commentable_type: "beatmapset",
		created_at: "2025-05-12T12:41:48Z",
		deleted_at: None,
		edited_at: None,
		edited_by_id: None,
		id: 3612366,
		legacy_name: None,
		message: Some("i wanted to thank you"),
		message_html: Some("<div class='osu-md osu-md--comment'><p class=\"osu-md__paragraph\">i wanted to thank you</p>\n</div>"),
		parent_id: Some(3607999),
		pinned: false,
		replies_count: 0,
		updated_at: "2025-05-12T12:41:48Z",
		user_id: 20599345,
		votes_count: 0
	}],
	cursor: Some(Cursor {
		created_at: Some("2025-05-12T12:41:48.000000Z"),
		id: Some(3612366)
	}),
	has_more: false,
	has_more_id: Some(3612366),
	included_comments: [Comment {
		commentable_id: 2282646,
		commentable_type: "beatmapset",
		created_at: "2025-05-09T03:35:10Z",
		deleted_at: None,
		edited_at: None,
		edited_by_id: None,
		id: 3607999,
		legacy_name: None,
		message: Some("I have a song to share"),
		message_html: Some("<div class='osu-md osu-md--comment'><p class=\"osu-md__paragraph\">I have a song to share</p>\n</div>"),
		parent_id: Some(3607513),
		pinned: false,
		replies_count: 1,
		updated_at: "2025-05-12T12:41:48Z",
		user_id: 14906730,
		votes_count: 1
	}],
	pinned_comments: Some([]),
	sort: "new",
	top_level_count: None,
	total: None,
	user_follow: false,
	user_votes: [],
	users: [User {
		avatar_url: "https://a.ppy.sh/14906730?1735266969.jpeg",
		country_code: "PH",
		default_group: "default",
		id: 14906730,
		is_active: true,
		is_bot: false,
		is_online: false,
		is_supporter: false,
		last_visit: Some("2025-05-12T09:13:56+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "HydeManRs"
	}, User {
		avatar_url: "https://a.ppy.sh/20599345?1623687204.jpeg",
		country_code: "ID",
		default_group: "default",
		id: 20599345,
		is_active: true,
		is_bot: false,
		is_online: true,
		is_supporter: false,
		last_visit: Some("2025-05-12T12:47:57+00:00"),
		pm_friends_only: false,
		profile_colour: None,
		username: "KamiAyame"
	}]
}
*/