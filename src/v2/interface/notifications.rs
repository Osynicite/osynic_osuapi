// Placeholder
use crate::error::Result;

use crate::v2::model::notification::dtos::request::MarkNotificationsRequest;
use crate::v2::model::notification::dtos::response::GetNotificationsResponse;

pub trait INotifications {
    fn get_notifications(
        &self,
        max_id: Option<String>,
    ) -> impl std::future::Future<Output = Result<GetNotificationsResponse>>;
    fn mark_notifications_as_read(
        &self,
        params: Option<MarkNotificationsRequest>,
    ) -> impl std::future::Future<Output = Result<()>>;
}

// identities   object  optional
// category   string  optional

// Notification category.
// object_id   string  optional

// Id of the object triggered the notification.
// object_type   string  optional

// Type of the object triggered the notification.
// notifications   object  optional
// category   string  optional

// Notification category.
// id   integer  optional

// Id of notifications to be marked as read.
// object_id   string  optional

// Id of the object triggered the notification.
// object_type   string  optional

// Type of the object triggered the notification.
