// Get Spotlights
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::ranking::IRanking;
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
    let spotlights = client.ranking.get_spotlights().await?;
    println!("{:?}", spotlights);
    Ok(())
}

/*
ReqwestRanking get_spotlights
Spotlights {
    spotlights: [Spotlight {
        end_date: "2020-06-25T00:00:00+00:00",
        id: 276,
        mode_specific: true,
        name: "Seasonal Spotlights: Winter 2020",
        participant_count: None,
        start_date: "2020-04-09T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2020-03-15T00:00:00+00:00",
        id: 271,
        mode_specific: true,
        name: "Seasonal Spotlights: Autumn 2019",
        participant_count: None,
        start_date: "2020-01-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2019-12-15T00:00:00+00:00",
        id: 267,
        mode_specific: true,
        name: "Seasonal Spotlights: Summer 2019",
        participant_count: None,
        start_date: "2019-10-05T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2019-09-15T00:00:00+00:00",
        id: 263,
        mode_specific: true,
        name: "Seasonal Spotlights: Spring 2019",
        participant_count: None,
        start_date: "2019-07-07T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2019-06-25T00:00:00+00:00",
        id: 259,
        mode_specific: true,
        name: "Seasonal Spotlights: Winter 2019",
        participant_count: None,
        start_date: "2019-04-10T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2019-03-15T00:00:00+00:00",
        id: 255,
        mode_specific: true,
        name: "Seasonal Spotlights: Fall 2018",
        participant_count: None,
        start_date: "2019-01-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-12-15T00:00:00+00:00",
        id: 251,
        mode_specific: true,
        name: "Seasonal Spotlights: Summer 2018",
        participant_count: None,
        start_date: "2018-11-02T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-06-30T00:00:00+00:00",
        id: 250,
        mode_specific: true,
        name: "Camellia Themed Spotlights",
        participant_count: None,
        start_date: "2018-05-05T00:00:00+00:00",
        spotlight_type: "spotlight"
    }, Spotlight {
        end_date: "2018-05-28T00:00:00+00:00",
        id: 246,
        mode_specific: true,
        name: "Spotlights March 2018",
        participant_count: None,
        start_date: "2018-04-28T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-04-30T00:00:00+00:00",
        id: 245,
        mode_specific: true,
        name: "Anime Themed Spotlights 2018",
        participant_count: None,
        start_date: "2018-03-29T00:00:00+00:00",
        spotlight_type: "spotlight"
    }, Spotlight {
        end_date: "2018-04-15T00:00:00+00:00",
        id: 240,
        mode_specific: true,
        name: "Spotlights February 2018",
        participant_count: None,
        start_date: "2018-03-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-03-15T00:00:00+00:00",
        id: 235,
        mode_specific: true,
        name: "Spotlights January 2018",
        participant_count: None,
        start_date: "2018-02-23T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-02-15T00:00:00+00:00",
        id: 231,
        mode_specific: true,
        name: "Spotlights December 2017",
        participant_count: None,
        start_date: "2018-01-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2018-01-31T00:00:00+00:00",
        id: 227,
        mode_specific: true,
        name: "Winter Themed Spotlights 2017",
        participant_count: None,
        start_date: "2017-12-24T00:00:00+00:00",
        spotlight_type: "spotlight"
    }, Spotlight {
        end_date: "2018-01-15T00:00:00+00:00",
        id: 223,
        mode_specific: true,
        name: "Spotlights November 2017",
        participant_count: None,
        start_date: "2017-12-19T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-12-15T00:00:00+00:00",
        id: 219,
        mode_specific: true,
        name: "Spotlights October 2017",
        participant_count: None,
        start_date: "2017-11-29T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-11-20T00:00:00+00:00",
        id: 218,
        mode_specific: true,
        name: "Spotlights September 2017",
        participant_count: None,
        start_date: "2017-11-09T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-10-15T00:00:00+00:00",
        id: 214,
        mode_specific: true,
        name: "Spotlights July/August 2017",
        participant_count: None,
        start_date: "2017-09-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-08-15T00:00:00+00:00",
        id: 207,
        mode_specific: true,
        name: "Spotlights June 2017",
        participant_count: None,
        start_date: "2017-08-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-07-15T00:00:00+00:00",
        id: 203,
        mode_specific: true,
        name: "Spotlights May 2017",
        participant_count: None,
        start_date: "2017-07-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-06-15T00:00:00+00:00",
        id: 199,
        mode_specific: true,
        name: "Spotlights April 2017",
        participant_count: None,
        start_date: "2017-05-28T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-05-15T00:00:00+00:00",
        id: 195,
        mode_specific: true,
        name: "Spotlights March 2017",
        participant_count: None,
        start_date: "2017-04-20T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-04-15T00:00:00+00:00",
        id: 191,
        mode_specific: true,
        name: "Spotlights January/February 2017",
        participant_count: None,
        start_date: "2017-03-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-02-15T00:00:00+00:00",
        id: 187,
        mode_specific: true,
        name: "Monthly Ranking December 2016",
        participant_count: None,
        start_date: "2017-01-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2017-01-31T00:00:00+00:00",
        id: 183,
        mode_specific: true,
        name: "Themed Chart: Winter/Christmas 2016",
        participant_count: None,
        start_date: "2016-12-24T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2017-01-15T00:00:00+00:00",
        id: 179,
        mode_specific: true,
        name: "Monthly Ranking November 2016",
        participant_count: None,
        start_date: "2016-12-19T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-12-20T00:00:00+00:00",
        id: 175,
        mode_specific: true,
        name: "Monthly Ranking October 2016",
        participant_count: None,
        start_date: "2016-12-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-11-12T00:00:00+00:00",
        id: 174,
        mode_specific: false,
        name: "Halloween Spotlight 2016",
        participant_count: None,
        start_date: "2016-10-29T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-11-15T00:00:00+00:00",
        id: 170,
        mode_specific: true,
        name: "Monthly Ranking September 2016",
        participant_count: None,
        start_date: "2016-10-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-10-30T00:00:00+00:00",
        id: 166,
        mode_specific: true,
        name: "Monthly Ranking August 2016",
        participant_count: None,
        start_date: "2016-10-09T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-10-05T00:00:00+00:00",
        id: 162,
        mode_specific: true,
        name: "Monthly Ranking July 2016",
        participant_count: None,
        start_date: "2016-09-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-08-20T00:00:00+00:00",
        id: 158,
        mode_specific: true,
        name: "Monthly Ranking June 2016",
        participant_count: None,
        start_date: "2016-07-27T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-07-15T00:00:00+00:00",
        id: 154,
        mode_specific: true,
        name: "Monthly Ranking May 2016",
        participant_count: None,
        start_date: "2016-06-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-06-15T00:00:00+00:00",
        id: 150,
        mode_specific: true,
        name: "Monthly Ranking April 2016",
        participant_count: None,
        start_date: "2016-05-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-05-15T00:00:00+00:00",
        id: 146,
        mode_specific: true,
        name: "Monthly Ranking March 2016",
        participant_count: None,
        start_date: "2016-04-27T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-04-15T00:00:00+00:00",
        id: 142,
        mode_specific: true,
        name: "Monthly Ranking February 2016",
        participant_count: None,
        start_date: "2016-03-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-03-15T00:00:00+00:00",
        id: 138,
        mode_specific: true,
        name: "Monthly Ranking January 2016",
        participant_count: None,
        start_date: "2016-02-27T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-02-20T00:00:00+00:00",
        id: 134,
        mode_specific: true,
        name: "December 2015 Monthly Ranking Charts",
        participant_count: None,
        start_date: "2016-01-24T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2016-01-15T00:00:00+00:00",
        id: 130,
        mode_specific: true,
        name: "Monthly Ranking Chart November 2015",
        participant_count: None,
        start_date: "2015-12-19T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-12-15T00:00:00+00:00",
        id: 126,
        mode_specific: true,
        name: "Monthly Ranking October 2015",
        participant_count: None,
        start_date: "2015-11-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-11-15T00:00:00+00:00",
        id: 122,
        mode_specific: true,
        name: "Monthly Ranking September",
        participant_count: None,
        start_date: "2015-10-20T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-10-15T00:00:00+00:00",
        id: 121,
        mode_specific: true,
        name: "Monthly Ranking August 2015",
        participant_count: None,
        start_date: "2015-09-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-09-15T00:00:00+00:00",
        id: 117,
        mode_specific: true,
        name: "Monthly Ranking July 2015",
        participant_count: None,
        start_date: "2015-08-20T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-08-15T00:00:00+00:00",
        id: 113,
        mode_specific: true,
        name: "Monthly Ranking June 2015",
        participant_count: None,
        start_date: "2015-07-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-07-15T00:00:00+00:00",
        id: 109,
        mode_specific: true,
        name: "Monthly Ranking May 2015",
        participant_count: None,
        start_date: "2015-06-23T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-06-15T00:00:00+00:00",
        id: 108,
        mode_specific: true,
        name: "Monthly Ranking April 2015",
        participant_count: None,
        start_date: "2015-05-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-05-15T00:00:00+00:00",
        id: 107,
        mode_specific: true,
        name: "Monthly Ranking March 2015",
        participant_count: None,
        start_date: "2015-04-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-04-15T00:00:00+00:00",
        id: 106,
        mode_specific: true,
        name: "Monthly Ranking February 2015",
        participant_count: None,
        start_date: "2015-03-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-03-15T00:00:00+00:00",
        id: 105,
        mode_specific: true,
        name: "Monthly Ranking January 2015",
        participant_count: None,
        start_date: "2015-02-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-03-01T00:00:00+00:00",
        id: 103,
        mode_specific: true,
        name: "Best of 2014",
        participant_count: None,
        start_date: "2015-02-01T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2015-02-15T00:00:00+00:00",
        id: 99,
        mode_specific: true,
        name: "Monthly Ranking December 2014",
        participant_count: None,
        start_date: "2015-01-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2015-01-15T00:00:00+00:00",
        id: 97,
        mode_specific: true,
        name: "Monthly Ranking November 2014",
        participant_count: None,
        start_date: "2014-12-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-12-15T00:00:00+00:00",
        id: 96,
        mode_specific: true,
        name: "Monthly Ranking October 2014",
        participant_count: None,
        start_date: "2014-11-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-11-15T00:00:00+00:00",
        id: 95,
        mode_specific: true,
        name: "Monthly Ranking September 2014",
        participant_count: None,
        start_date: "2014-10-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-10-15T00:00:00+00:00",
        id: 93,
        mode_specific: true,
        name: "Monthly Ranking August 2014",
        participant_count: None,
        start_date: "2014-09-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-09-15T00:00:00+00:00",
        id: 92,
        mode_specific: true,
        name: "Monthly Ranking July 2014",
        participant_count: None,
        start_date: "2014-08-17T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-08-15T00:00:00+00:00",
        id: 91,
        mode_specific: true,
        name: "Monthly Ranking June 2014",
        participant_count: None,
        start_date: "2014-07-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-07-15T00:00:00+00:00",
        id: 90,
        mode_specific: true,
        name: "Monthly Ranking May 2014",
        participant_count: None,
        start_date: "2014-06-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-06-15T00:00:00+00:00",
        id: 89,
        mode_specific: true,
        name: "Monthly Ranking April 2014",
        participant_count: None,
        start_date: "2014-05-16T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-05-15T00:00:00+00:00",
        id: 88,
        mode_specific: true,
        name: "Monthly Ranking March 2014",
        participant_count: None,
        start_date: "2014-04-18T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-04-20T00:00:00+00:00",
        id: 87,
        mode_specific: true,
        name: "Monthly Ranking February 2014",
        participant_count: None,
        start_date: "2014-03-23T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-04-20T00:00:00+00:00",
        id: 86,
        mode_specific: true,
        name: "Monthly Ranking January 2014",
        participant_count: None,
        start_date: "2014-03-23T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-03-10T00:00:00+00:00",
        id: 85,
        mode_specific: true,
        name: "Best of 2013",
        participant_count: None,
        start_date: "2014-02-10T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2014-02-20T00:00:00+00:00",
        id: 84,
        mode_specific: true,
        name: "Monthly Ranking December 2013",
        participant_count: None,
        start_date: "2014-01-27T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2014-01-15T00:00:00+00:00",
        id: 83,
        mode_specific: true,
        name: "Monthly Ranking November 2013",
        participant_count: None,
        start_date: "2013-12-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-12-15T00:00:00+00:00",
        id: 82,
        mode_specific: true,
        name: "Monthly Ranking October 2013",
        participant_count: None,
        start_date: "2013-11-22T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-11-15T00:00:00+00:00",
        id: 81,
        mode_specific: true,
        name: "Monthly Ranking September 2013 #2",
        participant_count: None,
        start_date: "2013-10-21T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-10-15T00:00:00+00:00",
        id: 80,
        mode_specific: true,
        name: "Monthly Ranking September 2013",
        participant_count: None,
        start_date: "2013-09-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-09-15T00:00:00+00:00",
        id: 79,
        mode_specific: true,
        name: "Monthly Ranking August 2013",
        participant_count: None,
        start_date: "2013-08-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-08-31T00:00:00+00:00",
        id: 78,
        mode_specific: true,
        name: "August 2013 Themed Chart: Marathon Maps",
        participant_count: None,
        start_date: "2013-08-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2013-08-15T00:00:00+00:00",
        id: 77,
        mode_specific: true,
        name: "Monthly Ranking July 2013",
        participant_count: None,
        start_date: "2013-07-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-07-15T00:00:00+00:00",
        id: 76,
        mode_specific: true,
        name: "Monthly Ranking June 2013",
        participant_count: None,
        start_date: "2013-06-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-06-15T00:00:00+00:00",
        id: 75,
        mode_specific: true,
        name: "Monthly Ranking May 2013",
        participant_count: None,
        start_date: "2013-05-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-05-15T00:00:00+00:00",
        id: 74,
        mode_specific: true,
        name: "Monthly Ranking April 2013",
        participant_count: None,
        start_date: "2013-04-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-04-15T00:00:00+00:00",
        id: 73,
        mode_specific: true,
        name: "Monthly Ranking March 2013",
        participant_count: None,
        start_date: "2013-03-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-03-31T00:00:00+00:00",
        id: 72,
        mode_specific: true,
        name: "Themed Chart: Video Games",
        participant_count: None,
        start_date: "2013-03-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2013-03-15T00:00:00+00:00",
        id: 71,
        mode_specific: true,
        name: "Monthly Ranking February 2013",
        participant_count: None,
        start_date: "2013-02-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-02-28T00:00:00+00:00",
        id: 70,
        mode_specific: true,
        name: "Themed Chart: Love",
        participant_count: None,
        start_date: "2013-02-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2013-02-15T00:00:00+00:00",
        id: 69,
        mode_specific: true,
        name: "Monthly Ranking January 2013",
        participant_count: None,
        start_date: "2013-01-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2013-01-31T00:00:00+00:00",
        id: 68,
        mode_specific: true,
        name: "Best of 2012",
        participant_count: None,
        start_date: "2013-02-01T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2013-01-15T00:00:00+00:00",
        id: 67,
        mode_specific: true,
        name: "Monthly Ranking December 2012",
        participant_count: None,
        start_date: "2012-12-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-12-31T00:00:00+00:00",
        id: 66,
        mode_specific: true,
        name: "Themed Chart: Winter/Christmas",
        participant_count: None,
        start_date: "2012-12-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-12-14T00:00:00+00:00",
        id: 65,
        mode_specific: true,
        name: "Monthly Ranking November 2012",
        participant_count: None,
        start_date: "2012-11-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-11-30T00:00:00+00:00",
        id: 64,
        mode_specific: true,
        name: "Themed Chart: Collabs",
        participant_count: None,
        start_date: "2012-11-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-11-14T00:00:00+00:00",
        id: 63,
        mode_specific: true,
        name: "Monthly Ranking October 2012",
        participant_count: None,
        start_date: "2012-10-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-10-31T00:00:00+00:00",
        id: 62,
        mode_specific: true,
        name: "Themed Chart: Relaxing",
        participant_count: None,
        start_date: "2012-10-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-10-14T00:00:00+00:00",
        id: 61,
        mode_specific: true,
        name: "Monthly Ranking September 2012",
        participant_count: None,
        start_date: "2012-09-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-09-30T00:00:00+00:00",
        id: 60,
        mode_specific: true,
        name: "Themed Chart: Dance Dance Revolution",
        participant_count: None,
        start_date: "2012-09-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-09-14T00:00:00+00:00",
        id: 59,
        mode_specific: true,
        name: "Monthly Ranking August 2012",
        participant_count: None,
        start_date: "2012-08-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-08-31T00:00:00+00:00",
        id: 58,
        mode_specific: true,
        name: "Themed Chart: English Pop/Rock",
        participant_count: None,
        start_date: "2012-08-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-08-14T00:00:00+00:00",
        id: 57,
        mode_specific: true,
        name: "Monthly Ranking July 2012",
        participant_count: None,
        start_date: "2012-07-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-07-31T00:00:00+00:00",
        id: 56,
        mode_specific: true,
        name: "Themed Chart: DJMAX",
        participant_count: None,
        start_date: "2012-07-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-07-14T00:00:00+00:00",
        id: 55,
        mode_specific: true,
        name: "Monthly Ranking June 2012",
        participant_count: None,
        start_date: "2012-06-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-06-30T00:00:00+00:00",
        id: 54,
        mode_specific: true,
        name: "Themed Chart: Vocaloid",
        participant_count: None,
        start_date: "2012-06-01T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-06-14T00:00:00+00:00",
        id: 53,
        mode_specific: true,
        name: "Monthly Ranking May 2012",
        participant_count: None,
        start_date: "2012-05-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-05-14T00:00:00+00:00",
        id: 52,
        mode_specific: true,
        name: "Monthly Ranking April 2012",
        participant_count: None,
        start_date: "2012-04-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-04-30T00:00:00+00:00",
        id: 51,
        mode_specific: true,
        name: "Themed Chart: Approved Maps",
        participant_count: None,
        start_date: "2012-04-02T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2012-04-02T00:00:00+00:00",
        id: 50,
        mode_specific: false,
        name: "Bad Apple Ranking chart!",
        participant_count: None,
        start_date: "2012-04-01T00:00:00+00:00",
        spotlight_type: "special"
    }, Spotlight {
        end_date: "2012-04-14T00:00:00+00:00",
        id: 49,
        mode_specific: true,
        name: "Monthly Ranking March 2012",
        participant_count: None,
        start_date: "2012-03-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-03-31T00:00:00+00:00",
        id: 48,
        mode_specific: false,
        name: "Best of James, Al-Azif, and DaRRi MIx",
        participant_count: None,
        start_date: "2012-03-01T00:00:00+00:00",
        spotlight_type: "special"
    }, Spotlight {
        end_date: "2012-03-14T00:00:00+00:00",
        id: 47,
        mode_specific: true,
        name: "Monthly Ranking February 2012",
        participant_count: None,
        start_date: "2012-02-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-02-15T00:00:00+00:00",
        id: 46,
        mode_specific: true,
        name: "Monthly Ranking January 2012",
        participant_count: None,
        start_date: "2012-01-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2012-01-31T00:00:00+00:00",
        id: 45,
        mode_specific: false,
        name: "Best of 2011",
        participant_count: None,
        start_date: "2012-01-01T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2012-01-14T00:00:00+00:00",
        id: 44,
        mode_specific: true,
        name: "Monthly Ranking December 2011",
        participant_count: None,
        start_date: "2011-12-15T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-11-30T00:00:00+00:00",
        id: 43,
        mode_specific: true,
        name: "Monthly Ranking November 2011",
        participant_count: None,
        start_date: "2011-11-11T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-11-20T00:00:00+00:00",
        id: 42,
        mode_specific: true,
        name: "Beatmania IIDX Chart",
        participant_count: None,
        start_date: "2011-10-27T00:00:00+00:00",
        spotlight_type: "special"
    }, Spotlight {
        end_date: "2011-10-31T00:00:00+00:00",
        id: 41,
        mode_specific: true,
        name: "Monthly Ranking October 2011",
        participant_count: None,
        start_date: "2011-10-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-09-30T00:00:00+00:00",
        id: 29,
        mode_specific: false,
        name: "Monthly Ranking September 2011",
        participant_count: None,
        start_date: "2011-09-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-09-10T00:00:00+00:00",
        id: 28,
        mode_specific: false,
        name: "Touhou Chart",
        participant_count: None,
        start_date: "2011-08-13T00:00:00+00:00",
        spotlight_type: "theme"
    }, Spotlight {
        end_date: "2011-08-31T00:00:00+00:00",
        id: 27,
        mode_specific: false,
        name: "Monthly Ranking August 2011",
        participant_count: None,
        start_date: "2011-08-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-08-10T00:00:00+00:00",
        id: 26,
        mode_specific: false,
        name: "3 Snap",
        participant_count: None,
        start_date: "2011-07-10T00:00:00+00:00",
        spotlight_type: "special"
    }, Spotlight {
        end_date: "2011-06-30T00:00:00+00:00",
        id: 25,
        mode_specific: false,
        name: "Monthly Ranking June 2011",
        participant_count: None,
        start_date: "2011-06-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-05-31T00:00:00+00:00",
        id: 24,
        mode_specific: false,
        name: "Monthly Ranking May 2011",
        participant_count: None,
        start_date: "2011-05-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-04-30T00:00:00+00:00",
        id: 23,
        mode_specific: false,
        name: "Monthly Ranking April 2011",
        participant_count: None,
        start_date: "2011-04-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-03-31T00:00:00+00:00",
        id: 22,
        mode_specific: false,
        name: "Monthly Ranking March 2011",
        participant_count: None,
        start_date: "2011-03-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-02-28T00:00:00+00:00",
        id: 21,
        mode_specific: false,
        name: "Monthly Ranking February 2011",
        participant_count: None,
        start_date: "2011-02-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2011-01-31T00:00:00+00:00",
        id: 20,
        mode_specific: false,
        name: "Best of 2010",
        participant_count: None,
        start_date: "2011-01-01T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2011-01-10T00:00:00+00:00",
        id: 19,
        mode_specific: false,
        name: "Mod Restricted Chart 1",
        participant_count: None,
        start_date: "2010-12-05T00:00:00+00:00",
        spotlight_type: "special"
    }, Spotlight {
        end_date: "2010-12-31T00:00:00+00:00",
        id: 18,
        mode_specific: false,
        name: "Monthly Ranking December 2010",
        participant_count: None,
        start_date: "2010-12-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-11-30T00:00:00+00:00",
        id: 17,
        mode_specific: false,
        name: "Monthly Ranking November 2010",
        participant_count: None,
        start_date: "2010-11-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-10-31T00:00:00+00:00",
        id: 16,
        mode_specific: false,
        name: "Monthly Ranking October 2010",
        participant_count: None,
        start_date: "2010-10-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-09-30T00:00:00+00:00",
        id: 15,
        mode_specific: false,
        name: "Monthly Ranking September 2010",
        participant_count: None,
        start_date: "2010-09-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-08-31T00:00:00+00:00",
        id: 14,
        mode_specific: false,
        name: "Monthly Ranking August 2010",
        participant_count: None,
        start_date: "2010-08-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-06-30T00:00:00+00:00",
        id: 13,
        mode_specific: false,
        name: "Monthly Ranking June 2010",
        participant_count: None,
        start_date: "2010-06-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-05-31T00:00:00+00:00",
        id: 12,
        mode_specific: false,
        name: "Monthly Ranking May 2010",
        participant_count: None,
        start_date: "2010-05-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-04-30T00:00:00+00:00",
        id: 11,
        mode_specific: false,
        name: "Monthly Ranking April 2010",
        participant_count: None,
        start_date: "2010-04-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-03-31T00:00:00+00:00",
        id: 10,
        mode_specific: false,
        name: "Monthly Ranking March 2010",
        participant_count: None,
        start_date: "2010-03-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-02-28T00:00:00+00:00",
        id: 9,
        mode_specific: false,
        name: "Monthly Ranking February 2010",
        participant_count: None,
        start_date: "2010-02-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2010-02-01T00:00:00+00:00",
        id: 8,
        mode_specific: false,
        name: "Best Beatmaps of 2009",
        participant_count: None,
        start_date: "2010-01-01T00:00:00+00:00",
        spotlight_type: "bestof"
    }, Spotlight {
        end_date: "2009-12-31T00:00:00+00:00",
        id: 7,
        mode_specific: false,
        name: "Monthly Ranking December 2009",
        participant_count: None,
        start_date: "2009-12-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2009-11-30T00:00:00+00:00",
        id: 6,
        mode_specific: false,
        name: "Monthly Ranking November 2009",
        participant_count: None,
        start_date: "2009-11-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }, Spotlight {
        end_date: "2009-10-31T00:00:00+00:00",
        id: 5,
        mode_specific: false,
        name: "Monthly Ranking October 2009",
        participant_count: None,
        start_date: "2009-10-01T00:00:00+00:00",
        spotlight_type: "monthly"
    }]
}
*/
