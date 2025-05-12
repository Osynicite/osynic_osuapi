// Get topics listing
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::forum::IForum;
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
    let topic_listing = client
        .forum
        .get_topics_listing(
            None,
            None,
            Some(5),
            None,
        )
        .await?;
    println!("{:?}", topic_listing);
    Ok(())
}


/*
ReqwestForum get_topics_listing
TopicListing {
	topics: [Topic {
		created_at: "2025-04-22T19:53:27+00:00",
		deleted_at: None,
		first_post_id: 9877838,
		forum_id: 10,
		id: 2070055,
		is_locked: true,
		last_post_id: 9877838,
		poll: None,
		post_count: 1,
		title: "Various Artists - Songs Compilation",
		topic_type: "normal",
		updated_at: "2025-05-12T13:16:50+00:00",
		user_id: 13317401
	}, Topic {
		created_at: "2022-04-22T14:31:00+00:00",
		deleted_at: None,
		first_post_id: 8608095,
		forum_id: 6,
		id: 1563225,
		is_locked: true,
		last_post_id: 8608095,
		poll: None,
		post_count: 1,
		title: "Memme - Avalanche [OsuMania]",
		topic_type: "normal",
		updated_at: "2025-05-12T13:16:02+00:00",
		user_id: 7222699
	}, Topic {
		created_at: "2024-03-07T17:25:20+00:00",
		deleted_at: None,
		first_post_id: 9462815,
		forum_id: 6,
		id: 1893760,
		is_locked: true,
		last_post_id: 9462815,
		poll: None,
		post_count: 1,
		title: "Roselia - THRONE OF ROSE",
		topic_type: "normal",
		updated_at: "2025-05-12T13:15:49+00:00",
		user_id: 9863870
	}, Topic {
		created_at: "2025-05-12T13:14:20+00:00",
		deleted_at: None,
		first_post_id: 9896906,
		forum_id: 10,
		id: 2078275,
		is_locked: true,
		last_post_id: 9896906,
		poll: None,
		post_count: 1,
		title: "Rin - Daishibyo set 08 ~ Furuki Yuanxian",
		topic_type: "normal",
		updated_at: "2025-05-12T13:15:40+00:00",
		user_id: 16465883
	}, Topic {
		created_at: "2023-11-06T12:58:23+00:00",
		deleted_at: None,
		first_post_id: 9325759,
		forum_id: 6,
		id: 1842038,
		is_locked: true,
		last_post_id: 9325759,
		poll: None,
		post_count: 1,
		title: "AISHA - The Disaster of Passion",
		topic_type: "normal",
		updated_at: "2025-05-12T13:13:51+00:00",
		user_id: 7936305
	}],
	cursor_string: "eyJ0b3BpY19sYXN0X3Bvc3RfdGltZSI6IjIwMjUtMDUtMTJUMTM6MTM6NTEuMDAwMDAwWiJ9"
}
*/