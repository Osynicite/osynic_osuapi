// Get events
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::events::IEvents;
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
    let events = client.events.get_events(None, None).await?;
    println!("{:?}", events);
    Ok(())
}

/*
ReqwestEvents get_events
GetEventsResponse {
    events: [Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:51+00:00",
            created_at_alt: "2025-05-11T12:13:51+00:00",
            id: 908876299,
            event_type: "rank",
            user: User {
                username: "MrKocheng",
                url: "/u/12158505"
            }
        },
        score_rank: "X",
        rank: 39,
        mode: "mania",
        beatmap: Beatmap {
            title: "xaki - rog-unlimitation [medkek's Easy]",
            url: "/b/4273482?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:51+00:00",
            created_at_alt: "2025-05-11T12:13:51+00:00",
            id: 908876298,
            event_type: "rank",
            user: User {
                username: "sumini",
                url: "/u/37734801"
            }
        },
        score_rank: "A",
        rank: 25,
        mode: "mania",
        beatmap: Beatmap {
            title: "Yorushika - Forget it [To You, Whom I Can't Forget]",
            url: "/b/4745387?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:51+00:00",
            created_at_alt: "2025-05-11T12:13:51+00:00",
            id: 908876297,
            event_type: "rank",
            user: User {
                username: "WubWoofWolf",
                url: "/u/39828"
            }
        },
        score_rank: "XH",
        rank: 52,
        mode: "osu",
        beatmap: Beatmap {
            title: "SENTIVE - Plymouth Forget (loop) [Normal]",
            url: "/b/4478782?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:51+00:00",
            created_at_alt: "2025-05-11T12:13:51+00:00",
            id: 908876296,
            event_type: "rank",
            user: User {
                username: "Chefsito2151",
                url: "/u/35728086"
            }
        },
        score_rank: "B",
        rank: 313,
        mode: "osu",
        beatmap: Beatmap {
            title: "Traktion - Gid V (Cut Ver.) [Smile]",
            url: "/b/3628308?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:50+00:00",
            created_at_alt: "2025-05-11T12:13:50+00:00",
            id: 908876295,
            event_type: "achievement",
            user: User {
                username: "Vo1D_agent",
                url: "/users/36046390"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/osu-combo-1000.png",
            id: 4,
            name: "1,000 Combo",
            grouping: "Skill & Dedication",
            ordering: 0,
            slug: "osu-combo-1000",
            description: "A thousand reasons why you rock at this game.",
            mode: Some("osu"),
            instructions: Some("aiming for a combo of at least 1,000 on any beatmap (try a <a href='/p/beatmaplist?q=marathon'>marathon</a>)")
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:50+00:00",
            created_at_alt: "2025-05-11T12:13:50+00:00",
            id: 908876294,
            event_type: "achievement",
            user: User {
                username: "jasminemilkteas",
                url: "/users/37845963"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/all-intro-nofail.png",
            id: 127,
            name: "Risk Averse",
            grouping: "Mod Introduction",
            ordering: 0,
            slug: "all-intro-nofail",
            description: "Safety nets are fun!",
            mode: None,
            instructions: Some("completing a map with the <b>No Fail</b> mod enabled!")
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:50+00:00",
            created_at_alt: "2025-05-11T12:13:50+00:00",
            id: 908876293,
            event_type: "rank",
            user: User {
                username: "MarcusTZ",
                url: "/u/36512593"
            }
        },
        score_rank: "B",
        rank: 382,
        mode: "mania",
        beatmap: Beatmap {
            title: "25-ji, Nightcord de. x KAITO - BAKENOHANA [I want to disappear.]",
            url: "/b/4823582?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:49+00:00",
            created_at_alt: "2025-05-11T12:13:49+00:00",
            id: 908876292,
            event_type: "rank",
            user: User {
                username: "Umekoo",
                url: "/u/26985102"
            }
        },
        score_rank: "B",
        rank: 45,
        mode: "osu",
        beatmap: Beatmap {
            title: "Yousei Teikoku - Kyouki Chinden (TV Size) [Extra]",
            url: "/b/5043504?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:49+00:00",
            created_at_alt: "2025-05-11T12:13:49+00:00",
            id: 908876291,
            event_type: "rank",
            user: User {
                username: "Karal982",
                url: "/u/26071701"
            }
        },
        score_rank: "A",
        rank: 67,
        mode: "osu",
        beatmap: Beatmap {
            title: "LonePi feat. Hanakuma Chifuyu - Gentiana [Rehabilitate]",
            url: "/b/4716340?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:48+00:00",
            created_at_alt: "2025-05-11T12:13:48+00:00",
            id: 908876290,
            event_type: "achievement",
            user: User {
                username: "skrijlex",
                url: "/users/31250410"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/osu-skill-pass-4.png",
            id: 58,
            name: "Insanity Approaches",
            grouping: "Skill & Dedication",
            ordering: 4,
            slug: "osu-skill-pass-4",
            description: "You're not twitching, you're just ready.",
            mode: Some("osu"),
            instructions: None
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:47+00:00",
            created_at_alt: "2025-05-11T12:13:47+00:00",
            id: 908876289,
            event_type: "rank",
            user: User {
                username: "Sakyu",
                url: "/u/37745859"
            }
        },
        score_rank: "B",
        rank: 74,
        mode: "osu",
        beatmap: Beatmap {
            title: "polysha feat. Sennzai - Hurt Urge [Tokyo 727's Hard]",
            url: "/b/5054461?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:47+00:00",
            created_at_alt: "2025-05-11T12:13:47+00:00",
            id: 908876288,
            event_type: "rank",
            user: User {
                username: "GxShadow",
                url: "/u/12314891"
            }
        },
        score_rank: "B",
        rank: 930,
        mode: "mania",
        beatmap: Beatmap {
            title: "KURORYU vs. Dispel - Miserea [Peachy vs. Sya's 4K Monochrome]",
            url: "/b/4996035?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:47+00:00",
            created_at_alt: "2025-05-11T12:13:47+00:00",
            id: 908876287,
            event_type: "rank",
            user: User {
                username: "ogoneknumberone",
                url: "/u/16649191"
            }
        },
        score_rank: "S",
        rank: 159,
        mode: "osu",
        beatmap: Beatmap {
            title: "EDGE of LIFE - Believe in Myself (TV Size) [Normal]",
            url: "/b/3519375?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:47+00:00",
            created_at_alt: "2025-05-11T12:13:47+00:00",
            id: 908876286,
            event_type: "achievement",
            user: User {
                username: "bondOzzl",
                url: "/users/37875973"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/taiko-skill-pass-4.png",
            id: 74,
            name: "Face Your Demons",
            grouping: "Skill & Dedication",
            ordering: 4,
            slug: "taiko-skill-pass-4",
            description: "The first trials are now behind you, but are you a match for the Oni?",
            mode: Some("taiko"),
            instructions: None
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:46+00:00",
            created_at_alt: "2025-05-11T12:13:46+00:00",
            id: 908876285,
            event_type: "rank",
            user: User {
                username: "kei1472",
                url: "/u/2337293"
            }
        },
        score_rank: "B",
        rank: 440,
        mode: "mania",
        beatmap: Beatmap {
            title: "ClariS - Koi Sekai (Cut Ver.) [Hard]",
            url: "/b/4564815?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:46+00:00",
            created_at_alt: "2025-05-11T12:13:46+00:00",
            id: 908876284,
            event_type: "rank",
            user: User {
                username: "RSTFpppi",
                url: "/u/37478391"
            }
        },
        score_rank: "C",
        rank: 236,
        mode: "osu",
        beatmap: Beatmap {
            title: "Morfonica - Kakurenbo (Game Ver.) [Kyuuchie's Insane]",
            url: "/b/4972951?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:45+00:00",
            created_at_alt: "2025-05-11T12:13:45+00:00",
            id: 908876283,
            event_type: "rank",
            user: User {
                username: "Sakiiii",
                url: "/u/16776958"
            }
        },
        score_rank: "C",
        rank: 468,
        mode: "osu",
        beatmap: Beatmap {
            title: "Kano - ghost [Hello, \"GoodBye\" Did you hear that?]",
            url: "/b/4608953?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:45+00:00",
            created_at_alt: "2025-05-11T12:13:45+00:00",
            id: 908876282,
            event_type: "rank",
            user: User {
                username: "nekoko0219",
                url: "/u/17169749"
            }
        },
        score_rank: "A",
        rank: 153,
        mode: "osu",
        beatmap: Beatmap {
            title: "biz x ZERA feat. LOLUET - Loveit? [Loveit]",
            url: "/b/4981492?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:45+00:00",
            created_at_alt: "2025-05-11T12:13:45+00:00",
            id: 908876281,
            event_type: "rank",
            user: User {
                username: "snare",
                url: "/u/11636288"
            }
        },
        score_rank: "A",
        rank: 18,
        mode: "osu",
        beatmap: Beatmap {
            title: "monet - Boku no Shiawase [Bloom]",
            url: "/b/5028366?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:44+00:00",
            created_at_alt: "2025-05-11T12:13:44+00:00",
            id: 908876280,
            event_type: "rank",
            user: User {
                username: "enn",
                url: "/u/34262773"
            }
        },
        score_rank: "B",
        rank: 403,
        mode: "osu",
        beatmap: Beatmap {
            title: "Akatsuki Records - LOVE FOR YOU [Extra]",
            url: "/b/2336398?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:43+00:00",
            created_at_alt: "2025-05-11T12:13:43+00:00",
            id: 908876279,
            event_type: "rank",
            user: User {
                username: "artimius06",
                url: "/u/27966801"
            }
        },
        score_rank: "A",
        rank: 24,
        mode: "osu",
        beatmap: Beatmap {
            title: "Within Temptation - Faster (Radio Edit) [Riko's Hard]",
            url: "/b/4956806?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:43+00:00",
            created_at_alt: "2025-05-11T12:13:43+00:00",
            id: 908876278,
            event_type: "rank",
            user: User {
                username: "mj_yahya",
                url: "/u/14013843"
            }
        },
        score_rank: "A",
        rank: 32,
        mode: "mania",
        beatmap: Beatmap {
            title: "YOASOBI - Idol (TV Size) [Mochi's Hard]",
            url: "/b/4124457?m=3"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:40+00:00",
            created_at_alt: "2025-05-11T12:13:40+00:00",
            id: 908876277,
            event_type: "achievement",
            user: User {
                username: "cebx",
                url: "/users/35922718"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/all-secret-youre-here-forever.png",
            id: 302,
            name: "You're Here Forever",
            grouping: "Hush-Hush",
            ordering: 2,
            slug: "all-secret-youre-here-forever",
            description: "We knew you'd be back.",
            mode: None,
            instructions: Some("<i>Touch grass.</i>")
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:40+00:00",
            created_at_alt: "2025-05-11T12:13:40+00:00",
            id: 908876276,
            event_type: "rank",
            user: User {
                username: "infinite story",
                url: "/u/32704040"
            }
        },
        score_rank: "SH",
        rank: 22,
        mode: "fruits",
        beatmap: Beatmap {
            title: "Against The Current - Running With The Wild Things (Sped Up & Cut Ver.) [Visionary's Insane]",
            url: "/b/4276158?m=2"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:40+00:00",
            created_at_alt: "2025-05-11T12:13:40+00:00",
            id: 908876275,
            event_type: "achievement",
            user: User {
                username: "skeleskull",
                url: "/users/37877041"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/mania-skill-pass-1.png",
            id: 87,
            name: "First Steps",
            grouping: "Skill & Dedication",
            ordering: 4,
            slug: "mania-skill-pass-1",
            description: "It isn't 9-to-5, but 1-to-9. Keys, that is.",
            mode: Some("mania"),
            instructions: None
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876274,
            event_type: "rank",
            user: User {
                username: "cocowto",
                url: "/u/10408365"
            }
        },
        score_rank: "C",
        rank: 436,
        mode: "osu",
        beatmap: Beatmap {
            title: "KISIDA KYODAN & THE AKEBOSI ROCKETS - Blood on the EDGE (TV Size) [Nachmark's Hard]",
            url: "/b/4654203?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876273,
            event_type: "rank",
            user: User {
                username: "flon_zon",
                url: "/u/24994595"
            }
        },
        score_rank: "S",
        rank: 63,
        mode: "osu",
        beatmap: Beatmap {
            title: "FUWAMOCO - Kaibutsu [Amats' Hard]",
            url: "/b/4254372?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876272,
            event_type: "achievement",
            user: User {
                username: "14dxin",
                url: "/users/36657401"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/mania-skill-pass-5.png",
            id: 91,
            name: "Ever Onwards",
            grouping: "Skill & Dedication",
            ordering: 4,
            slug: "mania-skill-pass-5",
            description: "Another challenge is just around the corner.",
            mode: Some("mania"),
            instructions: None
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876271,
            event_type: "achievement",
            user: User {
                username: "14dxin",
                url: "/users/36657401"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/mania-secret-meganekko.png",
            id: 54,
            name: "Twin Perspectives",
            grouping: "Hush-Hush",
            ordering: 2,
            slug: "mania-secret-meganekko",
            description: "You met Mani and Mari, our twin osu!mania mascots.",
            mode: Some("mania"),
            instructions: None
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876270,
            event_type: "achievement",
            user: User {
                username: "14dxin",
                url: "/users/36657401"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/mania-hits-40000.png",
            id: 46,
            name: "40,000 Keys",
            grouping: "Skill & Dedication",
            ordering: 3,
            slug: "mania-hits-40000",
            description: "Just the start of the rainbow.",
            mode: Some("mania"),
            instructions: None
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876269,
            event_type: "rank",
            user: User {
                username: "Napeace",
                url: "/u/30833471"
            }
        },
        score_rank: "X",
        rank: 2,
        mode: "mania",
        beatmap: Beatmap {
            title: "Rin'ca - Pleasure garden (Game Ver.) [Nabi's Normal]",
            url: "/b/5056231?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876268,
            event_type: "rank",
            user: User {
                username: "Boss1526",
                url: "/u/37499799"
            }
        },
        score_rank: "SH",
        rank: 378,
        mode: "osu",
        beatmap: Beatmap {
            title: "Pegboard Nerds - Emoji [hackSL's Light Insane]",
            url: "/b/879358?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:39+00:00",
            created_at_alt: "2025-05-11T12:13:39+00:00",
            id: 908876267,
            event_type: "achievement",
            user: User {
                username: "Boss1526",
                url: "/users/37499799"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/osu-combo-750.png",
            id: 3,
            name: "750 Combo",
            grouping: "Skill & Dedication",
            ordering: 0,
            slug: "osu-combo-750",
            description: "750 notes back to back? Woah.",
            mode: Some("osu"),
            instructions: Some("aiming for a combo of 750 or higher on any beatmap")
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:38+00:00",
            created_at_alt: "2025-05-11T12:13:38+00:00",
            id: 908876266,
            event_type: "rank",
            user: User {
                username: "assqwssa",
                url: "/u/16307500"
            }
        },
        score_rank: "B",
        rank: 69,
        mode: "osu",
        beatmap: Beatmap {
            title: "KANA-BOON - Haguruma (TV Size) [HowRengar's Extra]",
            url: "/b/5030701?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:37+00:00",
            created_at_alt: "2025-05-11T12:13:37+00:00",
            id: 908876265,
            event_type: "rank",
            user: User {
                username: "Ballistical",
                url: "/u/32270794"
            }
        },
        score_rank: "X",
        rank: 310,
        mode: "mania",
        beatmap: Beatmap {
            title: "synoxa - just dance is now on xbox game pass (feat. jayceko) [easy]",
            url: "/b/4527727?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:37+00:00",
            created_at_alt: "2025-05-11T12:13:37+00:00",
            id: 908876264,
            event_type: "rank",
            user: User {
                username: "FujiwaraBunta",
                url: "/u/11038247"
            }
        },
        score_rank: "D",
        rank: 42,
        mode: "osu",
        beatmap: Beatmap {
            title: "jioyi attacked by mirror - I lost my flp [Expert]",
            url: "/b/5063067?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:37+00:00",
            created_at_alt: "2025-05-11T12:13:37+00:00",
            id: 908876263,
            event_type: "rank",
            user: User {
                username: "PastelRain",
                url: "/u/15541730"
            }
        },
        score_rank: "A",
        rank: 599,
        mode: "osu",
        beatmap: Beatmap {
            title: "Rohi - Kakuzetsu Thanatos [Collab Extra]",
            url: "/b/2722864?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:36+00:00",
            created_at_alt: "2025-05-11T12:13:36+00:00",
            id: 908876262,
            event_type: "rank",
            user: User {
                username: "SuuperDesu",
                url: "/u/17740599"
            }
        },
        score_rank: "S",
        rank: 393,
        mode: "mania",
        beatmap: Beatmap {
            title: "LeaF - Calamity Fortune [Luminous Zenith]",
            url: "/b/4884644?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:36+00:00",
            created_at_alt: "2025-05-11T12:13:36+00:00",
            id: 908876261,
            event_type: "rank",
            user: User {
                username: "-Martishiru-",
                url: "/u/9790175"
            }
        },
        score_rank: "S",
        rank: 30,
        mode: "fruits",
        beatmap: Beatmap {
            title: "natori - Absolute Zero (TV Size) [Pepti's Overdose]",
            url: "/b/4850663?m=2"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:36+00:00",
            created_at_alt: "2025-05-11T12:13:36+00:00",
            id: 908876260,
            event_type: "rank",
            user: User {
                username: "KOMAGURUMI",
                url: "/u/10885585"
            }
        },
        score_rank: "B",
        rank: 170,
        mode: "osu",
        beatmap: Beatmap {
            title: "Maeda Jun x yanaginagi - Bougainvillea [Collab]",
            url: "/b/4957881?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:36+00:00",
            created_at_alt: "2025-05-11T12:13:36+00:00",
            id: 908876259,
            event_type: "rank",
            user: User {
                username: "melt2358",
                url: "/u/35901118"
            }
        },
        score_rank: "C",
        rank: 48,
        mode: "osu",
        beatmap: Beatmap {
            title: "turbo - Faris ga Kakkakka [Normal-nyan]",
            url: "/b/4876742?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:36+00:00",
            created_at_alt: "2025-05-11T12:13:36+00:00",
            id: 908876258,
            event_type: "rank",
            user: User {
                username: "kunpo",
                url: "/u/18605562"
            }
        },
        score_rank: "B",
        rank: 157,
        mode: "osu",
        beatmap: Beatmap {
            title: "Maeda Jun x yanaginagi - Bougainvillea [Collab]",
            url: "/b/4957881?m=0"
        }
    }, Achievement {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:35+00:00",
            created_at_alt: "2025-05-11T12:13:35+00:00",
            id: 908876257,
            event_type: "achievement",
            user: User {
                username: "Chinchopaz",
                url: "/users/33629660"
            }
        },
        achievement: Achievement {
            icon_url: "https://assets.ppy.sh/medals/web/osu-skill-pass-2.png",
            id: 56,
            name: "Constellation Prize",
            grouping: "Skill & Dedication",
            ordering: 4,
            slug: "osu-skill-pass-2",
            description: "Definitely not a consolation prize. Now things start getting hard!",
            mode: Some("osu"),
            instructions: None
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:33+00:00",
            created_at_alt: "2025-05-11T12:13:33+00:00",
            id: 908876256,
            event_type: "rank",
            user: User {
                username: "Eggd",
                url: "/u/21468503"
            }
        },
        score_rank: "C",
        rank: 993,
        mode: "osu",
        beatmap: Beatmap {
            title: "UNDEAD CORPORATION - MEGALOMANIA [YUMERIOS' ULTRA]",
            url: "/b/4862041?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:33+00:00",
            created_at_alt: "2025-05-11T12:13:33+00:00",
            id: 908876255,
            event_type: "rank",
            user: User {
                username: "Nallybtw",
                url: "/u/31928445"
            }
        },
        score_rank: "A",
        rank: 432,
        mode: "taiko",
        beatmap: Beatmap {
            title: "Foreground Eclipse - From Under Cover (Caught Up In A Love Song) [Akitoshi's Normal]",
            url: "/b/1084459?m=1"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:33+00:00",
            created_at_alt: "2025-05-11T12:13:33+00:00",
            id: 908876254,
            event_type: "rank",
            user: User {
                username: "Kolo71",
                url: "/u/4648329"
            }
        },
        score_rank: "SH",
        rank: 136,
        mode: "mania",
        beatmap: Beatmap {
            title: "nano.RIPE - Azalea [Normal]",
            url: "/b/1526563?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:33+00:00",
            created_at_alt: "2025-05-11T12:13:33+00:00",
            id: 908876253,
            event_type: "rank",
            user: User {
                username: "Zurasho",
                url: "/u/12207539"
            }
        },
        score_rank: "B",
        rank: 477,
        mode: "mania",
        beatmap: Beatmap {
            title: "Camellia feat. Nanahira - Bassdrop Freaks (2018 \"Redrop\" ver.) [Hard (old SV ver.)]",
            url: "/b/2025494?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:31+00:00",
            created_at_alt: "2025-05-11T12:13:31+00:00",
            id: 908876252,
            event_type: "rank",
            user: User {
                username: "BATUKAM",
                url: "/u/34607408"
            }
        },
        score_rank: "S",
        rank: 955,
        mode: "mania",
        beatmap: Beatmap {
            title: "Aoi (as Midnight Shift) - Magia omnia of blue sonus feat. GUMI SV [Hyper]",
            url: "/b/4850498?m=3"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:29+00:00",
            created_at_alt: "2025-05-11T12:13:29+00:00",
            id: 908876251,
            event_type: "rank",
            user: User {
                username: "K u R o C h i",
                url: "/u/4278885"
            }
        },
        score_rank: "A",
        rank: 555,
        mode: "osu",
        beatmap: Beatmap {
            title: "Nanahira - Chikatto Chika Chika [O K U's Insane]",
            url: "/b/4884621?m=0"
        }
    }, Rank {
        base: BaseEvent {
            created_at: "2025-05-11T12:13:29+00:00",
            created_at_alt: "2025-05-11T12:13:29+00:00",
            id: 908876250,
            event_type: "rank",
            user: User {
                username: "thefruss032",
                url: "/u/13215465"
            }
        },
        score_rank: "S",
        rank: 16,
        mode: "mania",
        beatmap: Beatmap {
            title: "Rin'ca - Pleasure garden (Game Ver.) [Nabi's Normal]",
            url: "/b/5056231?m=3"
        }
    }],
    cursor: Cursor {
        event_id: 908876250
    },
    cursor_string: "eyJldmVudF9pZCI6OTA4ODc2MjUwfQ"
}
*/
