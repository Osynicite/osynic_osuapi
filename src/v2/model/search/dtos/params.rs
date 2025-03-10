use crate::v2::model::search::enums::categories::Categories;
use crate::v2::model::search::enums::extra::Extra;
use crate::v2::model::search::enums::general::General;
use crate::v2::model::search::enums::genre::Genre;
use crate::v2::model::search::enums::language::Language;
use crate::v2::model::search::enums::mode::Mode;
use crate::v2::model::search::enums::rank_achieved::RankAchieved;
use crate::v2::model::search::enums::sort::Sort;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapsetsSearchParams {
    pub query: Option<String>,
    pub general: Option<Vec<General>>,
    pub mode: Option<Mode>,
    pub categories: Option<Categories>,
    pub explicit_content: Option<bool>,
    pub genre: Option<Genre>,
    pub language: Option<Language>,
    pub extra: Option<Vec<Extra>>,
    pub rank_achieved: Option<Vec<RankAchieved>>,
    pub played: Option<bool>,
    pub sort: Option<Sort>,
}

impl BeatmapsetsSearchParams {
    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn general(mut self, general: Vec<General>) -> Self {
        self.general = Some(general);
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn categories(mut self, categories: Categories) -> Self {
        self.categories = Some(categories);
        self
    }

    pub fn explicit_content(mut self, explicit_content: bool) -> Self {
        self.explicit_content = Some(explicit_content);
        self
    }

    pub fn genre(mut self, genre: Genre) -> Self {
        self.genre = Some(genre);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn extra(mut self, extra: Vec<Extra>) -> Self {
        self.extra = Some(extra);
        self
    }

    pub fn rank_achieved(mut self, rank_achieved: Vec<RankAchieved>) -> Self {
        self.rank_achieved = Some(rank_achieved);
        self
    }

    pub fn played(mut self, played: bool) -> Self {
        self.played = Some(played);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];
        if let Some(query) = &self.query {
            params.push(("q".to_string(), query.clone()));
        }
        if let Some(general) = &self.general {
            let mut general_str = "".to_string();
            for i in 0..general.len() {
                general_str.push_str(&general[i].to_beatmapset_search());
                if i != general.len() - 1 {
                    general_str.push_str(".");
                }
            }
            params.push(("c".to_string(), general_str));
        }
        if let Some(mode) = &self.mode {
            params.push(("m".to_string(), mode.to_beatmapset_search()));
        }
        if let Some(categories) = &self.categories {
            params.push(("s".to_string(), categories.to_beatmapset_search()));
        }
        if let Some(explicit_content) = &self.explicit_content {
            params.push(("nsfw".to_string(), explicit_content.to_string()));
        }
        if let Some(genre) = &self.genre {
            params.push(("g".to_string(), genre.to_beatmapset_search()));
        }
        if let Some(language) = &self.language {
            params.push(("l".to_string(), language.to_beatmapset_search()));
        }
        if let Some(extra) = &self.extra {
            let mut extra_str = "".to_string();
            for i in 0..extra.len() {
                extra_str.push_str(&extra[i].to_beatmapset_search());
                if i != extra.len() - 1 {
                    extra_str.push_str(".");
                }
            }
            params.push(("e".to_string(), extra_str));
        }
        if let Some(rank_achieved) = &self.rank_achieved {
            let mut rank_achieved_str = "".to_string();
            for i in 0..rank_achieved.len() {
                rank_achieved_str.push_str(&rank_achieved[i].to_beatmapset_search());
                if i != rank_achieved.len() - 1 {
                    rank_achieved_str.push_str(".");
                }
            }
            params.push(("r".to_string(), rank_achieved_str));
        }
        if let Some(played) = &self.played {
            if *played {
                params.push(("played".to_string(), "played".to_string()));
            } else {
                params.push(("played".to_string(), "unplayed".to_string()));
            }
        }
        if let Some(sort) = &self.sort {
            params.push(("sort".to_string(), sort.to_beatmapset_search()));
        }
        params
    }

    pub fn to_raw(&self) -> BeatmapsetsSearchParamsRaw {
        BeatmapsetsSearchParamsRaw {
            q: self.query.clone(),
            c: self.general.clone(),
            m: self.mode.clone(),
            s: self.categories.clone(),
            nsfw: self.explicit_content.clone(),
            g: self.genre.clone(),
            l: self.language.clone(),
            e: self.extra.clone(),
            r: self.rank_achieved.clone(),
            played: self.played.clone(),
            sort: self.sort.clone(),
        }
    }
}

// 留作纪念
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapsetsSearchParamsRaw {
    pub q: Option<String>,
    pub c: Option<Vec<General>>,
    pub m: Option<Mode>,
    pub s: Option<Categories>,
    pub nsfw: Option<bool>,
    pub g: Option<Genre>,
    pub l: Option<Language>,
    pub e: Option<Vec<Extra>>,
    pub r: Option<Vec<RankAchieved>>,
    pub played: Option<bool>,
    pub sort: Option<Sort>,
}

impl BeatmapsetsSearchParamsRaw {
    pub fn q(mut self, q: String) -> Self {
        self.q = Some(q);
        self
    }

    pub fn c(mut self, c: Vec<General>) -> Self {
        self.c = Some(c);
        self
    }

    pub fn m(mut self, m: Mode) -> Self {
        self.m = Some(m);
        self
    }

    pub fn s(mut self, s: Categories) -> Self {
        self.s = Some(s);
        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    pub fn g(mut self, genre: Genre) -> Self {
        self.g = Some(genre);
        self
    }

    pub fn l(mut self, l: Language) -> Self {
        self.l = Some(l);
        self
    }

    pub fn e(mut self, e: Vec<Extra>) -> Self {
        self.e = Some(e);
        self
    }

    pub fn r(mut self, r: Vec<RankAchieved>) -> Self {
        self.r = Some(r);
        self
    }

    pub fn played(mut self, played: bool) -> Self {
        self.played = Some(played);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];
        if let Some(query) = &self.q {
            params.push(("q".to_string(), query.clone()));
        }
        if let Some(general) = &self.c {
            let mut general_str = "".to_string();
            for i in 0..general.len() {
                general_str.push_str(&general[i].to_beatmapset_search());
                if i != general.len() - 1 {
                    general_str.push_str(".");
                }
            }
            params.push(("c".to_string(), general_str));
        }
        if let Some(mode) = &self.m {
            params.push(("m".to_string(), mode.to_beatmapset_search()));
        }
        if let Some(categories) = &self.s {
            params.push(("s".to_string(), categories.to_beatmapset_search()));
        }
        if let Some(explicit_content) = &self.nsfw {
            params.push(("nsfw".to_string(), explicit_content.to_string()));
        }
        if let Some(genre) = &self.g {
            params.push(("g".to_string(), genre.to_beatmapset_search()));
        }
        if let Some(language) = &self.l {
            params.push(("l".to_string(), language.to_beatmapset_search()));
        }
        if let Some(extra) = &self.e {
            let mut extra_str = "".to_string();
            for i in 0..extra.len() {
                extra_str.push_str(&extra[i].to_beatmapset_search());
                if i != extra.len() - 1 {
                    extra_str.push_str(".");
                }
            }
            params.push(("e".to_string(), extra_str));
        }
        if let Some(rank_achieved) = &self.r {
            let mut rank_achieved_str = "".to_string();
            for i in 0..rank_achieved.len() {
                rank_achieved_str.push_str(&rank_achieved[i].to_beatmapset_search());
                if i != rank_achieved.len() - 1 {
                    rank_achieved_str.push_str(".");
                }
            }
            params.push(("r".to_string(), rank_achieved_str));
        }
        if let Some(played) = &self.played {
            if *played {
                params.push(("played".to_string(), "played".to_string()));
            } else {
                params.push(("played".to_string(), "unplayed".to_string()));
            }
        }
        if let Some(sort) = &self.sort {
            params.push(("sort".to_string(), sort.to_beatmapset_search()));
        }
        params
    }
}
