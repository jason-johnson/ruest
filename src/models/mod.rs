pub mod page;
pub mod org;

use chrono::{DateTime, Utc};
use org::Role;

struct EventTemplate {
    title: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    published: bool,
    content: String,
    roles: Vec<Role>,
}