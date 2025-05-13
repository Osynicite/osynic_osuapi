// Get topic and posts
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

    let topic_and_posts = client
        .forum
        .get_topic_and_posts(2070055, None, None, None, None, None)
        .await?;
    println!("{:?}", topic_and_posts);
    Ok(())
}

/*
ReqwestForum get_topic_and_posts
GetTopicAndPostsResponse {
    cursor_string: "eyJpZCI6OTg3NzgzOH0",
    posts: [ForumPost {
        created_at: "2025-04-22T19:53:27+00:00",
        deleted_at: None,
        edited_at: Some("2025-05-10T16:00:38+00:00"),
        edited_by_id: Some(13317401),
        forum_id: 10,
        id: 9877838,
        topic_id: 2070055,
        user_id: 13317401,
        body: Some(Body {
            html: "<div class='bbcode'><span style=\"font-size:85%;\">This beatmap was submitted using in-game submission on Monday, May 12, 2025 at 2:16:31 PM</span><br /><br /><strong>Artist:</strong> Various Artists<br /><strong>Title:</strong> Songs Compilation<br /><strong>Tags:</strong> shiina taki KioSHiIgg offline mafia -xam- xam therealxam Azukimi cerule<br /><strong>BPM:</strong> 260<br /><strong>Filesize:</strong> 9748kb<br /><strong>Play Time:</strong> 05:45<br /><strong>Difficulties Available:</strong><br /><ol class=\"unordered\"><li><a rel=\"nofollow\" href=\"https://osu.ppy.sh/web/maps/Various%20Artists%20-%20Songs%20Compilation%20(-%2039)%20%5BCollab%20Extra%5D.osu\">Collab Extra</a> (7.29 stars, 971 notes)</li></ol><span style=\"font-size:150%;\"><strong>Download: <a rel=\"nofollow\" href=\"https://osu.ppy.sh/d/2359303\">Various Artists - Songs Compilation</a></strong></span><br /><strong>Information:</strong> <a rel=\"nofollow\" href=\"https://osu.ppy.sh/beatmapsets/2359303\">Scores/Beatmap Listing</a><br />---------------<br />Sentou! Battle Tower -&gt; Tyllis39<br />ChuChu Lovely -&gt; Xam<br />Title Screen -&gt; Synaelle<br />Rage of Dust -&gt; KioSHiIgg<br />Polar 240 -&gt; Shiina Taki<br />A Little Bit -&gt;<br />Chug Jug -&gt;<br />Happy! Lucky! Dochy! -&gt;<br />When It Falls -&gt; Tyllis39<br />erase u -&gt; Azukimi &amp; cerule<br />Brazil -&gt; Tyllis39<br />Decided -&gt; KioSHiIgg</div>",
            raw: "[size=85]This beatmap was submitted using in-game submission on Monday, May 12, 2025 at 2:16:31 PM[/size]\n\n[b]Artist:[/b] Various Artists\n[b]Title:[/b] Songs Compilation\n[b]Tags:[/b] shiina taki KioSHiIgg offline mafia -xam- xam therealxam Azukimi cerule\n[b]BPM:[/b] 260\n[b]Filesize:[/b] 9748kb\n[b]Play Time:[/b] 05:45\n[b]Difficulties Available:[/b]\n[list][*][url=https://osu.ppy.sh/web/maps/Various%20Artists%20-%20Songs%20Compilation%20(-%2039)%20%5bCollab%20Extra%5d.osu]Collab Extra[/url] (7.29 stars, 971 notes)\n[/list]\n\n[size=150][b]Download: [url=https://osu.ppy.sh/d/2359303]Various Artists - Songs Compilation[/url][/b][/size]\n[b]Information:[/b] [url=https://osu.ppy.sh/beatmapsets/2359303]Scores/Beatmap Listing[/url]\n---------------\nSentou! Battle Tower -> Tyllis39\nChuChu Lovely -> Xam\nTitle Screen -> Synaelle\nRage of Dust -> KioSHiIgg\nPolar 240 -> Shiina Taki\nA Little Bit ->\nChug Jug ->\nHappy! Lucky! Dochy! ->\nWhen It Falls -> Tyllis39\nerase u -> Azukimi & cerule\nBrazil -> Tyllis39\nDecided -> KioSHiIgg"
        })
    }],
    search: Search {
        limit: 50,
        sort: "id_asc"
    },
    topic: ForumTopic {
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
    }
}
*/
