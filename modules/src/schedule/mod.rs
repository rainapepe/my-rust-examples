mod schedule_type;

use schedule_type::ScheduleType;

pub struct Schedule {
    pub name: String,
    pub cron: String,
    pub enable: bool,
    pub schedule_type: ScheduleType,
}

pub mod schedules {
    use super::{Schedule, ScheduleType};

    pub fn create(name: String) -> Schedule {
        Schedule {
            name,
            cron: "* * * * * *".to_string(),
            enable: false,
            schedule_type: ScheduleType::Low,
        }
    }
}
