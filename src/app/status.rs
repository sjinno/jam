use chrono::{Duration, Local};

enum Status {
    Applied(DateTime),
    Interview(DateTime),
    Accepted(Option<DateTime>),
    Rejected(Option<DateTime>),
}
