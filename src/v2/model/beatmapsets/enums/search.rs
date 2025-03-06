use super::general::General;
use super::mode::Mode;
use super::categories::Categories;
use super::genre::Genre;
use super::language::Language;
use super::extra::Extra;
use super::rank_achieved::RankAchieved;

use serde::{Serialize, Deserialize};


#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct BeatmapsetsSearchParams{
    pub query: Option<String>,
    pub general: Option<Vec<General>>,
    pub mode: Option<Mode>,
    pub categories: Option<Categories>,
    pub explicit_content: Option<bool>,
    pub genre: Option<Genre>,
    pub language: Option<Language>,
    pub extra: Option<Vec<Extra>>,
    pub rank_achieved: Option<Vec<RankAchieved>>,
    pub played: Option<bool>
}

impl BeatmapsetsSearchParams{
    pub fn query(mut self, query: String) -> Self{
        self.query = Some(query);
        self
    }

    pub fn general(mut self, general: Vec<General>) -> Self{
        self.general = Some(general);
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self{
        self.mode = Some(mode);
        self
    }

    pub fn categories(mut self, categories: Categories) -> Self{
        self.categories = Some(categories);
        self
    }

    pub fn explicit_content(mut self, explicit_content: bool) -> Self{
        self.explicit_content = Some(explicit_content);
        self
    }

    pub fn genre(mut self, genre: Genre) -> Self{
        self.genre = Some(genre);
        self
    }

    pub fn language(mut self, language: Language) -> Self{
        self.language = Some(language);
        self
    }

    pub fn extra(mut self, extra: Vec<Extra>) -> Self{
        self.extra = Some(extra);
        self
    }

    pub fn rank_achieved(mut self, rank_achieved: Vec<RankAchieved>) -> Self{
        self.rank_achieved = Some(rank_achieved);
        self
    }

    pub fn played(mut self, played: bool) -> Self{
        self.played = Some(played);
        self
    }



    pub fn to_raw(&self) -> BeatmapsetsSearchParamsRaw{
        BeatmapsetsSearchParamsRaw{
            q: self.query.clone(),
            c: self.general.clone(),
            m: self.mode.clone(),
            s: self.categories.clone(),
            nsfw: self.explicit_content.clone(),
            g: self.genre.clone(),
            l: self.language.clone(),
            e: self.extra.clone(),
            r: self.rank_achieved.clone(),
            played: self.played.clone()
        }
    }
}



// 留作纪念
#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct BeatmapsetsSearchParamsRaw{
    pub q: Option<String>,
    pub c: Option<Vec<General>>,
    pub m: Option<Mode>,
    pub s: Option<Categories>,
    pub nsfw: Option<bool>,
    pub g: Option<Genre>,
    pub l: Option<Language>,
    pub e: Option<Vec<Extra>>,
    pub r: Option<Vec<RankAchieved>>,
    pub played: Option<bool>
}


impl BeatmapsetsSearchParamsRaw{
    pub fn q(mut self,q: String) -> Self{
        self.q = Some(q);
        self
    }

    pub fn c(mut self,c: Vec<General>) -> Self{
        self.c = Some(c);
        self
    }

    pub fn m(mut self,m: Mode) -> Self{
        self.m = Some(m);
        self
    }

    pub fn s(mut self,s: Categories) -> Self{
        self.s = Some(s);
        self
    }

    pub fn nsfw(mut self,nsfw: bool) -> Self{
        self.nsfw = Some(nsfw);
        self
    }

    pub fn g(mut self,genre: Genre) -> Self{
        self.g = Some(genre);
        self
    }

    pub fn l(mut self,l: Language) -> Self{
        self.l = Some(l);
        self
    }

    pub fn e(mut self,e: Vec<Extra>) -> Self{
        self.e = Some(e);
        self
    }

    pub fn r(mut self,r: Vec<RankAchieved>) -> Self{
        self.r = Some(r);
        self
    }

    pub fn played(mut self,played: bool) -> Self{
        self.played = Some(played);
        self
    }


}