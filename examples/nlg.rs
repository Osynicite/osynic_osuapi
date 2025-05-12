// Get news listing
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::news::INews;
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
    let news_listing = client.news.get_news_listing(None, None, None).await?;
    println!("{:?}", news_listing);
    Ok(())
}

/*
ReqwestNews get_news_listing
GetNewsListingResponse {
    news_posts: [NewsPost {
        id: 1703,
        author: "Phreel & Hoaq",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-11-resurrection-cup-2025-registrations-now-open.md",
        first_image: "https://i.ppy.sh/12a0825b379d5f26695668da3e0e208e88f4d12a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f524553432f323032352f696d672f62616e6e65722e6a7067",
        published_at: "2025-05-11T00:00:00+00:00",
        updated_at: "2025-05-11T00:00:00+00:00",
        slug: "2025-05-11-resurrection-cup-2025-registrations-now-open",
        title: "Resurrection Cup 2025: Registrations Now Open!",
        preview: Some("The renaissance is upon us with Resurrection Cup returning for a 2025 iteration — with more bangers, more customs and a large focus on competitivity!")
    }, NewsPost {
        id: 1702,
        author: "the GTS Admin Team",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-09-squad-global-taiko-showdown-2025-registrations-now-open.md",
        first_image: "https://i.ppy.sh/b6d2cdcddd0b59c6e727d9d9a1febc66e0a40de6/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30352d30392d73717561642d676c6f62616c2d7461696b6f2d73686f77646f776e2d323032352d726567697374726174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
        published_at: "2025-05-09T22:00:00+00:00",
        updated_at: "2025-05-09T22:00:00+00:00",
        slug: "2025-05-09-squad-global-taiko-showdown-2025-registrations-now-open",
        title: "Squad Global Taiko Showdown 2025: Registrations Now Open!",
        preview: Some("We invite you to challenge the world in a 3v3 drum-smashing showdown — get ready for SGTS 2025, its third and final iteration!")
    }, NewsPost {
        id: 1701,
        author: "LeoFLT",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-09-osucatch-world-cup-2025-registrations-now-open.md",
        first_image: "https://i.ppy.sh/cbf76bfe621502a26f9b4cec0929cff39cc18686/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4357432f323032352f696d672f637763323032352d62616e6e65722e6a7067",
        published_at: "2025-05-09T15:00:00+00:00",
        updated_at: "2025-05-09T15:04:21+00:00",
        slug: "2025-05-09-osucatch-world-cup-2025-registrations-now-open",
        title: "osu!catch World Cup 2025: Registrations Now Open!",
        preview: Some("Calling out all fruit catchers for the 2025 osu!catch World Cup!")
    }, NewsPost {
        id: 1700,
        author: "Kasumi-sama, Joogs, --Glitchy--",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-06-expert-global-taiko-showdown-2025-concludes.md",
        first_image: "https://i.ppy.sh/3372b4cd7dd6552bd0ac488f7ccd493999eda83f/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d30352d656774732d323032352d726567697374726174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
        published_at: "2025-05-06T20:00:00+00:00",
        updated_at: "2025-05-06T20:00:00+00:00",
        slug: "2025-05-06-expert-global-taiko-showdown-2025-concludes",
        title: "Expert Global Taiko Showdown 2025 Concludes",
        preview: Some("After two months of some of the highest levels of tournament matches osu!taiko has ever seen, the final chapter of EGTS has come to a conclusion! Please join us to see what went down in the most ambitious osu!taiko tournament yet.")
    }, NewsPost {
        id: 1699,
        author: "pishifat",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-04-featured-artist-track-updates-krimek.md",
        first_image: "https://i.ppy.sh/ca2c414fd7cea6dc83ce2d13797201612b75f34b/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3337312f6865616465722e6a7067",
        published_at: "2025-05-04T21:00:00+00:00",
        updated_at: "2025-05-04T21:10:32+00:00",
        slug: "2025-05-04-featured-artist-track-updates-krimek",
        title: "Featured Artist Track Updates: Krimek",
        preview: Some("From a regular in osu!'s music production scene to one of osu!'s Featured Artists, Krimek returns with a new batch of tracks!")
    }, NewsPost {
        id: 1698,
        author: "Yasuho, Sakura006, Walavouchey, mangomizer, Nurend, overdahedge2015",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-04-osutaiko-world-cup-concludes.md",
        first_image: "https://i.ppy.sh/36e860a55084f80d4aa9bab9cc0babed4be1f67a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30352d30342d6f73757461696b6f2d776f726c642d6375702d636f6e636c756465732f62616e6e65722e6a7067",
        published_at: "2025-05-04T14:00:00+00:00",
        updated_at: "2025-05-04T14:00:00+00:00",
        slug: "2025-05-04-osutaiko-world-cup-concludes",
        title: "osu!taiko World Cup 2025 Concludes",
        preview: Some("The champion has ascended to the throne once again!")
    }, NewsPost {
        id: 1697,
        author: "LeoFLT",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-02-the-lazer-grand-arena-returns.md",
        first_image: "https://i.ppy.sh/dd6bcb180df8d960e896c66b061799dcb16d4b17/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4c47412f323032352f696d672f6c6761323032352d62616e6e65722e6a7067",
        published_at: "2025-05-02T15:00:00+00:00",
        updated_at: "2025-05-04T20:22:18+00:00",
        slug: "2025-05-02-the-lazer-grand-arena-returns",
        title: "The Lazer Grand Arena Returns",
        preview: Some("The Lazer Grand Arena is back for its 2025 edition, with a new format and streamlined entry!")
    }, NewsPost {
        id: 1696,
        author: "MegaMix & -Isla-",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-30-the-followpoint-spectator-the-osu-catch-mapping-pioneer.md",
        first_image: "https://i.ppy.sh/0d97365575ae736525324bbc102999d92695bdce/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d33302d7468652d666f6c6c6f77706f696e742d737065637461746f722d7468652d6f73752d63617463682d6d617070696e672d70696f6e6565722f62616e6e65722e6a7067",
        published_at: "2025-04-30T09:00:00+00:00",
        updated_at: "2025-05-07T17:27:19+00:00",
        slug: "2025-04-30-the-followpoint-spectator-the-osu-catch-mapping-pioneer",
        title: "The Followpoint: Spectator, the osu!catch Mapping Pioneer",
        preview: Some("Have you ever looked at osu! interviews and thought to yourself: &quot;Hey, aren't there too few mapper interviews?&quot; Well, today is your lucky day, as we are interviewing Spectator, one of the most iconic osu!catch mappers out there!")
    }, NewsPost {
        id: 1694,
        author: "0x84f",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-29-kel-lan-tournament-3-recap.md",
        first_image: "https://i.ppy.sh/db124cbd26ff56928887b19b688874570d12286a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032342d31312d31382d6b656c2d6c616e2d746f75726e616d656e742d332f62616e6e65722e6a7067",
        published_at: "2025-04-29T10:00:00+00:00",
        updated_at: "2025-04-29T10:00:00+00:00",
        slug: "2025-04-29-kel-lan-tournament-3-recap",
        title: "KEL LAN Tournament 3 Recap",
        preview: Some("The third edition of the KEL tournament has concluded! Join us for the tournament recap, with coverage done directly from the LAN!")
    }, NewsPost {
        id: 1695,
        author: "pishifat",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-28-new-featured-artist-1zm8.md",
        first_image: "https://i.ppy.sh/d3e58f6396bc7bac510b3b8fc937b73f7f3e3016/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3437342f6865616465722e6a7067",
        published_at: "2025-04-28T19:00:00+00:00",
        updated_at: "2025-04-29T08:20:44+00:00",
        slug: "2025-04-28-new-featured-artist-1zm8",
        title: "New Featured Artist: 1zm8",
        preview: Some("Bow before 1zm8, the latest addition to our Featured Artists!")
    }, NewsPost {
        id: 1693,
        author: "Yasuho & overdahedge2015",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-25-osutaiko-world-cup-finals-recap.md",
        first_image: "https://i.ppy.sh/c92a29a118e99063945b870ac9e27116f5afa403/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d32352d6f73757461696b6f2d776f726c642d6375702d66696e616c732d72656361702f62616e6e65722e6a7067",
        published_at: "2025-04-25T12:30:00+00:00",
        updated_at: "2025-04-25T13:54:15+00:00",
        slug: "2025-04-25-osutaiko-world-cup-finals-recap",
        title: "osu!taiko World Cup 2025: Finals Recap",
        preview: Some("The last remaining teams approached the summit. One final step to the ultimate battle!")
    }, NewsPost {
        id: 1692,
        author: "aceticke",
        edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-20-project-loved-april-2025.md",
        first_image: "https://i.ppy.sh/f5982d2fe7d489faac36230fc235617f3bd1f6b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70726f6a6563742d6c6f7665642d322e6a7067",
        published_at: "2025-04-20T12:15:00+00:00",
        updated_at: "2025-04-20T16:09:19+00:00",
        slug: "2025-04-20-project-loved-april-2025",
        title: "Project Loved: April 2025",
        preview: Some("No fools here, welcome to Project Loved April!")
    }],
    news_sidebar: NewsSidebar {
        current_year: 2025,
        news_posts: [NewsPost {
            id: 1703,
            author: "Phreel & Hoaq",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-11-resurrection-cup-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/12a0825b379d5f26695668da3e0e208e88f4d12a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f524553432f323032352f696d672f62616e6e65722e6a7067",
            published_at: "2025-05-11T00:00:00+00:00",
            updated_at: "2025-05-11T00:00:00+00:00",
            slug: "2025-05-11-resurrection-cup-2025-registrations-now-open",
            title: "Resurrection Cup 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1702,
            author: "the GTS Admin Team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-09-squad-global-taiko-showdown-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/b6d2cdcddd0b59c6e727d9d9a1febc66e0a40de6/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30352d30392d73717561642d676c6f62616c2d7461696b6f2d73686f77646f776e2d323032352d726567697374726174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
            published_at: "2025-05-09T22:00:00+00:00",
            updated_at: "2025-05-09T22:00:00+00:00",
            slug: "2025-05-09-squad-global-taiko-showdown-2025-registrations-now-open",
            title: "Squad Global Taiko Showdown 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1701,
            author: "LeoFLT",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-09-osucatch-world-cup-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/cbf76bfe621502a26f9b4cec0929cff39cc18686/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4357432f323032352f696d672f637763323032352d62616e6e65722e6a7067",
            published_at: "2025-05-09T15:00:00+00:00",
            updated_at: "2025-05-09T15:04:21+00:00",
            slug: "2025-05-09-osucatch-world-cup-2025-registrations-now-open",
            title: "osu!catch World Cup 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1700,
            author: "Kasumi-sama, Joogs, --Glitchy--",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-06-expert-global-taiko-showdown-2025-concludes.md",
            first_image: "https://i.ppy.sh/3372b4cd7dd6552bd0ac488f7ccd493999eda83f/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d30352d656774732d323032352d726567697374726174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
            published_at: "2025-05-06T20:00:00+00:00",
            updated_at: "2025-05-06T20:00:00+00:00",
            slug: "2025-05-06-expert-global-taiko-showdown-2025-concludes",
            title: "Expert Global Taiko Showdown 2025 Concludes",
            preview: None
        }, NewsPost {
            id: 1699,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-04-featured-artist-track-updates-krimek.md",
            first_image: "https://i.ppy.sh/ca2c414fd7cea6dc83ce2d13797201612b75f34b/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3337312f6865616465722e6a7067",
            published_at: "2025-05-04T21:00:00+00:00",
            updated_at: "2025-05-04T21:10:32+00:00",
            slug: "2025-05-04-featured-artist-track-updates-krimek",
            title: "Featured Artist Track Updates: Krimek",
            preview: None
        }, NewsPost {
            id: 1698,
            author: "Yasuho, Sakura006, Walavouchey, mangomizer, Nurend, overdahedge2015",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-04-osutaiko-world-cup-concludes.md",
            first_image: "https://i.ppy.sh/36e860a55084f80d4aa9bab9cc0babed4be1f67a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30352d30342d6f73757461696b6f2d776f726c642d6375702d636f6e636c756465732f62616e6e65722e6a7067",
            published_at: "2025-05-04T14:00:00+00:00",
            updated_at: "2025-05-04T14:00:00+00:00",
            slug: "2025-05-04-osutaiko-world-cup-concludes",
            title: "osu!taiko World Cup 2025 Concludes",
            preview: None
        }, NewsPost {
            id: 1697,
            author: "LeoFLT",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-05-02-the-lazer-grand-arena-returns.md",
            first_image: "https://i.ppy.sh/dd6bcb180df8d960e896c66b061799dcb16d4b17/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4c47412f323032352f696d672f6c6761323032352d62616e6e65722e6a7067",
            published_at: "2025-05-02T15:00:00+00:00",
            updated_at: "2025-05-04T20:22:18+00:00",
            slug: "2025-05-02-the-lazer-grand-arena-returns",
            title: "The Lazer Grand Arena Returns",
            preview: None
        }, NewsPost {
            id: 1696,
            author: "MegaMix & -Isla-",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-30-the-followpoint-spectator-the-osu-catch-mapping-pioneer.md",
            first_image: "https://i.ppy.sh/0d97365575ae736525324bbc102999d92695bdce/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d33302d7468652d666f6c6c6f77706f696e742d737065637461746f722d7468652d6f73752d63617463682d6d617070696e672d70696f6e6565722f62616e6e65722e6a7067",
            published_at: "2025-04-30T09:00:00+00:00",
            updated_at: "2025-05-07T17:27:19+00:00",
            slug: "2025-04-30-the-followpoint-spectator-the-osu-catch-mapping-pioneer",
            title: "The Followpoint: Spectator, the osu!catch Mapping Pioneer",
            preview: None
        }, NewsPost {
            id: 1694,
            author: "0x84f",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-29-kel-lan-tournament-3-recap.md",
            first_image: "https://i.ppy.sh/db124cbd26ff56928887b19b688874570d12286a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032342d31312d31382d6b656c2d6c616e2d746f75726e616d656e742d332f62616e6e65722e6a7067",
            published_at: "2025-04-29T10:00:00+00:00",
            updated_at: "2025-04-29T10:00:00+00:00",
            slug: "2025-04-29-kel-lan-tournament-3-recap",
            title: "KEL LAN Tournament 3 Recap",
            preview: None
        }, NewsPost {
            id: 1695,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-28-new-featured-artist-1zm8.md",
            first_image: "https://i.ppy.sh/d3e58f6396bc7bac510b3b8fc937b73f7f3e3016/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3437342f6865616465722e6a7067",
            published_at: "2025-04-28T19:00:00+00:00",
            updated_at: "2025-04-29T08:20:44+00:00",
            slug: "2025-04-28-new-featured-artist-1zm8",
            title: "New Featured Artist: 1zm8",
            preview: None
        }, NewsPost {
            id: 1693,
            author: "Yasuho & overdahedge2015",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-25-osutaiko-world-cup-finals-recap.md",
            first_image: "https://i.ppy.sh/c92a29a118e99063945b870ac9e27116f5afa403/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d32352d6f73757461696b6f2d776f726c642d6375702d66696e616c732d72656361702f62616e6e65722e6a7067",
            published_at: "2025-04-25T12:30:00+00:00",
            updated_at: "2025-04-25T13:54:15+00:00",
            slug: "2025-04-25-osutaiko-world-cup-finals-recap",
            title: "osu!taiko World Cup 2025: Finals Recap",
            preview: None
        }, NewsPost {
            id: 1692,
            author: "aceticke",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-20-project-loved-april-2025.md",
            first_image: "https://i.ppy.sh/f5982d2fe7d489faac36230fc235617f3bd1f6b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70726f6a6563742d6c6f7665642d322e6a7067",
            published_at: "2025-04-20T12:15:00+00:00",
            updated_at: "2025-04-20T16:09:19+00:00",
            slug: "2025-04-20-project-loved-april-2025",
            title: "Project Loved: April 2025",
            preview: None
        }, NewsPost {
            id: 1691,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-19-new-featured-artist-naikou.md",
            first_image: "https://i.ppy.sh/cec76cdb6921168e4badce61748ad6569088568e/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3437312f6865616465722e6a7067",
            published_at: "2025-04-19T20:00:00+00:00",
            updated_at: "2025-04-21T02:04:06+00:00",
            slug: "2025-04-19-new-featured-artist-naikou",
            title: "New Featured Artist: Naikou",
            preview: None
        }, NewsPost {
            id: 1690,
            author: "Yasuho & Teezel",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-18-osutaiko-world-cup-semifinals-recap.md",
            first_image: "https://i.ppy.sh/6a8a8dd9b47ff5db9907b9b763a091228ffcd684/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d31382d6f73757461696b6f2d776f726c642d6375702d73656d6966696e616c732d72656361702f62616e6e65722e6a7067",
            published_at: "2025-04-18T20:15:00+00:00",
            updated_at: "2025-04-18T20:15:00+00:00",
            slug: "2025-04-18-osutaiko-world-cup-semifinals-recap",
            title: "osu!taiko World Cup 2025: Semifinals Recap",
            preview: None
        }, NewsPost {
            id: 1689,
            author: "MegaMix",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-15-the-followpoint-azer-the-tournament-titan.md",
            first_image: "https://i.ppy.sh/42a73d12154ba589a652a67fc62cddb86921dd6f/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d31352d7468652d666f6c6c6f77706f696e742d617a65722d7468652d746f75726e616d656e742d746974616e2f62616e6e65722e6a7067",
            published_at: "2025-04-15T17:00:00+00:00",
            updated_at: "2025-04-15T17:00:00+00:00",
            slug: "2025-04-15-the-followpoint-azer-the-tournament-titan",
            title: "The Followpoint: Azer, the Tournament Titan",
            preview: None
        }, NewsPost {
            id: 1688,
            author: "Locus Team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-14-locus-2025-update-1.md",
            first_image: "https://i.ppy.sh/2ad7b99ded7eff3483731d0dcc3405bf4ac84540/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f436f6e74657374732f4c6f6375732f323032352f696d672f62616e6e65722e6a7067",
            published_at: "2025-04-14T11:00:00+00:00",
            updated_at: "2025-04-14T11:04:40+00:00",
            slug: "2025-04-14-locus-2025-update-1",
            title: "Locus 2025 Update #1",
            preview: None
        }, NewsPost {
            id: 1687,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-13-new-featured-artist-terminal-11.md",
            first_image: "https://i.ppy.sh/ceab092df681ad30152b2b7abe9b61755f9a788b/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3437302f6865616465722e6a7067",
            published_at: "2025-04-13T15:45:00+00:00",
            updated_at: "2025-04-13T15:46:37+00:00",
            slug: "2025-04-13-new-featured-artist-terminal-11",
            title: "New Featured Artist: Terminal 11",
            preview: None
        }, NewsPost {
            id: 1686,
            author: "Yasuho, Hivie, Raphalge, Teezel",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-11-osutaiko-world-cup-quarterfinals-recap.md",
            first_image: "https://i.ppy.sh/ed283b5640dd68f297bf29218e73e68b8dd387a7/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d31312d6f73757461696b6f2d776f726c642d6375702d7175617274657266696e616c732d72656361702f62616e6e65722e6a7067",
            published_at: "2025-04-11T12:00:00+00:00",
            updated_at: "2025-04-11T12:00:00+00:00",
            slug: "2025-04-11-osutaiko-world-cup-quarterfinals-recap",
            title: "osu!taiko World Cup 2025: Quarterfinals Recap",
            preview: None
        }, NewsPost {
            id: 1685,
            author: "pishifat, Morusya, RandomeLoL",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-10-monthly-beatmapping-contest-april-2025.md",
            first_image: "https://i.ppy.sh/e62524b04dc9f7eac075ee76c8c30a35a0e9ba4a/68747470733a2f2f6173736574732e7070792e73682f6d656469612f6d6f6e74686c792d626561746d617070696e672d636f6e746573742e706e67",
            published_at: "2025-04-10T21:30:00+00:00",
            updated_at: "2025-04-13T10:15:02+00:00",
            slug: "2025-04-10-monthly-beatmapping-contest-april-2025",
            title: "Monthly Beatmapping Contest: April 2025",
            preview: None
        }, NewsPost {
            id: 1684,
            author: "Venix",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-09-beatmap-spotlights-revival-spring-2025.md",
            first_image: "https://i.ppy.sh/13432c48f79111c0180d14f5f14c2e461e2dc07a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d30392d626561746d61702d73706f746c69676874732d7265766976616c2d737072696e672d323032352f62616e6e65722e6a7067",
            published_at: "2025-04-09T02:00:00+00:00",
            updated_at: "2025-04-09T04:50:29+00:00",
            slug: "2025-04-09-beatmap-spotlights-revival-spring-2025",
            title: "Beatmap Spotlights Revival: Spring 2025",
            preview: None
        }, NewsPost {
            id: 1683,
            author: "momoyo, Chaoslitz, Lince Cosmico, seros, Mafumafu",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-07-osu-beatmapping-world-championship-2025.md",
            first_image: "https://i.ppy.sh/cca5d3bc9c0faee5c5fd76ab3106098aeb9ad952/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f436f6e74657374732f6f216277632f342f696d672f62616e6e65722e6a7067",
            published_at: "2025-04-07T15:00:00+00:00",
            updated_at: "2025-05-05T13:27:59+00:00",
            slug: "2025-04-07-osu-beatmapping-world-championship-2025",
            title: "osu! Beatmapping World Championship 2025",
            preview: None
        }, NewsPost {
            id: 1682,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-05-new-featured-artist-lexycat.md",
            first_image: "https://i.ppy.sh/27c483b40d11a212f4e777d7d50bf375324f705f/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3436382f6865616465722e6a7067",
            published_at: "2025-04-05T23:00:00+00:00",
            updated_at: "2025-04-05T23:01:27+00:00",
            slug: "2025-04-05-new-featured-artist-lexycat",
            title: "New Featured Artist: lexycat",
            preview: None
        }, NewsPost {
            id: 1681,
            author: "Walavouchey, Raphalge, Nurend, Yasuho",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-03-osutaiko-world-cup-round-of-16-recap.md",
            first_image: "https://i.ppy.sh/dca664dedf31c76774d4de36fa627af0508c55f0/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d30332d6f73757461696b6f2d776f726c642d6375702d323032352d726f756e642d6f662d31362d72656361702f62616e6e65722e6a7067",
            published_at: "2025-04-03T23:30:00+00:00",
            updated_at: "2025-04-04T01:26:45+00:00",
            slug: "2025-04-03-osutaiko-world-cup-round-of-16-recap",
            title: "osu!taiko World Cup 2025: Round of 16 Recap",
            preview: None
        }, NewsPost {
            id: 1680,
            author: "shdewz, nik, this1neguy, I-Flame, Damarsh",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-02-3-digit-world-cup-2025-concludes.md",
            first_image: "https://i.ppy.sh/66cdd110b6f4f26cfce33d35592f841280225700/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d30322d332d64696769742d776f726c642d6375702d323032352d636f6e636c756465732f636f6e636c7573696f6e2d62616e6e65722e6a7067",
            published_at: "2025-04-02T12:00:00+00:00",
            updated_at: "2025-04-02T12:00:00+00:00",
            slug: "2025-04-02-3-digit-world-cup-2025-concludes",
            title: "3 Digit World Cup 2025 Concludes",
            preview: None
        }, NewsPost {
            id: 1679,
            author: "Hivie & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-04-01-springtime-showdown-art-contest-results.md",
            first_image: "https://i.ppy.sh/88967f844ac13ee49527367cb3c929acb90f51d8/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30342d30312d737072696e6774696d652d73686f77646f776e2d6172742d636f6e746573742d726573756c74732f62616e6e65722e6a7067",
            published_at: "2025-04-01T12:00:00+00:00",
            updated_at: "2025-04-04T01:15:15+00:00",
            slug: "2025-04-01-springtime-showdown-art-contest-results",
            title: "Springtime Showdown Art Contest: Results",
            preview: None
        }, NewsPost {
            id: 1678,
            author: "The Roundtable",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-31-the-roundtable-open-at-lvl-up-expo.md",
            first_image: "https://i.ppy.sh/e340ec265113f76f1dd0b9059544f533dfa96982/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d33312d7468652d726f756e647461626c652d6f70656e2d61742d6c766c2d75702d6578706f2f72746f5f62616e6e65722e706e67",
            published_at: "2025-03-31T20:00:00+00:00",
            updated_at: "2025-04-11T20:08:49+00:00",
            slug: "2025-03-31-the-roundtable-open-at-lvl-up-expo",
            title: "The Roundtable Open @ LVL UP EXPO",
            preview: None
        }, NewsPost {
            id: 1677,
            author: "shdewz",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-30-4-digit-world-cup-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/d6d23b1fb8d9a644aed68378c9fea549faba20f2/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f3457432f323032352f696d672f62616e6e65722e706e67",
            published_at: "2025-03-30T12:00:00+00:00",
            updated_at: "2025-03-30T12:00:00+00:00",
            slug: "2025-03-30-4-digit-world-cup-2025-registrations-now-open",
            title: "4 Digit World Cup 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1676,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-29-new-featured-artist-ak-q.md",
            first_image: "https://i.ppy.sh/b998ae14bbb88f15df5b16f1bd0182a723ee7513/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3436362f6865616465722e6a7067",
            published_at: "2025-03-29T19:15:00+00:00",
            updated_at: "2025-03-29T19:15:00+00:00",
            slug: "2025-03-29-new-featured-artist-ak-q",
            title: "New Featured Artist: ak+q",
            preview: None
        }, NewsPost {
            id: 1675,
            author: "Hivie & Yasuho",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-28-osutaiko-world-cup-2025-round-of-32-recap.md",
            first_image: "https://i.ppy.sh/c579fb93af243d03c0d4869a10f9ddc1cb499ed0/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f5457432f323032352f696d672f747763323032352d62616e6e65722e6a7067",
            published_at: "2025-03-28T06:00:00+00:00",
            updated_at: "2025-03-28T10:40:23+00:00",
            slug: "2025-03-28-osutaiko-world-cup-2025-round-of-32-recap",
            title: "osu!taiko World Cup 2025: Round of 32 Recap",
            preview: None
        }, NewsPost {
            id: 1674,
            author: "MegaMix",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-27-the-followpoint-special-behind-the-banhammer.md",
            first_image: "https://i.ppy.sh/1693a8ee768aae527a65db89b4b746aa448473e1/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d32372d7468652d666f6c6c6f77706f696e742d7370656369616c2d626568696e642d7468652d62616e68616d6d65722f62616e6e65722e6a7067",
            published_at: "2025-03-27T17:00:00+00:00",
            updated_at: "2025-03-27T17:00:00+00:00",
            slug: "2025-03-27-the-followpoint-special-behind-the-banhammer",
            title: "The Followpoint Special: Behind the Banhammer",
            preview: None
        }, NewsPost {
            id: 1673,
            author: "Maxus & momoyo",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-26-mappers-choice-awards-2024.md",
            first_image: "https://i.ppy.sh/b16ccc7ba838f8667d3b717f5ba7902d15a67cf8/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d32362d6d6170706572732d63686f6963652d6177617264732d323032342f62616e6e65722e6a7067",
            published_at: "2025-03-26T16:00:00+00:00",
            updated_at: "2025-04-16T15:49:26+00:00",
            slug: "2025-03-26-mappers-choice-awards-2024",
            title: "Mapper's Choice Awards 2024",
            preview: None
        }, NewsPost {
            id: 1672,
            author: "the osu! team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-25-community-contributors-2024.md",
            first_image: "https://i.ppy.sh/f28b1f3df9d65d3a9992b18c32c215a9ea149285/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d32352d636f6d6d756e6974792d636f6e7472696275746f72732d323032342f6865616465722e6a7067",
            published_at: "2025-03-25T19:30:00+00:00",
            updated_at: "2025-03-26T00:46:19+00:00",
            slug: "2025-03-25-community-contributors-2024",
            title: "Community Contributors: 2024",
            preview: None
        }, NewsPost {
            id: 1671,
            author: "the osu! team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-24-community-contributors-elite-nominators-2024.md",
            first_image: "https://i.ppy.sh/25691c5fb44568fb82419a8da147fcbfb66f9ed5/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d32342d636f6d6d756e6974792d636f6e7472696275746f72732d656c6974652d6e6f6d696e61746f72732d323032342f62616e6e65722e6a7067",
            published_at: "2025-03-24T14:00:00+00:00",
            updated_at: "2025-03-24T14:05:35+00:00",
            slug: "2025-03-24-community-contributors-elite-nominators-2024",
            title: "Community Contributors 2024: Elite Nominators",
            preview: None
        }, NewsPost {
            id: 1670,
            author: "0x84f & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-23-osulazer-updates-march-21-2025.md",
            first_image: "https://i.ppy.sh/5b9f7878febdc692d87475fe3c80c0f3ca9a541f/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d32332d6f73756c617a65722d757064617465732d6d617263682d32312d323032352f62616e6e65722e6a7067",
            published_at: "2025-03-23T19:00:00+00:00",
            updated_at: "2025-03-23T19:02:49+00:00",
            slug: "2025-03-23-osulazer-updates-march-21-2025",
            title: "osu!(lazer) Updates: March 21, 2025",
            preview: None
        }, NewsPost {
            id: 1669,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-22-new-featured-artist-rae.md",
            first_image: "https://i.ppy.sh/98fee006365d752338f7fb33e0668b261c76edf7/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3436342f6865616465722e6a7067",
            published_at: "2025-03-22T22:00:00+00:00",
            updated_at: "2025-03-22T22:10:35+00:00",
            slug: "2025-03-22-new-featured-artist-rae",
            title: "New Featured Artist: rae",
            preview: None
        }, NewsPost {
            id: 1668,
            author: "Hivie & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-21-springtime-showdown-art-contest-voting-open.md",
            first_image: "https://i.ppy.sh/a469c3ed755731a962347484fc63fc84afc7487a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d32382d737072696e6774696d652d73686f77646f776e2d6172742d636f6e746573742f62616e6e65722e6a7067",
            published_at: "2025-03-21T16:00:00+00:00",
            updated_at: "2025-03-22T18:16:08+00:00",
            slug: "2025-03-21-springtime-showdown-art-contest-voting-open",
            title: "Springtime Showdown Art Contest: Voting Open",
            preview: None
        }, NewsPost {
            id: 1667,
            author: "Hivie",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-20-project-loved-march-2025.md",
            first_image: "https://i.ppy.sh/f5982d2fe7d489faac36230fc235617f3bd1f6b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70726f6a6563742d6c6f7665642d322e6a7067",
            published_at: "2025-03-20T15:30:00+00:00",
            updated_at: "2025-03-20T15:30:00+00:00",
            slug: "2025-03-20-project-loved-march-2025",
            title: "Project Loved: March 2025",
            preview: None
        }, NewsPost {
            id: 1666,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-15-new-featured-artist-hikota.md",
            first_image: "https://i.ppy.sh/9879146d878b66434e5a6c65aa32d10c003480e2/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3436332f6865616465722e6a7067",
            published_at: "2025-03-15T20:30:00+00:00",
            updated_at: "2025-03-15T20:30:00+00:00",
            slug: "2025-03-15-new-featured-artist-hikota",
            title: "New Featured Artist: hikota",
            preview: None
        }, NewsPost {
            id: 1665,
            author: "Chaoslitz, seros & Elayue",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-14-pending-cup-2024-results.md",
            first_image: "https://i.ppy.sh/931c29d36406506ec02ba88b0f614fcd14f451f4/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f436f6e74657374732f5044432f323032342f696d672f62616e6e65722e6a7067",
            published_at: "2025-03-14T18:00:00+00:00",
            updated_at: "2025-03-14T18:00:00+00:00",
            slug: "2025-03-14-pending-cup-2024-results",
            title: "Pending Cup 2024 Results",
            preview: None
        }, NewsPost {
            id: 1664,
            author: "skinship",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-13-skin-of-the-year-2024-results.md",
            first_image: "https://i.ppy.sh/a5a934934b54f6344ec35f5d1a97e9a4a94fd790/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d31332d736b696e2d6f662d7468652d796561722d323032342d726573756c74732f62616e6e65722e6a7067",
            published_at: "2025-03-13T21:00:00+00:00",
            updated_at: "2025-03-13T22:22:43+00:00",
            slug: "2025-03-13-skin-of-the-year-2024-results",
            title: "Skin of the Year 2024: Results",
            preview: None
        }, NewsPost {
            id: 1663,
            author: "0x84f",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-09-gmt-apps-now-open.md",
            first_image: "https://i.ppy.sh/a1e6bde856d5eedd6ee4cffb9d751fb97e97e710/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f676d742d67656e657269632e6a7067",
            published_at: "2025-03-09T18:30:00+00:00",
            updated_at: "2025-05-09T03:37:28+00:00",
            slug: "2025-03-09-gmt-apps-now-open",
            title: "Global Moderation Team Applications Now Open!",
            preview: None
        }, NewsPost {
            id: 1662,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-09-new-featured-artist-seven-lives.md",
            first_image: "https://i.ppy.sh/2f8174349aab021c13f1c901296fcca94c3ccad1/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d30392d6e65772d66656174757265642d6172746973742d736576656e2d6c697665732f6865616465722e706e67",
            published_at: "2025-03-09T11:15:00+00:00",
            updated_at: "2025-05-08T12:05:53+00:00",
            slug: "2025-03-09-new-featured-artist-seven-lives",
            title: "New Featured Artist: SEVEN LIVES",
            preview: None
        }, NewsPost {
            id: 1661,
            author: "the performance point committees",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-06-performance-points-star-rating-updates.md",
            first_image: "https://i.ppy.sh/676abc25f777fb76a9a41d70befc030873c057a4/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70702d73722d70697070692e6a7067",
            published_at: "2025-03-06T10:00:00+00:00",
            updated_at: "2025-05-10T08:04:57+00:00",
            slug: "2025-03-06-performance-points-star-rating-updates",
            title: "Performance Points & Star Rating Updates",
            preview: None
        }, NewsPost {
            id: 1660,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-03-mappers-guild-spring-2025-priority-quests.md",
            first_image: "https://i.ppy.sh/b02f670efe62990d4affaa44de7d74391b2d13da/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f6d6170706572736775696c642e706e67",
            published_at: "2025-03-03T20:15:00+00:00",
            updated_at: "2025-05-02T21:34:56+00:00",
            slug: "2025-03-03-mappers-guild-spring-2025-priority-quests",
            title: "Mappers' Guild - Spring 2025 Priority Quests",
            preview: None
        }, NewsPost {
            id: 1659,
            author: "RandomeLoL, _Kobii, SurfChu85, Maxus, Sakura006",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-03-osumania-7k-world-cup-2025-concludes.md",
            first_image: "https://i.ppy.sh/99614d6db3717c3ab8b12e61bfa8a8ffb53a5321/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30332d30332d6f73756d616e69612d376b2d776f726c642d6375702d323032352d636f6e636c756465732f62616e6e65722e6a7067",
            published_at: "2025-03-03T01:00:00+00:00",
            updated_at: "2025-05-03T00:38:07+00:00",
            slug: "2025-03-03-osumania-7k-world-cup-2025-concludes",
            title: "osu!mania 7K World Cup 2025 Concludes",
            preview: None
        }, NewsPost {
            id: 1658,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-03-02-featured-artist-track-updates-elfensjon.md",
            first_image: "https://i.ppy.sh/2583e16eef9594882e704412880cb813ff65246a/68747470733a2f2f6173736574732e7070792e73682f617274697374732f36392f6865616465722e6a7067",
            published_at: "2025-03-02T20:15:00+00:00",
            updated_at: "2025-05-01T20:25:55+00:00",
            slug: "2025-03-02-featured-artist-track-updates-elfensjon",
            title: "Featured Artist Track Updates: ELFENSJoN",
            preview: None
        }, NewsPost {
            id: 1657,
            author: "Hivie & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-28-springtime-showdown-art-contest.md",
            first_image: "https://i.ppy.sh/a469c3ed755731a962347484fc63fc84afc7487a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d32382d737072696e6774696d652d73686f77646f776e2d6172742d636f6e746573742f62616e6e65722e6a7067",
            published_at: "2025-02-28T16:00:00+00:00",
            updated_at: "2025-04-29T17:42:06+00:00",
            slug: "2025-02-28-springtime-showdown-art-contest",
            title: "Springtime Showdown Art Contest",
            preview: None
        }, NewsPost {
            id: 1656,
            author: "Walavouchey & RandomeLoL",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-27-introducing-teams.md",
            first_image: "https://i.ppy.sh/1557910d309bd1d74ddf3d639ffe8c1174cc4a53/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d32372d696e74726f647563696e672d7465616d732f62616e6e65722e6a7067",
            published_at: "2025-02-27T19:00:00+00:00",
            updated_at: "2025-04-28T20:19:31+00:00",
            slug: "2025-02-27-introducing-teams",
            title: "Introducing Teams",
            preview: None
        }, NewsPost {
            id: 1655,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-22-new-featured-artist-weary.md",
            first_image: "https://i.ppy.sh/effbe763d6c283f2f28e9d2afe10232d94c8b1b7/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3436302f6865616465722e6a7067",
            published_at: "2025-02-22T18:00:00+00:00",
            updated_at: "2025-04-23T20:03:19+00:00",
            slug: "2025-02-22-new-featured-artist-weary",
            title: "New Featured Artist: WEARY",
            preview: None
        }, NewsPost {
            id: 1654,
            author: "RandomeLoL, _Kobii, [GB]yobrevelc, Maxus, SurfChu85",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-21-osumania-7k-world-cup-2025-finals-recap.md",
            first_image: "https://i.ppy.sh/30c2fcce568c919cfe43a5cec3e705a3ab2466ff/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d32312d6f73756d616e69612d376b2d776f726c642d6375702d323032352d66696e616c732d72656361702f62616e6e65722e6a7067",
            published_at: "2025-02-21T23:59:00+00:00",
            updated_at: "2025-04-23T15:14:20+00:00",
            slug: "2025-02-21-osumania-7k-world-cup-2025-finals-recap",
            title: "osu!mania 7K World Cup 2025: Finals Recap",
            preview: None
        }, NewsPost {
            id: 1653,
            author: "0x84f & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-20-osulazer-updates-february-20-2025.md",
            first_image: "https://i.ppy.sh/2f581bcad06aa13acb34dd19cdc2edf8eb88ea4e/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d32302d6f73756c617a65722d757064617465732d66656272756172792d32302d323032352f62616e6e65722e6a7067",
            published_at: "2025-02-20T18:30:00+00:00",
            updated_at: "2025-04-21T18:51:00+00:00",
            slug: "2025-02-20-osulazer-updates-february-20-2025",
            title: "osu!(lazer) Updates: February 20, 2025",
            preview: None
        }, NewsPost {
            id: 1652,
            author: "Hivie",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-18-community-choice-2024-results.md",
            first_image: "https://i.ppy.sh/007045b59d74e1db6e00bae69bdd950cf814f79d/68747470733a2f2f6173736574732e7070792e73682f636f6e74657374732f3233312f6865616465722e6a7067",
            published_at: "2025-02-18T17:45:00+00:00",
            updated_at: "2025-04-20T00:30:00+00:00",
            slug: "2025-02-18-community-choice-2024-results",
            title: "Community Choice 2024: Results",
            preview: None
        }, NewsPost {
            id: 1651,
            author: "pishifat, RandomeLoL",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-17-monthly-beatmapping-contest-february-2025.md",
            first_image: "https://i.ppy.sh/e62524b04dc9f7eac075ee76c8c30a35a0e9ba4a/68747470733a2f2f6173736574732e7070792e73682f6d656469612f6d6f6e74686c792d626561746d617070696e672d636f6e746573742e706e67",
            published_at: "2025-02-17T22:00:00+00:00",
            updated_at: "2025-04-18T23:28:21+00:00",
            slug: "2025-02-17-monthly-beatmapping-contest-february-2025",
            title: "Monthly Beatmapping Contest: February 2025",
            preview: None
        }, NewsPost {
            id: 1650,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-16-new-featured-artist-ludicin.md",
            first_image: "https://i.ppy.sh/3b06d1fbe27cb5f18ca0b245165642ca00b78132/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3435392f6865616465722e6a7067",
            published_at: "2025-02-16T18:45:00+00:00",
            updated_at: "2025-04-18T00:33:23+00:00",
            slug: "2025-02-16-new-featured-artist-ludicin",
            title: "New Featured Artist: Ludicin",
            preview: None
        }, NewsPost {
            id: 1649,
            author: "LeoFLT",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-15-osutaiko-world-cup-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/c579fb93af243d03c0d4869a10f9ddc1cb499ed0/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f5457432f323032352f696d672f747763323032352d62616e6e65722e6a7067",
            published_at: "2025-02-15T00:00:00+00:00",
            updated_at: "2025-04-16T03:58:13+00:00",
            slug: "2025-02-15-osutaiko-world-cup-2025-registrations-now-open",
            title: "osu!taiko World Cup 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1648,
            author: "0x84f",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-14-project-loved-february-2025.md",
            first_image: "https://i.ppy.sh/f5982d2fe7d489faac36230fc235617f3bd1f6b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70726f6a6563742d6c6f7665642d322e6a7067",
            published_at: "2025-02-14T13:45:00+00:00",
            updated_at: "2025-04-16T03:58:10+00:00",
            slug: "2025-02-14-project-loved-february-2025",
            title: "Project Loved: February 2025",
            preview: None
        }, NewsPost {
            id: 1647,
            author: "RandomeLoL, _Kobii, [GB]yobrevelc, Maxus, SurfChu85, yukina meng",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-13-osumania-7k-world-cup-2025-semifinals-recap.md",
            first_image: "https://i.ppy.sh/ad0e7a322c2670ef55985d4488ebc4b51ed2bf62/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d31332d6f73756d616e69612d376b2d776f726c642d6375702d323032352d73656d6966696e616c732d72656361702f62616e6e65722e6a7067",
            published_at: "2025-02-13T20:30:00+00:00",
            updated_at: "2025-04-15T03:13:53+00:00",
            slug: "2025-02-13-osumania-7k-world-cup-2025-semifinals-recap",
            title: "osu!mania 7K World Cup 2025: Semifinals Recap",
            preview: None
        }, NewsPost {
            id: 1646,
            author: "osu! team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-12-osu-tablets-return.md",
            first_image: "https://i.ppy.sh/34478cf97f0ba67d98fe2d91487eea236be0b1b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d31322d6f73752d7461626c6574732d72657475726e2f62616e6e65722e6a7067",
            published_at: "2025-02-12T03:30:00+00:00",
            updated_at: "2025-04-13T07:04:42+00:00",
            slug: "2025-02-12-osu-tablets-return",
            title: "osu!tablets return",
            preview: None
        }, NewsPost {
            id: 1645,
            author: "skinship",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-11-skin-of-the-year-2024.md",
            first_image: "https://i.ppy.sh/3eab8deb40608117991986ec85e17125323593da/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d31312d736b696e2d6f662d7468652d796561722d323032342f62616e6e65722e6a7067",
            published_at: "2025-02-11T18:00:00+00:00",
            updated_at: "2025-04-12T19:39:06+00:00",
            slug: "2025-02-11-skin-of-the-year-2024",
            title: "Skin of the Year 2024: Voting Open",
            preview: None
        }, NewsPost {
            id: 1644,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-08-new-featured-artist-silis.md",
            first_image: "https://i.ppy.sh/3ecb14d57775ef076a041e6d8fb084d0e652a0f3/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30322d30382d6e65772d66656174757265642d6172746973742d73696c69732f6865616465722e6a70673f31",
            published_at: "2025-02-08T15:55:00+00:00",
            updated_at: "2025-04-09T22:27:28+00:00",
            slug: "2025-02-08-new-featured-artist-silis",
            title: "New Featured Artist: SiLiS",
            preview: None
        }, NewsPost {
            id: 1643,
            author: "RandomeLoL, SurfChu85, Maxus, [GB]yobrevelc",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-07-osumania-7k-world-cup-2025-quarterfinals-recap.md",
            first_image: "https://i.ppy.sh/1a9e007aaf7663b4d7b552c55a70d3f593497d52/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4d57432f323032355f374b2f696d672f6d7763376b323032352d62616e6e65722e6a7067",
            published_at: "2025-02-07T12:00:00+00:00",
            updated_at: "2025-04-08T21:03:21+00:00",
            slug: "2025-02-07-osumania-7k-world-cup-2025-quarterfinals-recap",
            title: "osu!mania 7K World Cup 2025: Quarterfinals Recap",
            preview: None
        }, NewsPost {
            id: 1642,
            author: "LeoFLT",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-04-osutaiko-osucatch-world-cup-2025-staff-recruitment.md",
            first_image: "https://i.ppy.sh/fca63905df92341c9376e82b936c73671bb0b9e2/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f4f57435f4d61737465725f4c6f676f2e706e67",
            published_at: "2025-02-04T09:00:00+00:00",
            updated_at: "2025-04-05T14:32:03+00:00",
            slug: "2025-02-04-osutaiko-osucatch-world-cup-2025-staff-recruitment",
            title: "osu!taiko & osu!catch World Cup 2025 Staff Recruitment",
            preview: None
        }, NewsPost {
            id: 1641,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-02-01-featured-artist-track-updates-akiri.md",
            first_image: "https://i.ppy.sh/eb65a75e8d8bdbb6602b56525c56567054da36af/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3430312f6865616465722e6a7067",
            published_at: "2025-02-01T19:00:00+00:00",
            updated_at: "2025-04-03T06:11:06+00:00",
            slug: "2025-02-01-featured-artist-track-updates-akiri",
            title: "Featured Artist Track Updates: Akiri",
            preview: None
        }, NewsPost {
            id: 1640,
            author: "mango, Matrix, and a bunch of other people",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-31-locus.md",
            first_image: "https://i.ppy.sh/2ad7b99ded7eff3483731d0dcc3405bf4ac84540/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f436f6e74657374732f4c6f6375732f323032352f696d672f62616e6e65722e6a7067",
            published_at: "2025-01-31T18:30:00+00:00",
            updated_at: "2025-04-01T19:33:07+00:00",
            slug: "2025-01-31-locus",
            title: "Locus",
            preview: None
        }, NewsPost {
            id: 1639,
            author: "RandomeLoL, SurfChu85, Maxus",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-31-osumania-7k-world-cup-2025-round-of-16-recap.md",
            first_image: "https://i.ppy.sh/1a9e007aaf7663b4d7b552c55a70d3f593497d52/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4d57432f323032355f374b2f696d672f6d7763376b323032352d62616e6e65722e6a7067",
            published_at: "2025-01-31T11:30:00+00:00",
            updated_at: "2025-04-04T03:53:48+00:00",
            slug: "2025-01-31-osumania-7k-world-cup-2025-round-of-16-recap",
            title: "osu!mania 7K World Cup 2025: Round of 16 Recap",
            preview: None
        }, NewsPost {
            id: 1638,
            author: "Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-30-community-choice-2024-voting-open.md",
            first_image: "https://i.ppy.sh/007045b59d74e1db6e00bae69bdd950cf814f79d/68747470733a2f2f6173736574732e7070792e73682f636f6e74657374732f3233312f6865616465722e6a7067",
            published_at: "2025-01-30T16:00:00+00:00",
            updated_at: "2025-04-01T11:23:51+00:00",
            slug: "2025-01-30-community-choice-2024-voting-open",
            title: "Community Choice 2024: Voting Open",
            preview: None
        }, NewsPost {
            id: 1637,
            author: "Mafumafu, Fycho, Atrue",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-29-newspaper-cup-2025.md",
            first_image: "https://i.ppy.sh/65ca74e52355785e57def056ee04f1a6a0ff438a/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d32392d6e65777370617065722d6375702d323032352f62616e6e65722e6a7067",
            published_at: "2025-01-29T12:15:00+00:00",
            updated_at: "2025-04-02T17:14:32+00:00",
            slug: "2025-01-29-newspaper-cup-2025",
            title: "Newspaper Cup 2025",
            preview: None
        }, NewsPost {
            id: 1636,
            author: "Bubbleman",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-27-osu-uk-epic44.md",
            first_image: "https://i.ppy.sh/53c0800077ebf03c3ab975020661729c2643cdd2/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d32372d6f73752d756b2d6570696334342f62616e6e65722e6a7067",
            published_at: "2025-01-27T18:00:00+00:00",
            updated_at: "2025-03-28T20:00:04+00:00",
            slug: "2025-01-27-osu-uk-epic44",
            title: "osu! UK @ EPIC44",
            preview: None
        }, NewsPost {
            id: 1635,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-25-new-featured-artist-powerwolf.md",
            first_image: "https://i.ppy.sh/34e7fce1b136d2ad0982aea8cedf2c2e6122b8b4/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3435372f6865616465722e6a7067",
            published_at: "2025-01-25T21:30:00+00:00",
            updated_at: "2025-03-31T19:33:17+00:00",
            slug: "2025-01-25-new-featured-artist-powerwolf",
            title: "New Featured Artist: Powerwolf",
            preview: None
        }, NewsPost {
            id: 1634,
            author: "RandomeLoL, Maxus, SurfChu85, [GB]yobrevelc",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-24-osumania-7k-world-cup-2025-round-of-32-recap.md",
            first_image: "https://i.ppy.sh/1a9e007aaf7663b4d7b552c55a70d3f593497d52/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4d57432f323032355f374b2f696d672f6d7763376b323032352d62616e6e65722e6a7067",
            published_at: "2025-01-24T20:00:00+00:00",
            updated_at: "2025-03-25T20:50:36+00:00",
            slug: "2025-01-24-osumania-7k-world-cup-2025-round-of-32-recap",
            title: "osu!mania 7K World Cup 2025: Round of 32 Recap",
            preview: None
        }, NewsPost {
            id: 1633,
            author: "pishifat, Hivie, Morusya",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-23-monthly-beatmapping-contest-january-2025.md",
            first_image: "https://i.ppy.sh/e62524b04dc9f7eac075ee76c8c30a35a0e9ba4a/68747470733a2f2f6173736574732e7070792e73682f6d656469612f6d6f6e74686c792d626561746d617070696e672d636f6e746573742e706e67",
            published_at: "2025-01-23T19:30:00+00:00",
            updated_at: "2025-03-24T20:22:25+00:00",
            slug: "2025-01-23-monthly-beatmapping-contest-january-2025",
            title: "Monthly Beatmapping Contest: January 2025",
            preview: None
        }, NewsPost {
            id: 1632,
            author: "the Mentorship Organisation Team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-20-community-mentorship-program-spring-2025-signups-now-open.md",
            first_image: "https://i.ppy.sh/e804199f65907595f127cf0b01cc73d293b5060b/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f636f6d6d756e6974792d6d656e746f72736869702d70726f6772616d2e6a7067",
            published_at: "2025-01-20T13:00:00+00:00",
            updated_at: "2025-03-21T23:28:01+00:00",
            slug: "2025-01-20-community-mentorship-program-spring-2025-signups-now-open",
            title: "Community Mentorship Program Spring 2025 Signups Now Open",
            preview: None
        }, NewsPost {
            id: 1631,
            author: "osu! team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-19-meet-the-medal-pioneers.md",
            first_image: "https://i.ppy.sh/ae9b777dd8a64fc71591a814431b232682337b1d/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d31392d6d6565742d7468652d6d6564616c2d70696f6e656572732f62616e6e65722e6a7067",
            published_at: "2025-01-19T15:15:00+00:00",
            updated_at: "2025-03-20T15:46:24+00:00",
            slug: "2025-01-19-meet-the-medal-pioneers",
            title: "meet the medal pioneers",
            preview: None
        }, NewsPost {
            id: 1630,
            author: "0x84f & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-19-osulazer-updates-january-19-2025.md",
            first_image: "https://i.ppy.sh/340f1add3855812af6e152072fd84842cb3876d1/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d31392d6f73756c617a65722d757064617465732d6a616e756172792d31392d323032352f62616e6e65722e6a7067",
            published_at: "2025-01-19T12:00:00+00:00",
            updated_at: "2025-03-20T17:59:19+00:00",
            slug: "2025-01-19-osulazer-updates-january-19-2025",
            title: "osu!(lazer) Updates: January 19, 2025",
            preview: None
        }, NewsPost {
            id: 1629,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-18-new-featured-artist-ntyn.md",
            first_image: "https://i.ppy.sh/31fdc4b042d47fee77d1352b6dcaf5308e4bc697/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3435362f6865616465722e6a7067",
            published_at: "2025-01-18T20:00:00+00:00",
            updated_at: "2025-03-20T15:46:28+00:00",
            slug: "2025-01-18-new-featured-artist-ntyn",
            title: "New Featured Artist: ntyn",
            preview: None
        }, NewsPost {
            id: 1628,
            author: "RandomeLoL, _Kobii, Maxus",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-17-osumania-7k-world-cup-2025-qualifiers-recap.md",
            first_image: "https://i.ppy.sh/1a9e007aaf7663b4d7b552c55a70d3f593497d52/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4d57432f323032355f374b2f696d672f6d7763376b323032352d62616e6e65722e6a7067",
            published_at: "2025-01-17T21:00:00+00:00",
            updated_at: "2025-03-19T07:54:13+00:00",
            slug: "2025-01-17-osumania-7k-world-cup-2025-qualifiers-recap",
            title: "osu!mania 7K World Cup 2025: Qualifiers Recap",
            preview: None
        }, NewsPost {
            id: 1627,
            author: "aceticke",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-17-project-loved-january-2025.md",
            first_image: "https://i.ppy.sh/f5982d2fe7d489faac36230fc235617f3bd1f6b9/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f70726f6a6563742d6c6f7665642d322e6a7067",
            published_at: "2025-01-17T14:15:00+00:00",
            updated_at: "2025-03-19T07:23:20+00:00",
            slug: "2025-01-17-project-loved-january-2025",
            title: "Project Loved: January 2025",
            preview: None
        }, NewsPost {
            id: 1626,
            author: "ChillierPear & Stage",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-13-tournament-committee-applications-now-open.md",
            first_image: "https://i.ppy.sh/48b9b2e14a18f48701e4b9eb9628ed5ce1ce98b6/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d31332d746f75726e616d656e742d636f6d6d69747465652d6170706c69636174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
            published_at: "2025-01-13T17:00:00+00:00",
            updated_at: "2025-03-16T03:45:38+00:00",
            slug: "2025-01-13-tournament-committee-applications-now-open",
            title: "Tournament Committee Applications Now Open!",
            preview: None
        }, NewsPost {
            id: 1625,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-12-new-featured-artist-zvlian.md",
            first_image: "https://i.ppy.sh/3e9ea4b2c7629da66a5f17ed950402290cfb0d75/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3435352f6865616465722e6a7067",
            published_at: "2025-01-12T01:00:00+00:00",
            updated_at: "2025-03-13T10:13:35+00:00",
            slug: "2025-01-12-new-featured-artist-zvlian",
            title: "New Featured Artist: ZVLIAN",
            preview: None
        }, NewsPost {
            id: 1624,
            author: "Nomination Assessment Team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-10-bn-mentorship-cycle-4.md",
            first_image: "https://i.ppy.sh/3b1df650d5eac708379d5dc673bc5d94f39fee52/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f626e2d6d656e746f72736869702f62616e6e65722e6a7067",
            published_at: "2025-01-10T12:00:00+00:00",
            updated_at: "2025-05-11T00:24:20+00:00",
            slug: "2025-01-10-bn-mentorship-cycle-4",
            title: "BN Mentorship: Cycle 4 Sign-ups Open",
            preview: None
        }, NewsPost {
            id: 1623,
            author: "Kasumi-sama and the GTS Admin Team",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-05-expert-global-taiko-showdown-2025-registrations-now-open.md",
            first_image: "https://i.ppy.sh/3372b4cd7dd6552bd0ac488f7ccd493999eda83f/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d30352d656774732d323032352d726567697374726174696f6e732d6e6f772d6f70656e2f62616e6e65722e6a7067",
            published_at: "2025-01-05T00:00:00+00:00",
            updated_at: "2025-05-05T06:35:58+00:00",
            slug: "2025-01-05-expert-global-taiko-showdown-2025-registrations-now-open",
            title: "Expert Global Taiko Showdown 2025: Registrations Now Open!",
            preview: None
        }, NewsPost {
            id: 1622,
            author: "pishifat",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-04-featured-artist-track-updates-winter-2025.md",
            first_image: "https://i.ppy.sh/2c599f8ef010307562c66fbe17a553ca142304ec/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f62616e6e6572732f66656174757265642d6172746973742e6a7067",
            published_at: "2025-01-04T21:00:00+00:00",
            updated_at: "2025-05-01T05:23:06+00:00",
            slug: "2025-01-04-featured-artist-track-updates-winter-2025",
            title: "Featured Artist Track Updates: Winter 2025",
            preview: None
        }, NewsPost {
            id: 1621,
            author: "Hivie & Walavouchey",
            edit_url: "https://github.com/ppy/osu-wiki/tree/master/news/2025/2025-01-01-midnight-moment-art-contest-results.md",
            first_image: "https://i.ppy.sh/69eb119a885640d1c215cd56b51c24b2e1ad4236/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032352d30312d30312d6d69646e696768742d6d6f6d656e742d6172742d636f6e746573742d726573756c74732f62616e6e65722e6a7067",
            published_at: "2025-01-01T02:30:00+00:00",
            updated_at: "2025-05-01T10:38:34+00:00",
            slug: "2025-01-01-midnight-moment-art-contest-results",
            title: "Midnight Moment Art Contest: Results",
            preview: None
        }],
        years: [2025, 2024, 2023, 2022, 2021, 2020, 2019, 2018, 2017, 2016, 2015, 2014, 2013]
    },
    search: Search {
        limit: 12,
        sort: "published_desc"
    },
    cursor_string: "eyJwdWJsaXNoZWRfYXQiOiIyMDI1LTA0LTIwVDEyOjE1OjAwLjAwMDAwMFoiLCJpZCI6MTY5Mn0"
}
*/
