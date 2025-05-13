// Get Beatmap Packs
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmaps::IBeatmaps;
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
    let packs = client.beatmaps.get_beatmap_packs(None, None).await?;
    println!("{:?}", packs);

    Ok(())
}

/*
ReqwestBeatmaps get_beatmap_packs
BeatmapPacks {
    beatmap_packs: [BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-12T05:46:22.000000Z",
        name: "osu! Beatmap Pack #1623",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1623",
        url: "https://packs.ppy.sh/S1623%20-%20osu%21%20Beatmap%20Pack%20%231623.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-12T05:45:41.000000Z",
        name: "osu! Beatmap Pack #1622",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1622",
        url: "https://packs.ppy.sh/S1622%20-%20osu%21%20Beatmap%20Pack%20%231622.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-11T14:52:35.000000Z",
        name: "osu!taiko Beatmap Pack #338",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST338",
        url: "https://packs.ppy.sh/ST338%20-%20osu%21taiko%20Beatmap%20Pack%20%23338.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-11T14:51:57.000000Z",
        name: "osu! Beatmap Pack #1621",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1621",
        url: "https://packs.ppy.sh/S1621%20-%20osu%21%20Beatmap%20Pack%20%231621.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-09T09:22:52.000000Z",
        name: "osu!mania Beatmap Pack #285",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM285",
        url: "https://packs.ppy.sh/SM285%20-%20osu%21mania%20Beatmap%20Pack%20%23285.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-08T10:43:44.000000Z",
        name: "osu! Beatmap Pack #1620",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1620",
        url: "https://packs.ppy.sh/S1620%20-%20osu%21%20Beatmap%20Pack%20%231620.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-07T01:54:15.000000Z",
        name: "osu! Beatmap Pack #1619",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1619",
        url: "https://packs.ppy.sh/S1619%20-%20osu%21%20Beatmap%20Pack%20%231619.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:59:30.000000Z",
        name: "osu!mania Beatmap Pack #284",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM284",
        url: "https://packs.ppy.sh/SM284%20-%20osu%21mania%20Beatmap%20Pack%20%23284.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:58:46.000000Z",
        name: "osu! Beatmap Pack #1618",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1618",
        url: "https://packs.ppy.sh/S1618%20-%20osu%21%20Beatmap%20Pack%20%231618.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:57:45.000000Z",
        name: "osu!taiko Beatmap Pack #337",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST337",
        url: "https://packs.ppy.sh/ST337%20-%20osu%21taiko%20Beatmap%20Pack%20%23337.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:55:25.000000Z",
        name: "osu! Beatmap Pack #1617",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1617",
        url: "https://packs.ppy.sh/S1617%20-%20osu%21%20Beatmap%20Pack%20%231617.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:54:33.000000Z",
        name: "osu!catch Beatmap Pack #126",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC126",
        url: "https://packs.ppy.sh/SC126%20-%20osu%21catch%20Beatmap%20Pack%20%23126.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:30:36.000000Z",
        name: "osu! Beatmap Pack #1616",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1616",
        url: "https://packs.ppy.sh/S1616%20-%20osu%21%20Beatmap%20Pack%20%231616.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:29:49.000000Z",
        name: "osu!mania Beatmap Pack #283",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM283",
        url: "https://packs.ppy.sh/SM283%20-%20osu%21mania%20Beatmap%20Pack%20%23283.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:28:56.000000Z",
        name: "osu! Beatmap Pack #1615",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1615",
        url: "https://packs.ppy.sh/S1615%20-%20osu%21%20Beatmap%20Pack%20%231615.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-06T22:27:51.000000Z",
        name: "osu! Beatmap Pack #1614",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1614",
        url: "https://packs.ppy.sh/S1614%20-%20osu%21%20Beatmap%20Pack%20%231614.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-05-03T05:39:07.000000Z",
        name: "osu!catch Beatmap Pack #125",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC125",
        url: "https://packs.ppy.sh/SC125%20-%20osu%21catch%20Beatmap%20Pack%20%23125.zip?1746570349",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-30T04:28:42.000000Z",
        name: "osu! Beatmap Pack #1613",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1613",
        url: "https://packs.ppy.sh/S1613%20-%20osu%21%20Beatmap%20Pack%20%231613.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-28T03:35:18.000000Z",
        name: "osu! Beatmap Pack #1612",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1612",
        url: "https://packs.ppy.sh/S1612%20-%20osu%21%20Beatmap%20Pack%20%231612.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-28T03:34:30.000000Z",
        name: "osu!mania Beatmap Pack #282",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM282",
        url: "https://packs.ppy.sh/SM282%20-%20osu%21mania%20Beatmap%20Pack%20%23282.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-27T02:03:34.000000Z",
        name: "osu! Beatmap Pack #1611",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1611",
        url: "https://packs.ppy.sh/S1611%20-%20osu%21%20Beatmap%20Pack%20%231611.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-26T09:53:34.000000Z",
        name: "osu!taiko Beatmap Pack #336",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST336",
        url: "https://packs.ppy.sh/ST336%20-%20osu%21taiko%20Beatmap%20Pack%20%23336.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-26T09:48:50.000000Z",
        name: "osu! Beatmap Pack #1610",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1610",
        url: "https://packs.ppy.sh/S1610%20-%20osu%21%20Beatmap%20Pack%20%231610.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-26T09:35:05.000000Z",
        name: "osu! Beatmap Pack #1609",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1609",
        url: "https://packs.ppy.sh/S1609%20-%20osu%21%20Beatmap%20Pack%20%231609.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-24T16:09:17.000000Z",
        name: "osu! Beatmap Pack #1608",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1608",
        url: "https://packs.ppy.sh/S1608%20-%20osu%21%20Beatmap%20Pack%20%231608.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-24T16:08:26.000000Z",
        name: "osu! Beatmap Pack #1607",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1607",
        url: "https://packs.ppy.sh/S1607%20-%20osu%21%20Beatmap%20Pack%20%231607.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-24T16:07:11.000000Z",
        name: "osu! Beatmap Pack #1606",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1606",
        url: "https://packs.ppy.sh/S1606%20-%20osu%21%20Beatmap%20Pack%20%231606.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-24T16:06:01.000000Z",
        name: "osu!mania Beatmap Pack #281",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM281",
        url: "https://packs.ppy.sh/SM281%20-%20osu%21mania%20Beatmap%20Pack%20%23281.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-20T08:17:13.000000Z",
        name: "osu!catch Beatmap Pack #124",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC124",
        url: "https://packs.ppy.sh/SC124%20-%20osu%21catch%20Beatmap%20Pack%20%23124.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-20T08:15:03.000000Z",
        name: "osu! Beatmap Pack #1605",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1605",
        url: "https://packs.ppy.sh/S1605%20-%20osu%21%20Beatmap%20Pack%20%231605.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-18T10:50:29.000000Z",
        name: "osu!taiko Beatmap Pack #335",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST335",
        url: "https://packs.ppy.sh/ST335%20-%20osu%21taiko%20Beatmap%20Pack%20%23335.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-18T10:49:35.000000Z",
        name: "osu! Beatmap Pack #1604",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1604",
        url: "https://packs.ppy.sh/S1604%20-%20osu%21%20Beatmap%20Pack%20%231604.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-16T01:03:18.000000Z",
        name: "osu!mania Beatmap Pack #280",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM280",
        url: "https://packs.ppy.sh/SM280%20-%20osu%21mania%20Beatmap%20Pack%20%23280.zip?1744765585",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-16T00:29:25.000000Z",
        name: "osu! Beatmap Pack #1603",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1603",
        url: "https://packs.ppy.sh/S1603%20-%20osu%21%20Beatmap%20Pack%20%231603.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-13T07:32:21.000000Z",
        name: "osu! Beatmap Pack #1602",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1602",
        url: "https://packs.ppy.sh/S1602%20-%20osu%21%20Beatmap%20Pack%20%231602.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-11T16:05:40.000000Z",
        name: "osu! Beatmap Pack #1601",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1601",
        url: "https://packs.ppy.sh/S1601%20-%20osu%21%20Beatmap%20Pack%20%231601.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-09T00:37:40.000000Z",
        name: "osu!taiko Beatmap Pack #334",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST334",
        url: "https://packs.ppy.sh/ST334%20-%20osu%21taiko%20Beatmap%20Pack%20%23334.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-09T00:36:52.000000Z",
        name: "osu! Beatmap Pack #1600",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1600",
        url: "https://packs.ppy.sh/S1600%20-%20osu%21%20Beatmap%20Pack%20%231600.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-09T00:35:55.000000Z",
        name: "osu! Beatmap Pack #1599",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1599",
        url: "https://packs.ppy.sh/S1599%20-%20osu%21%20Beatmap%20Pack%20%231599.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-09T00:34:17.000000Z",
        name: "osu!mania Beatmap Pack #279",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM279",
        url: "https://packs.ppy.sh/SM279%20-%20osu%21mania%20Beatmap%20Pack%20%23279.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-06T14:13:07.000000Z",
        name: "osu! Beatmap Pack #1598",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1598",
        url: "https://packs.ppy.sh/S1598%20-%20osu%21%20Beatmap%20Pack%20%231598.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-04T06:09:33.000000Z",
        name: "osu!taiko Beatmap Pack #333",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST333",
        url: "https://packs.ppy.sh/ST333%20-%20osu%21taiko%20Beatmap%20Pack%20%23333.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-04T06:08:31.000000Z",
        name: "osu! Beatmap Pack #1597",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1597",
        url: "https://packs.ppy.sh/S1597%20-%20osu%21%20Beatmap%20Pack%20%231597.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-04T02:10:18.000000Z",
        name: "osu! Beatmap Pack #1596",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1596",
        url: "https://packs.ppy.sh/S1596%20-%20osu%21%20Beatmap%20Pack%20%231596.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-04-02T12:33:03.000000Z",
        name: "osu! Beatmap Pack #1595",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1595",
        url: "https://packs.ppy.sh/S1595%20-%20osu%21%20Beatmap%20Pack%20%231595.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-31T01:05:35.000000Z",
        name: "osu!mania Beatmap Pack #278",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM278",
        url: "https://packs.ppy.sh/SM278%20-%20osu%21mania%20Beatmap%20Pack%20%23278.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-31T01:04:08.000000Z",
        name: "osu! Beatmap Pack #1594",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1594",
        url: "https://packs.ppy.sh/S1594%20-%20osu%21%20Beatmap%20Pack%20%231594.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-28T15:22:52.000000Z",
        name: "osu!taiko Beatmap Pack #332",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST332",
        url: "https://packs.ppy.sh/ST332%20-%20osu%21taiko%20Beatmap%20Pack%20%23332.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-28T15:14:11.000000Z",
        name: "osu! Beatmap Pack #1593",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1593",
        url: "https://packs.ppy.sh/S1593%20-%20osu%21%20Beatmap%20Pack%20%231593.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-28T15:12:37.000000Z",
        name: "osu! Beatmap Pack #1592",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1592",
        url: "https://packs.ppy.sh/S1592%20-%20osu%21%20Beatmap%20Pack%20%231592.zip?1743174799",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-27T04:20:56.000000Z",
        name: "osu!catch Beatmap Pack #123",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC123",
        url: "https://packs.ppy.sh/SC123%20-%20osu%21catch%20Beatmap%20Pack%20%23123.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-26T13:01:31.000000Z",
        name: "osu!mania Beatmap Pack #277",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM277",
        url: "https://packs.ppy.sh/SM277%20-%20osu%21mania%20Beatmap%20Pack%20%23277.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-26T13:00:39.000000Z",
        name: "osu! Beatmap Pack #1591",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1591",
        url: "https://packs.ppy.sh/S1591%20-%20osu%21%20Beatmap%20Pack%20%231591.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-26T12:59:46.000000Z",
        name: "osu!taiko Beatmap Pack #331",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST331",
        url: "https://packs.ppy.sh/ST331%20-%20osu%21taiko%20Beatmap%20Pack%20%23331.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-26T12:59:09.000000Z",
        name: "osu! Beatmap Pack #1590",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1590",
        url: "https://packs.ppy.sh/S1590%20-%20osu%21%20Beatmap%20Pack%20%231590.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-24T04:28:04.000000Z",
        name: "osu!mania Beatmap Pack #276",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM276",
        url: "https://packs.ppy.sh/SM276%20-%20osu%21mania%20Beatmap%20Pack%20%23276.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-23T05:48:31.000000Z",
        name: "osu! Beatmap Pack #1589",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1589",
        url: "https://packs.ppy.sh/S1589%20-%20osu%21%20Beatmap%20Pack%20%231589.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-20T09:37:51.000000Z",
        name: "osu! Beatmap Pack #1588",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1588",
        url: "https://packs.ppy.sh/S1588%20-%20osu%21%20Beatmap%20Pack%20%231588.zip?1742465411",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-20T09:36:08.000000Z",
        name: "osu!taiko Beatmap Pack #330",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST330",
        url: "https://packs.ppy.sh/ST330%20-%20osu%21taiko%20Beatmap%20Pack%20%23330.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-19T04:44:33.000000Z",
        name: "osu!mania Beatmap Pack #275",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM275",
        url: "https://packs.ppy.sh/SM275%20-%20osu%21mania%20Beatmap%20Pack%20%23275.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-19T03:18:03.000000Z",
        name: "osu! Beatmap Pack #1587",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1587",
        url: "https://packs.ppy.sh/S1587%20-%20osu%21%20Beatmap%20Pack%20%231587.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-17T12:02:37.000000Z",
        name: "osu! Beatmap Pack #1586",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1586",
        url: "https://packs.ppy.sh/S1586%20-%20osu%21%20Beatmap%20Pack%20%231586.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-14T15:25:04.000000Z",
        name: "osu! Beatmap Pack #1585",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1585",
        url: "https://packs.ppy.sh/S1585%20-%20osu%21%20Beatmap%20Pack%20%231585.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-13T05:33:47.000000Z",
        name: "osu!mania Beatmap Pack #274",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM274",
        url: "https://packs.ppy.sh/SM274%20-%20osu%21mania%20Beatmap%20Pack%20%23274.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-13T05:33:06.000000Z",
        name: "osu! Beatmap Pack #1584",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1584",
        url: "https://packs.ppy.sh/S1584%20-%20osu%21%20Beatmap%20Pack%20%231584.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-11T00:39:39.000000Z",
        name: "osu! Beatmap Pack #1583",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1583",
        url: "https://packs.ppy.sh/S1583%20-%20osu%21%20Beatmap%20Pack%20%231583.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-11T00:39:05.000000Z",
        name: "osu!taiko Beatmap Pack #329",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST329",
        url: "https://packs.ppy.sh/ST329%20-%20osu%21taiko%20Beatmap%20Pack%20%23329.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-08T10:08:25.000000Z",
        name: "osu! Beatmap Pack #1582",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1582",
        url: "https://packs.ppy.sh/S1582%20-%20osu%21%20Beatmap%20Pack%20%231582.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-08T10:07:40.000000Z",
        name: "osu!catch Beatmap Pack #122",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC122",
        url: "https://packs.ppy.sh/SC122%20-%20osu%21catch%20Beatmap%20Pack%20%23122.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-08T10:06:59.000000Z",
        name: "osu! Beatmap Pack #1581",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1581",
        url: "https://packs.ppy.sh/S1581%20-%20osu%21%20Beatmap%20Pack%20%231581.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-08T10:06:21.000000Z",
        name: "osu!mania Beatmap Pack #273",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM273",
        url: "https://packs.ppy.sh/SM273%20-%20osu%21mania%20Beatmap%20Pack%20%23273.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-04T01:24:19.000000Z",
        name: "osu! Beatmap Pack #1580",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1580",
        url: "https://packs.ppy.sh/S1580%20-%20osu%21%20Beatmap%20Pack%20%231580.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-04T01:18:55.000000Z",
        name: "osu!taiko Beatmap Pack #328",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST328",
        url: "https://packs.ppy.sh/ST328%20-%20osu%21taiko%20Beatmap%20Pack%20%23328.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-03T00:12:46.000000Z",
        name: "osu! Beatmap Pack #1579",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1579",
        url: "https://packs.ppy.sh/S1579%20-%20osu%21%20Beatmap%20Pack%20%231579.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-03-02T10:55:07.000000Z",
        name: "osu!mania Beatmap Pack #272",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM272",
        url: "https://packs.ppy.sh/SM272%20-%20osu%21mania%20Beatmap%20Pack%20%23272.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-28T16:10:01.000000Z",
        name: "osu!taiko Beatmap Pack #327",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST327",
        url: "https://packs.ppy.sh/ST327%20-%20osu%21taiko%20Beatmap%20Pack%20%23327.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-28T16:09:09.000000Z",
        name: "osu! Beatmap Pack #1578",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1578",
        url: "https://packs.ppy.sh/S1578%20-%20osu%21%20Beatmap%20Pack%20%231578.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-25T08:56:38.000000Z",
        name: "osu! Beatmap Pack #1577",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1577",
        url: "https://packs.ppy.sh/S1577%20-%20osu%21%20Beatmap%20Pack%20%231577.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-24T10:11:06.000000Z",
        name: "osu! Beatmap Pack #1576",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1576",
        url: "https://packs.ppy.sh/S1576%20-%20osu%21%20Beatmap%20Pack%20%231576.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-23T09:22:18.000000Z",
        name: "osu!taiko Beatmap Pack #326",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST326",
        url: "https://packs.ppy.sh/ST326%20-%20osu%21taiko%20Beatmap%20Pack%20%23326.zip?1740302582",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-22T06:19:28.000000Z",
        name: "osu!mania Beatmap Pack #271",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM271",
        url: "https://packs.ppy.sh/SM271%20-%20osu%21mania%20Beatmap%20Pack%20%23271.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-22T06:16:02.000000Z",
        name: "osu! Beatmap Pack #1575",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1575",
        url: "https://packs.ppy.sh/S1575%20-%20osu%21%20Beatmap%20Pack%20%231575.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-20T11:20:03.000000Z",
        name: "osu!catch Beatmap Pack #121",
        no_diff_reduction: false,
        ruleset_id: Some(2),
        tag: "SC121",
        url: "https://packs.ppy.sh/SC121%20-%20osu%21catch%20Beatmap%20Pack%20%23121.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-20T11:14:59.000000Z",
        name: "osu! Beatmap Pack #1574",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1574",
        url: "https://packs.ppy.sh/S1574%20-%20osu%21%20Beatmap%20Pack%20%231574.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-20T11:12:04.000000Z",
        name: "osu!taiko Beatmap Pack #325",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST325",
        url: "https://packs.ppy.sh/ST325%20-%20osu%21taiko%20Beatmap%20Pack%20%23325.zip?1740050151",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-18T00:52:45.000000Z",
        name: "osu! Beatmap Pack #1573",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1573",
        url: "https://packs.ppy.sh/S1573%20-%20osu%21%20Beatmap%20Pack%20%231573.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-18T00:51:25.000000Z",
        name: "osu!mania Beatmap Pack #270",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM270",
        url: "https://packs.ppy.sh/SM270%20-%20osu%21mania%20Beatmap%20Pack%20%23270.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-16T08:38:48.000000Z",
        name: "osu!taiko Beatmap Pack #324",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST324",
        url: "https://packs.ppy.sh/ST324%20-%20osu%21taiko%20Beatmap%20Pack%20%23324.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-15T03:34:58.000000Z",
        name: "osu! Beatmap Pack #1572",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1572",
        url: "https://packs.ppy.sh/S1572%20-%20osu%21%20Beatmap%20Pack%20%231572.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-13T15:25:45.000000Z",
        name: "osu! Beatmap Pack #1571",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1571",
        url: "https://packs.ppy.sh/S1571%20-%20osu%21%20Beatmap%20Pack%20%231571.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-13T15:25:01.000000Z",
        name: "osu! Beatmap Pack #1570",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1570",
        url: "https://packs.ppy.sh/S1570%20-%20osu%21%20Beatmap%20Pack%20%231570.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-12T10:51:21.000000Z",
        name: "osu!mania Beatmap Pack #269",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM269",
        url: "https://packs.ppy.sh/SM269%20-%20osu%21mania%20Beatmap%20Pack%20%23269.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-12T10:50:49.000000Z",
        name: "osu!taiko Beatmap Pack #323",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST323",
        url: "https://packs.ppy.sh/ST323%20-%20osu%21taiko%20Beatmap%20Pack%20%23323.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-10T22:56:41.000000Z",
        name: "osu! Beatmap Pack #1569",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1569",
        url: "https://packs.ppy.sh/S1569%20-%20osu%21%20Beatmap%20Pack%20%231569.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-09T12:06:42.000000Z",
        name: "osu! Beatmap Pack #1568",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1568",
        url: "https://packs.ppy.sh/S1568%20-%20osu%21%20Beatmap%20Pack%20%231568.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-07T04:31:41.000000Z",
        name: "osu!mania Beatmap Pack #268",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM268",
        url: "https://packs.ppy.sh/SM268%20-%20osu%21mania%20Beatmap%20Pack%20%23268.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-06T00:10:23.000000Z",
        name: "osu!taiko Beatmap Pack #322",
        no_diff_reduction: false,
        ruleset_id: Some(1),
        tag: "ST322",
        url: "https://packs.ppy.sh/ST322%20-%20osu%21taiko%20Beatmap%20Pack%20%23322.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-05T10:53:21.000000Z",
        name: "osu! Beatmap Pack #1567",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1567",
        url: "https://packs.ppy.sh/S1567%20-%20osu%21%20Beatmap%20Pack%20%231567.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-04T02:47:36.000000Z",
        name: "osu! Beatmap Pack #1566",
        no_diff_reduction: false,
        ruleset_id: None,
        tag: "S1566",
        url: "https://packs.ppy.sh/S1566%20-%20osu%21%20Beatmap%20Pack%20%231566.zip",
        beatmapsets: None,
        user_completion_data: None
    }, BeatmapPack {
        author: "Shige-Tori[a]",
        date: "2025-02-01T08:35:53.000000Z",
        name: "osu!mania Beatmap Pack #267",
        no_diff_reduction: false,
        ruleset_id: Some(3),
        tag: "SM267",
        url: "https://packs.ppy.sh/SM267%20-%20osu%21mania%20Beatmap%20Pack%20%23267.zip",
        beatmapsets: None,
        user_completion_data: None
    }]
}
*/
