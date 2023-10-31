use chrono::{DateTime, Utc};
use uuid::Uuid;

#[warn(dead_code)]
pub struct Exception {
    id:Uuid,
    signature: String,
    application:String,
    created_at:DateTime<Utc>,
    updated_at:DateTime<Utc>
}