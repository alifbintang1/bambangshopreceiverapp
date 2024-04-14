use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::Notification;

// singleton of db
lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;
impl NotificationRepository{
}