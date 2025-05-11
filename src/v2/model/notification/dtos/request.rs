// 只不过是按照文档来写了...，比较难绷这些文档
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkNotificationsRequest {
    pub identities: Option<Identities>,
    pub notifications: Option<Notifications>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identities {
    pub category: Option<String>,
    pub object_id: Option<String>,
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notifications {
    pub category: Option<String>,
    pub id: Option<u32>,
    pub object_id: Option<String>,
    pub object_type: Option<String>,
}

impl MarkNotificationsRequest {
    pub fn new(
        identities: Option<Identities>,
        notifications: Option<Notifications>,
    ) -> MarkNotificationsRequest {
        MarkNotificationsRequest {
            identities,
            notifications,
        }
    }

    pub fn identities(
        category: Option<String>,
        object_id: Option<String>,
        object_type: Option<String>,
    ) -> Identities {
        Identities {
            category,
            object_id,
            object_type,
        }
    }

    pub fn notifications(
        category: Option<String>,
        id: Option<u32>,
        object_id: Option<String>,
        object_type: Option<String>,
    ) -> Notifications {
        Notifications {
            category,
            id,
            object_id,
            object_type,
        }
    }
}

impl Identities {
    pub fn new(
        category: Option<String>,
        object_id: Option<String>,
        object_type: Option<String>,
    ) -> Identities {
        Identities {
            category,
            object_id,
            object_type,
        }
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn object_id(mut self, object_id: String) -> Self {
        self.object_id = Some(object_id);
        self
    }

    pub fn object_type(mut self, object_type: String) -> Self {
        self.object_type = Some(object_type);
        self
    }
}

impl Notifications {
    pub fn new(
        category: Option<String>,
        id: Option<u32>,
        object_id: Option<String>,
        object_type: Option<String>,
    ) -> Notifications {
        Notifications {
            category,
            id,
            object_id,
            object_type,
        }
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn object_id(mut self, object_id: String) -> Self {
        self.object_id = Some(object_id);
        self
    }

    pub fn object_type(mut self, object_type: String) -> Self {
        self.object_type = Some(object_type);
        self
    }
}