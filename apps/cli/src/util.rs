use chrono::{Duration, NaiveDateTime};
use celerity::PriorityLevel;

/// Contains information to be used in filtering the task list.
pub struct ListInfo {
    pub completed: bool,
    pub before: Option<NaiveDateTime>,
    pub after: Option<NaiveDateTime>,
    pub shorter: Option<Duration>,
    pub longer: Option<Duration>,
    pub lower: Option<PriorityLevel>,
    pub higher: Option<PriorityLevel>,
}
