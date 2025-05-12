// \"body\": \"hello\",
// \"forum_id\": 1,
// \"title\": \"untitled\",
// \"with_poll\": true,
// \"forum_topic_poll[options]\": \"item A...\",
// \"forum_topic_poll[title]\": \"my poll\"
    
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTopicParams {
    pub title: String,
    pub body: String,
    pub forum_id: u64,
    pub with_poll: Option<bool>,
    pub forum_topic_poll: Option<ForumTopicPoll>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicPoll {
    pub options: String,
    pub title: String,
    pub hide_results: Option<bool>,
    pub length_days: Option<u32>,
    pub max_options: Option<u32>,
    pub vote_change: Option<bool>,
}

impl CreateTopicParams {
    pub fn new(
        title: String,
        body: String,
        forum_id: u64,
        with_poll: Option<bool>,
        poll_options: String,
        poll_title: String,
    ) -> Self {
        Self {
            title,
            body,
            forum_id,
            with_poll,
            forum_topic_poll: Some(ForumTopicPoll {
                options: poll_options,
                title: poll_title,
                hide_results: None,
                length_days: None,
                max_options: None,
                vote_change: None,
            }),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn forum_id(mut self, forum_id: u64) -> Self {
        self.forum_id = forum_id;
        self
    }

    pub fn with_poll(mut self, with_poll: Option<bool>) -> Self {
        self.with_poll = with_poll;
        self
    }

    pub fn poll_options(mut self, poll_options: String) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.options = poll_options;
        }
        self
    }

    pub fn poll_title(mut self, poll_title: String) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.title = poll_title;
        }
        self
    }

    pub fn poll_hide_results(mut self, hide_results: bool) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.hide_results = Some(hide_results);
        }
        self
    }

    pub fn poll_length_days(mut self, length_days: u32) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.length_days = Some(length_days);
        }
        self
    }

    pub fn poll_max_options(mut self, max_options: u32) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.max_options = Some(max_options);
        }
        self
    }

    pub fn poll_vote_change(mut self, vote_change: bool) -> Self {
        if let Some(ref mut poll) = self.forum_topic_poll {
            poll.vote_change = Some(vote_change);
        }
        self
    }

}