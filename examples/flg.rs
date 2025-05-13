// Get forum listing
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
    let forums = client.forum.get_forum_listing().await?;
    println!("{:?}", forums);
    Ok(())
}

/*
ReqwestForum get_forum_listing
Forums {
    forums: [Forum {
        id: 1,
        name: "osu!",
        description: "osu! specific",
        subforums: Some([Forum {
            id: 2,
            name: "Development",
            description: "Let's move osu! forwards.  Discuss projects, considerations, news and publicity, meetings and the likes.",
            subforums: Some([Forum {
                id: 116,
                name: "Completed Projects",
                description: "Completed projects can be archived and discussed here.",
                subforums: None
            }, Forum {
                id: 65,
                name: "Community Voice",
                description: "Weekly polls to help us understand our community a bit better.",
                subforums: None
            }, Forum {
                id: 4,
                name: "Feature Requests",
                description: "Suggest what you would like to see in osu!.",
                subforums: None
            }, Forum {
                id: 9,
                name: "Announcements (Archived)",
                description: "Important stuff. Feel free to post comments!",
                subforums: None
            }])
        }, Forum {
            id: 13,
            name: "Gameplay & Rankings",
            description: "Discuss your scores, strategies etc.  Challenge people.  Anything about playing osu!.",
            subforums: Some([Forum {
                id: 105,
                name: "osu!taiko",
                description: "",
                subforums: NNone
            }, Forum {
                id: 106,
                name: "osu!catch",
                description: "",
                subforums: None
            }, Forum {
                id: 107,
                name: "osu!mania",
                description: "",
                subforums: None
            }])
        }, Forum {
            id: 126,
            name: "Tournaments & Contests",
            description: "To become the very best.",
            subforums: Some([Forum {
                id: 55,
                name: "Tournaments",
                description: "Get involved in community tournaments for some friendly or fierce competition!",
                subforums: None
            }, Forum {
                id: 127,
                name: "Contests",
                description: "Compete in ways which don't involve playing the game!",
                subforums: None
            }])
        }, Forum {
            id: 15,
            name: "Skinning",
            description: "Discuss skinning and customization!",
            subforums: Some([Forum {
                id: 109,
                name: "Completed Skins",
                description: "",
                subforums: None
            }, Forum {
                id: 124,
                name: "Remixed Skins",
                description: "Skins which combine one or more other skins to make something new.",
                subforums: None
            }, Forum {
                id: 119,
                name: "Work In Progress Skins",
                description: "Share your original creations here.",
                subforums: None
            }])
        }, Forum {
            id: 5,
            name: "Help",
            description: "The place to go when you have a problem to report or a question to ask.",
            subforums: Some([Forum {
                id: 101,
                name: "Confirmed Issues",
                description: "",
                subforums: None
            }, Forum {
                id: 29,
                name: "Resolved Issues",
                description: "All resolved bugs RIP here.",
                subforums: None
            }])
        }])
    }, Forum {
        id: 12,
        name: "Beatmaps",
        description: "",
        subforums: Some([Forum {
            id: 56,
            name: "Mapping Discussion",
            description: "Anything-goes discussion about beatmapping, the editor etc.",
            subforums: Some([Forum {
                id: 87,
                name: "Ranking Criteria",
                description: "",
                subforums: None
            }, Forum {
                id: 61,
                name: "Mapping Techniques",
                description: "",
                subforums: None
            }, Forum {
                id: 20,
                name: "Storyboarding",
                description: "Discuss storyboarding, request help, and share your WIPs.",
                subforums: None
            }, Forum {
                id: 115,
                name: "Beatmap Management",
                description: "Members of the Beatmap Nominators discuss and propose changes to the mapping ecosystem here!",
                subforums: None
            }])
        }, Forum {
            id: 60,
            name: "Modding Queues",
            description: "Help other mappers or find someone to help mod your beatmap.",
            subforums: Some([Forum {
                id: 121,
                name: "osu!taiko",
                description: "Help other mappers or find someone to help mod your osu!taiko beatmap.",
                subforums: None
            }, Forum {
                id: 122,
                name: "osu!catch",
                description: "Help other mappers or find someone to help mod your osu!catch beatmap.",
                subforums: None
            }, Forum {
                id: 123,
                name: "osu!mania",
                description: "Help other mappers or find someone to help mod your osu!mania beatmap.",
                subforums: None
            }])
        }, Forum {
            id: 53,
            name: "Beatmap Projects",
            description: "Announce, discuss and participate in beatmapping projects here.",
            subforums: Some([Forum {
                id: 120,
                name: "Project Loved",
                description: "Vote for the beatmaps you'd like to see under the \"Loved\" category!",
                subforums: None
            }, Forum {
                id: 62,
                name: "Completed Projects",
                description: "",
                subforums: None
            }])
        }, Forum {
            id: 14,
            name: "Ranked Beatmaps (Archived)",
            description: "Ranked beatmap threads reside in here. Archived for posterity.\r\n",
            subforums: Some([Forum {
                id: 6,
                name: "Pending Beatmaps",
                description: "Beatmaps that are usually complete but pending approval.",
                subforums: None
            }, Forum {
                id: 10,
                name: "Works In Progress/Help",
                description: "Post your work-in-progress beatmaps, or find general help in here!",
                subforums: None
            }, Forum {
                id: 19,
                name: "Beatmap Graveyard",
                description: "Beatmaps that haven't been active for 4 weeks or more will be moved here.",
                subforums: None
            }])
        }])
    }, Forum {
        id: 11,
        name: "Other",
        description: "",
        subforums: Some([Forum {
            id: 7,
            name: "General Discussion",
            description: "Post about anything non-osu!, as long as it's quality content.",
            subforums: Some([])
        }, Forum {
            id: 52,
            name: "Off-Topic",
            description: "For the not-so-logical discussions.",
            subforums: Some([Forum {
                id: 68,
                name: "Forum Games",
                description: "A forum of forum games!",
                subforums: None
            }, Forum {
                id: 114,
                name: "Surveys",
                description: "PRETEND THIS IS ASK.FM AND ASK PEOPLE QUESTIONS THAT NOBODY CARES ABOUT.",
                subforums: None
            }])
        }, Forum {
            id: 8,
            name: "Introductions",
            description: "Introduce yourself!",
            subforums: Some([])
        }, Forum {
            id: 91,
            name: "Music Hall",
            description: "Music discussion outside the realm of osu!.",
            subforums: Some([])
        }, Forum {
            id: 75,
            name: "Otaku Culture",
            description: "Discuss Anime, Manga, and other Otaku-related topics here.",
            subforums: Some([])
        }, Forum {
            id: 17,
            name: "Video Games",
            description: "Discuss any and all non-osu! games in here. This includes other beat-style games.",
            subforums: Some([])
        }, Forum {
            id: 103,
            name: "Art",
            description: "Post your masterpieces, find new avatars and more!",
            subforums: Some([])
        }])
    }, Forum {
        id: 23,
        name: "Language Specific",
        description: "",
        subforums: Some([Forum {
            id: 35,
            name: "Русский",
            description: "Russian",
            subforums: Some([])
        }, Forum {
            id: 25,
            name: "中文",
            description: "Chinese",
            subforums: Some([])
        }, Forum {
            id: 58,
            name: "한국어",
            description: "Korean",
            subforums: Some([])
        }, Forum {
            id: 18,
            name: "Other Languages",
            description: "",
            subforums: Some([Forum {
                id: 73,
                name: "Indonesian",
                description: "",
                subforums: None
            }, Forum {
                id: 94,
                name: "Malaysian",
                description: "",
                subforums: None
            }, Forum {
                id: 37,
                name: "Deutsch",
                description: "German",
                subforums: None
            }, Forum {
                id: 33,
                name: "Español",
                description: "Spanish",
                subforums: None
            }, Forum {
                id: 34,
                name: "Français",
                description: "French",
                subforums: None
            }, Forum {
                id: 36,
                name: "Italiano",
                description: "Italian",
                subforums: None
            }, Forum {
                id: 95,
                name: "Magyar",
                description: "Hungarian",
                subforums: None
            }, Forum {
                id: 69,
                name: "Nederlands",
                description: "Dutch",
                subforums: None
            }, Forum {
                id: 26,
                name: "Polski",
                description: "Polish",
                subforums: None
            }, Forum {
                id: 74,
                name: "Português",
                description: "Portuguese",
                subforums: None
            }, Forum {
                id: 77,
                name: "Skandinavien",
                description: "Danish/Norwegian/Swedish",
                subforums: None
            }, Forum {
                id: 24,
                name: "Suomi",
                description: "Finnish",
                subforums: None
            }, Forum {
                id: 76,
                name: "Tagalog",
                description: "Filipino",
                subforums: None
            }, Forum {
                id: 93,
                name: "Türkçe",
                description: "Turkish",
                subforums: None
            }, Forum {
                id: 54,
                name: "ภาษาไทย",
                description: "Thai",
                subforums: None
            }, Forum {
                id: 32,
                name: "日本語",
                description: "Japanese",
                subforums: None
            }])
        }])
    }]
}
*/
